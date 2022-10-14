pub fn available_ports() -> Vec<String> {
    serialport::available_ports()
    .unwrap()
    .iter()
    .map(|port| port.port_name.clone())
    .collect()
}