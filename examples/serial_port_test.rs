use std::io;

use serialport::available_ports;
use std::time::Duration;

fn main() {
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

    for port in ports {
        println!("{:?}", port);
    }

    let mut serial_port = serialport::new("COM5", 115200)
        .timeout(std::time::Duration::from_millis(100)).open().unwrap();

    let mut buffer = [0u8; 1];
    let mut v = Vec::<u8>::new();
    let mut cnt = 0;

    serial_port.clear(serialport::ClearBuffer::Input).unwrap();

    loop {
        match serial_port.read_exact(&mut buffer) {
            Ok(_) => {
                if buffer[0] == b'\n' {
                    println!("{} {}", cnt, String::from_utf8_lossy(v.as_slice()));
                    v.clear();
                    cnt += 1;
                } else {
                    v.push(buffer[0]);
                }
            },
            Err(e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => println!("{}", e)

        }
    }
}