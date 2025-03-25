use serialport::{available_ports, SerialPortInfo, SerialPortType};

pub fn get_port() -> Option<SerialPortInfo> {
    match available_ports() {
        Ok(ports) => ports
            .iter()
            .find(|p| matches!(p.port_type, SerialPortType::UsbPort(_)))
            .cloned(),
        Err(_) => None,
    }
}
