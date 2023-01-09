use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};

// use serialport::{DataBits, FlowControl, Parity, StopBits};

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
            baudrate: 9600,
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

        // let builder = serialport::new(config.port, config.baudrate)
        //     .data_bits(config.data_bits)
        //     .flow_control(config.flow_control)
        //     .parity(config.parity)
        //     .stop_bits(config.stop_bits);

        // let mut serial_port = builder.open()?;
        // serial_port.clear(serialport::ClearBuffer::All)?;

        std::thread::spawn(move || {
            let mut poll = Poll::new().unwrap();

            let mut events = Events::with_capacity(1);

            let mut port = mio_serial::new(config.port, config.baudrate)
                .data_bits(config.data_bits)
                .flow_control(config.flow_control)
                .parity(config.parity)
                .stop_bits(config.stop_bits)
                .open_native_async().unwrap();

            port.clear(ClearBuffer::All).unwrap();

            poll.registry()
                .register(&mut port, SERIAL_TOKEN, Interest::READABLE)
                .unwrap();

            let mut buf = [0u8; 1024];

            // let mut buffer = [0u8; 1];
            // let mut q = Vec::new();

            loop {
                if state_receiver.try_recv().is_ok() {
                    break;
                }

                poll.poll(&mut events, Some(std::time::Duration::from_millis(1000))).ok();

                println!("here 2");

                for event in events.iter() {
                    println!("test");
                    if event.token() == SERIAL_TOKEN {
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

    pub fn stop(&self) {
        let (state_sender, _) = self.state_channel.clone();

        state_sender.send(true).unwrap();
    }

    pub fn get_receiver(&self) -> Receiver<String> {
        self.data_channel.1.clone()
    }
}
