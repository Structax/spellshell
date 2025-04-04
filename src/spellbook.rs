use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::parser::parse_spell;
use crate::executor::execute;

pub fn run_spellbook(path: &str) {
    let file = File::open(path).expect("Failed to open .spell file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(spell_line) = line {
            if spell_line.trim().is_empty() || spell_line.trim().starts_with('#') {
                continue; // コメントや空行はスキップ
            }
            match parse_spell(&spell_line) {
                Ok(spell) => execute(spell),
                Err(err) => eprintln!("⚠️ Failed to parse spell: {}", err),
            }
        }
    }
}
