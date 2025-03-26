pub mod animal;
pub mod error;
pub mod notes;
pub mod prelude;
pub mod utils;
use std::sync::Arc;

use error::Error;

use animal::{Animal, AnimalSounds};
use notes::Note;
use prelude::*;
use rodio::{OutputStream, Sink};
use serialport::SerialPortInfo;
use tauri::{Manager, State};

struct AppState {
    pub animal: Animal,
    pub port: Arc<Option<SerialPortInfo>>,
    pub sounds: AnimalSounds,
    pub port_sender: mpsc::Sender<Arc<std::option::Option<SerialPortInfo>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(port: Option<SerialPortInfo>) -> Result<()> {
    let (port_sender, port_receiver) = mpsc::channel(1);
    let (note_sender, mut note_receiver) = mpsc::channel(10);

    tauri::Builder::default()
        .setup(move |app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Teclado Interactivo")?;

            let state = Mutex::new(AppState {
                animal: Animal::Elephant,
                port: Arc::new(port),
                sounds: AnimalSounds::new(app.path())?,
                port_sender,
            });

            tauri::async_runtime::spawn(async move {
                let _ = watch_serial(port_receiver, note_sender).await;
            });

            tauri::async_runtime::spawn(async move {
                while let Some(note) = note_receiver.recv().await {
                    println!("Note received: {:?}", note);
                }
            });

            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            play_note,
            select_animal,
            reconnect_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
async fn play_note(note: String, state: State<'_, Mutex<AppState>>) -> Result<()> {
    let state = state.lock().map_err(|e| Error::Generic(e.to_string()))?;

    println!("port is {:?}", state.port);

    let animal = state.animal;
    match Note::new(&note) {
        Some(note) => {
            let sound = animal.sound(&state.sounds, &note)?;

            #[cfg(test)]
            println!("Playing note {} from animal {}", note, animal);

            tauri::async_runtime::spawn(async move { play_sound(sound).await });
            Ok(())
        }
        None => Err(Error::Generic(format!("Invalid note: {}", note))),
    }
}

async fn play_sound(sound: impl rodio::Source<Item = i16> + Send + 'static) -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    sink.append(sound);
    sink.sleep_until_end();
    Ok(())
}

#[tauri::command]
async fn select_animal(animal: String, state: State<'_, Mutex<AppState>>) -> Result<()> {
    let mut state = state.lock().map_err(|e| Error::Generic(e.to_string()))?;

    state.animal = Animal::new(&animal)
        .ok_or_else(|| Error::Generic(format!("Invalid animal: {}", animal)))?;

    Ok(())
}

#[tauri::command]
async fn reconnect_port(app_state: State<'_, Mutex<AppState>>) -> Result<String> {
    // Clone the port sender before locking the mutex
    let sender = {
        let state = app_state
            .lock()
            .map_err(|e| Error::Generic(e.to_string()))?;
        state.port_sender.clone()
    };

    // Get the new port outside of the mutex lock
    let new_port = utils::get_port();

    // If a new port is found, send it through the channel
    if let Some(port) = new_port {
        let port_arc = Arc::new(Some(port));

        if sender.send(Arc::clone(&port_arc)).await.is_err() {
            return Err(Error::Generic("Failed to send port".to_string()));
        }

        // Update the state in a separate lock
        let mut state = app_state
            .lock()
            .map_err(|e| Error::Generic(e.to_string()))?;
        state.port = port_arc;
    }

    // Format the port information for return
    let state = app_state
        .lock()
        .map_err(|e| Error::Generic(e.to_string()))?;
    Ok(format!(
        "{{ port: {} }}",
        match state.port.as_ref() {
            Some(port) => format!("{{ name: {}}}", port.port_name),
            None => "null".to_string(),
        }
    ))
}

async fn watch_serial(
    mut port_receiver: mpsc::Receiver<Arc<std::option::Option<SerialPortInfo>>>,
    note_sender: mpsc::Sender<String>,
) {
    // loop until receive a port
    loop {
        let mut buf: Vec<u8> = vec![0; 32];
        if let Some(port) = port_receiver.recv().await {
            let port = match port.as_ref() {
                Some(port) => port,
                None => continue,
            };
            let mut serial = match serialport::new(&port.port_name, 9600)
                .timeout(std::time::Duration::from_millis(10))
                .open()
            {
                Ok(serial) => serial,
                Err(_) => continue,
            };

            // loop until the serial read or the channel send fail
            loop {
                match serial.read(&mut buf) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        let note = String::from_utf8_lossy(&buf[..bytes_read]).to_string();
                        if let Err(_) = note_sender.send(note).await {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }
    }
}
