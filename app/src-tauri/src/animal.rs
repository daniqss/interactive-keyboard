use rodio::source::Speed;
use rodio::Decoder;
use rodio::Source;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tauri::{
    path::{BaseDirectory, PathResolver},
    Wry,
};

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
    elephant_path: PathBuf,
    tiger_path: PathBuf,
    dog_path: PathBuf,
    dolphin_path: PathBuf,
}

impl AnimalSounds {
    pub fn new(path: &PathResolver<Wry>) -> Result<Self> {
        Ok(AnimalSounds {
            elephant_path: path.resolve("assets/elephant_sound.mp3", BaseDirectory::Resource)?,
            tiger_path: path.resolve("assets/elephant_sound.mp3", BaseDirectory::Resource)?,
            dog_path: path.resolve("assets/elephant_sound.mp3", BaseDirectory::Resource)?,
            dolphin_path: path.resolve("assets/elephant_sound.mp3", BaseDirectory::Resource)?,
        })
    }

    fn load_decoder(path: &PathBuf) -> Result<Decoder<BufReader<File>>> {
        let file = File::open(path)?;
        Ok(Decoder::new(BufReader::new(file))?)
    }
}
