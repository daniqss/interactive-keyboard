pub mod animal;
pub mod error;
pub mod notes;
pub mod prelude;
pub mod utils;

use std::io::{BufRead, BufReader};

use animal::{Animal, AnimalSounds};
use notes::Note;
use prelude::*;
use rodio::{OutputStream, Sink};
use serialport::SerialPortInfo;
use tauri::{Manager, State};

struct AppState {
    pub animal: Arc<Animal>,
    pub port: Arc<Option<SerialPortInfo>>,
    pub sounds: Arc<AnimalSounds>,
    pub port_sender: Arc<Sender<Arc<Option<SerialPortInfo>>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(port: Option<SerialPortInfo>) -> Result<()> {
    let (port_sender, port_receiver) = mpsc::channel(1);
    let (note_sender, mut note_receiver) = mpsc::channel(100);

    tauri::Builder::default()
        .setup(move |app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.set_title("Teclado Interactivo")?;

            let animal = Arc::new(Animal::Elephant);
            let port = Arc::new(port);
            let sounds = Arc::new(AnimalSounds::new(app.path())?);
            let port_sender = Arc::new(port_sender);
            let note_sender = Arc::new(note_sender);

            let state = Mutex::new(AppState {
                animal: Arc::clone(&animal),
                port: Arc::clone(&port),
                sounds: Arc::clone(&sounds),
                port_sender: Arc::clone(&port_sender),
            });

            // thread to watch the serial port
            tauri::async_runtime::spawn(async move {
                let _ = watch_serial(port_receiver, note_sender).await;
            });

            // thread to receive notes from the serial port watcher
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(note) = note_receiver.recv().await {
                        println!("Received note on tauri thread: {}", note);

                        if let Err(e) = play_note(note, Arc::clone(&animal), Arc::clone(&sounds)) {
                            eprintln!("Error playing note from esp32: {}", e);
                        };
                    } else {
                        println!("No note received on tauri thread");
                    }
                }
            });

            app.manage(state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            play_note_command,
            select_animal,
            reconnect_port
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[tauri::command]
fn play_note_command(note: String, state: State<'_, Mutex<AppState>>) -> Result<()> {
    let state = state.lock().map_err(|e| Error::Generic(e.to_string()))?;

    let animal = Arc::clone(&state.animal);
    let sounds = Arc::clone(&state.sounds);

    play_note(note, animal, sounds)
}

fn play_note(note: String, animal: Arc<Animal>, sounds: Arc<AnimalSounds>) -> Result<()> {
    match Note::new(&note) {
        Some(note) => {
            let sound = animal.sound(&sounds, &note)?;

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

    state.animal = Arc::new(
        Animal::new(&animal)
            .ok_or_else(|| Error::Generic(format!("Invalid animal: {}", animal)))?,
    );

    Ok(())
}

#[tauri::command]
async fn reconnect_port(app_state: State<'_, Mutex<AppState>>) -> Result<String> {
    // Clone the port sender before locking the mutex
    let sender = {
        let state = app_state
            .lock()
            .map_err(|e| Error::Generic(e.to_string()))?;
        Arc::clone(&state.port_sender)
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
    mut port_receiver: Receiver<Arc<Option<SerialPortInfo>>>,
    note_sender: Arc<Sender<String>>,
) {
    loop {
        if let Some(port) = port_receiver.recv().await {
            let port = match port.as_ref() {
                Some(port) => port,
                None => {
                    println!("No port found");
                    continue;
                }
            };

            let serial = match serialport::new(&port.port_name, 115200)
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
            println!("Serial port opened: {}", port.port_name);

            loop {
                let mut buf = String::new();
                match serial.read_line(&mut buf) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        let note = buf.trim().to_string();
                        println!("Received {} bytes: {:?}", bytes_read, &buf);
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
}
