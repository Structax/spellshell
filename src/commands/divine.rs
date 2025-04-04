use crate::ai::gpt;

pub fn handle_divine(target: Vec<String>) {
    let prompt = target.join(" ");
    match gpt::ask_gpt(&prompt) {
        Ok(response) => println!("🔮 {}", response),
        Err(e) => eprintln!("💥 GPT invocation failed: {}", e),
    }
}
