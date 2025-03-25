pub mod animal;
pub mod error;
pub mod notes;
pub mod prelude;
pub mod utils;
use error::Error;
use std::sync::Mutex;

use animal::{Animal, AnimalSounds};
use notes::Note;
use prelude::*;
use rodio::{OutputStream, Sink};
use serialport::SerialPortInfo;
use tauri::{Manager, State};

struct AppState {
    pub animal: Animal,
    pub port: Option<SerialPortInfo>,
    pub sounds: AnimalSounds,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(port: Option<SerialPortInfo>) -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Teclado Interactivo")?;

            let state = Mutex::new(AppState {
                animal: Animal::Elephant,
                port,
                sounds: AnimalSounds::new(app.path())?,
            });

            // must change https://rfdonnelly.github.io/posts/tauri-async-rust-process/
            // i should use a channel through the main thread and the thread that listen the serial port
            if let Some(port) = state
                .lock()
                .map_err(|e| Error::Generic(e.to_string()))?
                .port
                .clone()
            {
                tauri::async_runtime::spawn(async move { watch_serial(port).await });
            }

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
fn reconnect_port(state: State<'_, Mutex<AppState>>) -> Result<String> {
    let mut state = state.lock().map_err(|e| Error::Generic(e.to_string()))?;

    state.port = utils::get_port();
    if let Some(port) = state.port.clone() {
        tauri::async_runtime::spawn(async move { watch_serial(port).await });
    }

    Ok(format!(
        "{{ port: {} }}",
        match state.port.clone() {
            Some(port) => format!("{{ name: {}}}", port.port_name),
            None => "null".to_string(),
        }
    ))
}

// useless implementation, i could create and appstate here
async fn watch_serial(port: SerialPortInfo) -> Result<()> {
    let mut serial = serialport::new(&port.port_name, 115200)
        .timeout(std::time::Duration::from_millis(10))
        .open()?;

    let mut buf: Vec<u8> = vec![0; 32];
    loop {
        match serial.read(&mut buf) {
            Ok(bytes_read) if bytes_read > 0 => {}
            _ => {}
        }
    }
}
