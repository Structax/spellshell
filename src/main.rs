use std::env;

mod parser;
mod executor;
mod spellbook;
mod ast; // 必要に応じて
mod commands; // 🪄 commands（divine）モジュールを登録
mod ai;       // ✨ gpt.rs を使っているならこれも

fn main() {
    dotenv::dotenv().ok(); // 🧙‍♂️ .spellrc を読み込む

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cast \"<spell>\" or cast <file>.spell");
        return;
    }

    let input = &args[1];

    if input.ends_with(".spell") {
        spellbook::run_spellbook(input);
    } else {
        match parser::parse_spell(input) {
            Ok(spell) => executor::execute(spell),
            Err(err) => eprintln!("💥 Failed to parse spell: {}", err),
        }
    }
}
