use rodio::source::Speed;
use rodio::Decoder;
use rodio::Source;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;

use crate::notes::Note;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Animal {
    Elephant,
    Tiger,
    Dog,
    Dolphin,
}

impl Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Animal::Elephant => write!(f, "Elefante"),
            Animal::Tiger => write!(f, "Tigre"),
            Animal::Dog => write!(f, "Perro"),
            Animal::Dolphin => write!(f, "Delfín"),
        }
    }
}

impl Animal {
    pub fn new(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "elefante" => Some(Animal::Elephant),
            "tigre" => Some(Animal::Tiger),
            "perro" => Some(Animal::Dog),
            "delfín" => Some(Animal::Dolphin),
            _ => None,
        }
    }

    pub fn sound(
        &self,
        sounds: &AnimalSounds,
        note: &Note,
    ) -> Result<Speed<Decoder<BufReader<File>>>> {
        let semitone = note.get_semitone();
        let reproduce_semitone = |s| 2.0f32.powf(s as f32 / 12.0);

        let decoder = match self {
            Animal::Elephant => AnimalSounds::load_decoder(&sounds.elephant_path)?,
            Animal::Tiger => AnimalSounds::load_decoder(&sounds.tiger_path)?,
            Animal::Dog => AnimalSounds::load_decoder(&sounds.dog_path)?,
            Animal::Dolphin => AnimalSounds::load_decoder(&sounds.dolphin_path)?,
        };

        Ok(decoder.speed(reproduce_semitone(semitone)))
    }
}

pub struct AnimalSounds {
    elephant_path: String,
    tiger_path: String,
    dog_path: String,
    dolphin_path: String,
}

impl AnimalSounds {
    pub fn new() -> Result<Self> {
        Ok(AnimalSounds {
            elephant_path: "../assets/elephant_sound.mp3".to_string(),
            tiger_path: "../assets/tiger_sound.mp3".to_string(),
            dog_path: "../assets/dog_sound.mp3".to_string(),
            dolphin_path: "../assets/dolphin_sound.mp3".to_string(),
        })
    }

    fn load_decoder(path: &str) -> Result<Decoder<BufReader<File>>> {
        let file = File::open(path)?;
        Ok(Decoder::new(BufReader::new(file))?)
    }
}
