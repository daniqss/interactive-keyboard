use crate::prelude::*;
use serialport::SerialPortInfo;
use std::io::{BufRead, BufReader};

pub async fn watch_serial(
    port: Arc<Option<SerialPortInfo>>,
    mut port_receiver: Receiver<Arc<Option<SerialPortInfo>>>,
    note_sender: Arc<Sender<String>>,
) {
    loop {
        let port: Option<&SerialPortInfo> = match port.as_ref() {
            Some(port) => Some(port),
            None => None,
        };

        let port_received: Arc<Option<SerialPortInfo>> = match port_receiver.recv().await {
            Some(port_received) => port_received,
            None => {
                println!("No port found");
                continue;
            }
        };

        let port_path = if port.is_none() {
            if let Some(path) = port_received.as_ref() {
                path.port_name.clone()
            } else {
                println!("No port found");
                continue;
            }
        } else {
            if let Some(path) = port.as_ref() {
                path.port_name.clone()
            } else {
                println!("No port found");
                continue;
            }
        };

        let serial = match serialport::new(&port_path, 115200)
            .timeout(std::time::Duration::from_millis(10))
            .open()
        {
            Ok(serial) => serial,
            Err(e) => {
                eprintln!("Failed to open serial port: {}", e);
                continue;
            }
        };

        let mut serial = BufReader::new(serial);
        println!("Serial port opened: {}", port_path);

        loop {
            let mut buf = String::new();
            match serial.read_line(&mut buf) {
                Ok(bytes_read) if bytes_read > 0 => {
                    let note = buf.trim().to_string();

                    #[cfg(test)]
                    println!("Received {} bytes: {:?}", bytes_read, &buf);
                    #[cfg(test)]
                    println!("Received note: {}", note);

                    let note_sender = Arc::clone(&note_sender);
                    tauri::async_runtime::spawn(async move {
                        note_sender
                            .send(note)
                            .await
                            .unwrap_or_else(|e| eprintln!("Failed to send note: {}", e));
                    });
                }
                _ => {}
            }
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}
