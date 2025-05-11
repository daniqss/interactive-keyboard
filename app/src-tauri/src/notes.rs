#[derive(Debug, PartialEq, Eq)]
pub enum Note {
    Do,
    Re,
    Mi,
    Fa,
    Sol,
    La,
    Si,
    DoSharp,
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Note::Do => write!(f, "do"),
            Note::Re => write!(f, "re"),
            Note::Mi => write!(f, "mi"),
            Note::Fa => write!(f, "fa"),
            Note::Sol => write!(f, "sol"),
            Note::La => write!(f, "la"),
            Note::Si => write!(f, "si"),
            Note::DoSharp => write!(f, "do#"),
        }
    }
}

impl Note {
    pub fn new(note: &str) -> Option<Self> {
        match note.to_lowercase().as_str() {
            "do" => Some(Note::Do),
            "re" => Some(Note::Re),
            "mi" => Some(Note::Mi),
            "fa" => Some(Note::Fa),
            "sol" => Some(Note::Sol),
            "la" => Some(Note::La),
            "si" => Some(Note::Si),
            "do-sharp" => Some(Note::DoSharp),
            _ => None,
        }
    }

    pub fn get_semitone(&self) -> f32 {
        // from high to low
        match self {
            Note::Do => 0,
            Note::Re => 2,
            Note::Mi => 4,
            Note::Fa => 5,
            Note::Sol => 7,
            Note::La => 9,
            Note::Si => 11,
            Note::DoSharp => 12,
        }
    }

    pub fn sound(
        &self,
        sounds: &AnimalSounds,
        note: &Note,
    ) -> Result<Speed<Decoder<BufReader<File>>>> {
        let semitone = note.get_semitone();
        let reproduce_semitone = |s| 2.0f32.powf(s / 12.0);

        let decoder = match self {
            Animal::Elephant => AnimalSounds::load_decoder(&sounds.elephant_path)?,
            Animal::Tiger => AnimalSounds::load_decoder(&sounds.tiger_path)?,
            Animal::Dog => AnimalSounds::load_decoder(&sounds.dog_path)?,
            Animal::Dolphin => AnimalSounds::load_decoder(&sounds.dolphin_path)?,
        };

        Ok(decoder.speed(reproduce_semitone(semitone)))
    }
}
