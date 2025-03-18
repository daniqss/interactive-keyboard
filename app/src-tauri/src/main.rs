// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use interactive_keyboard_lib::prelude::*;
use serialport::{available_ports, SerialPortInfo, SerialPortType};
fn main() -> Result<()> {
    let port: Option<SerialPortInfo> = available_ports()?
        .iter()
        .find(|p| matches!(p.port_type, SerialPortType::UsbPort(_)))
        .cloned();

    interactive_keyboard_lib::run(port)
}
