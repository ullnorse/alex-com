use std::io;

use serialport::available_ports;
use std::time::Duration;

fn main() {
    // let ports = serial2::SerialPort::available_ports().unwrap();

    // for port in ports {
    //     println!("{:?}", port);
    // }

    // println!("--------------------------------------------------");

    // let ports: Vec<serialport::SerialPortInfo> = serialport::available_ports().unwrap();
    let ports: Vec<String> =
        available_ports()
        .unwrap()
        .iter()
        .filter_map(|port| if port.port_name.contains("USB") {
            Some(port.port_name.clone())
        } else {
            None
        })
        .collect();

    for port in &ports {
        println!("{:?}", port);
    }

    

    let mut port = serialport::new(ports[0].clone(), 9600)
        .open()
        .unwrap();

    port.clear(serialport::ClearBuffer::All).unwrap();

    let mut buffer = [0u8; 1];

    let mut q = Vec::new();

    loop {
        match port.read_exact(&mut buffer) {
            Ok(_) => {
                let c = buffer[0] as char;
                match c {
                    '\n' => {
                        q.push(c);
                        let s: String = q.iter().collect();
                        print!("{s}");
                    },
                    _ => q.push(c),
                }
            },
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("{:?}", e)
        }

    }

    // let port = serialport::SerialPortBuilder::
}