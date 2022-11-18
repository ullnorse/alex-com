use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
use serialport::{DataBits, FlowControl, Parity, StopBits};

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
        serialport::available_ports()
            .unwrap()
            .iter()
            .map(|port| port.port_name.clone())
            .collect()
    }

    pub fn start(&self, config: SerialConfig) -> Result<()> {
        let (_, state_receiver) = self.state_channel.clone();
        let (data_sender, _) = self.data_channel.clone();
        let (_, output_receiver) = self.output_channel.clone();

        let builder = serialport::new(config.port, config.baudrate)
            .data_bits(config.data_bits)
            .flow_control(config.flow_control)
            .parity(config.parity)
            .stop_bits(config.stop_bits);

        let mut serial_port = builder.open()?;
        serial_port.clear(serialport::ClearBuffer::All)?;

        std::thread::spawn(move || {
            let mut buffer = [0u8; 1];
            let mut q = Vec::new();

            loop {
                if state_receiver.try_recv().is_ok() {
                    break;
                }

                if serial_port.bytes_to_read().unwrap() > 0 {
                    match serial_port.read_exact(&mut buffer) {
                        Ok(_) => {
                            let c = buffer[0] as char;
                            match c {
                                '\n' => {
                                    q.push(c);
                                    let s: String = q.iter().collect();
                                    data_sender.send(s).unwrap();
                                    q.clear();
                                }
                                _ => q.push(c),
                            }
                        }
                        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                        Err(e) => eprintln!("{:?}", e),
                    }
                }

                if let Ok(s) = output_receiver.try_recv() {
                    serial_port.write_all(s.as_bytes()).unwrap();
                    serial_port.flush().unwrap();
                }

                // std::thread::sleep(std::time::Duration::from_micros(1));
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
