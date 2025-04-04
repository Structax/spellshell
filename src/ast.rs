use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Modifier {
    Silently,
    Forcefully,
    For(Vec<String>),
    From(Vec<String>),
    To(Vec<String>),
    Custom(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Spell {
    Summon {
        target: Vec<String>,
        modifiers: Vec<Modifier>,
    },
    Banish {
        target: Vec<String>,
        modifiers: Vec<Modifier>,
    },
    Cloak {
        target: Vec<String>,
        modifiers: Vec<Modifier>,
    },
    Divine {
        target: Vec<String>,
        modifiers: Vec<Modifier>,
    },
    Teleport {
        target: Vec<String>,
        modifiers: Vec<Modifier>,
    },
}
