use crate::animal::{Animal, AnimalSounds};
use crate::notes::Note;
use crate::prelude::*;
use rodio::{OutputStream, Sink};

pub fn play_note(note: String, animal: Arc<Animal>, sounds: Arc<AnimalSounds>) -> Result<()> {
    match Note::new(&note) {
        Some(note) => {
            let sound = animal.sound(&sounds, &note)?;

            #[cfg(debug_assertions)]
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
