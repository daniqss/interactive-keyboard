use std::fmt::Display;

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
