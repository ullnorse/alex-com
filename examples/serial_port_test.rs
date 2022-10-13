use serialport::available_ports;

fn main() {
    let ports = serial2::SerialPort::available_ports().unwrap();

    for port in ports {
        println!("{:?}", port);
    }

    println!("--------------------------------------------------");

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

    for port in ports {
        println!("{:?}", port);
    }
}