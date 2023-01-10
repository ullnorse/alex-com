use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
use mio_serial::{SerialPortBuilderExt, available_ports, DataBits, FlowControl, Parity, StopBits, ClearBuffer, SerialPort};
use mio::{Events, Interest, Poll, Token};
use std::io::Read;

const SERIAL_TOKEN: Token = Token(0);

pub struct SerialConfig {
    pub port: String,
    pub baudrate: u32,
    pub data_bits: DataBits,
    pub flow_control: FlowControl,
    pub parity: Parity,
    pub stop_bits: StopBits,
}

impl Default for SerialConfig {
    fn default() -> Self {
        Self {
            port: Default::default(),
            baudrate: 115200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
        }
    }
}

pub struct Serial {
    state_channel: (Sender<bool>, Receiver<bool>),
    data_channel: (Sender<String>, Receiver<String>),
    pub output_channel: (Sender<String>, Receiver<String>),
}

impl Default for Serial {
    fn default() -> Self {
        Self {
            state_channel: unbounded(),
            data_channel: unbounded(),
            output_channel: unbounded(),
        }
    }
}

impl Serial {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn available_ports() -> Vec<String> {
        available_ports()
            .unwrap()
            .iter()
            .map(|port| port.port_name.clone())
            .collect()
    }

    pub fn start(&self, config: SerialConfig) -> Result<()> {
        let (_, state_receiver) = self.state_channel.clone();
        let (data_sender, _) = self.data_channel.clone();
        let (_, output_receiver) = self.output_channel.clone();

        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(5);

        let mut port = mio_serial::new(config.port, config.baudrate)
            .data_bits(config.data_bits)
            .flow_control(config.flow_control)
            .parity(config.parity)
            .stop_bits(config.stop_bits)
            .open_native_async()?;

        port.clear(ClearBuffer::All)?;

        poll.registry()
            .register(&mut port, SERIAL_TOKEN, Interest::READABLE)?;

        let mut buf = vec![0u8; 100];

        std::thread::spawn(move || {
            loop {
                if state_receiver.try_recv().is_ok() {
                    break;
                }

                poll.poll(&mut events, Some(std::time::Duration::from_millis(100))).ok();

                for event in &events {
                    if event.token() == SERIAL_TOKEN && event.is_readable() {
                        loop {
                            match port.read(&mut buf) {
                                Ok(count) => {
                                    let msg = String::from_utf8_lossy(&buf[..count]);

                                    data_sender.send(msg.into_owned()).unwrap();
                                }

                                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    break;
                                }

                                Err(e) => {
                                    println!("Quitting due to read error: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        self.state_channel.0.send(true)?;

        Ok(())
    }

    pub fn get_receiver(&self) -> Receiver<String> {
        self.data_channel.1.clone()
    }
}
