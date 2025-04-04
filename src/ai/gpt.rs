use std::env;
use reqwest::blocking::Client;
use serde_json::json; // 🛠️ ← これが必要だった！

pub fn ask_gpt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = env::var("OPENAI_API_KEY")?;
    let client = Client::new();

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(key)
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                { "role": "system", "content": "You are a poetic spell analyst. Respond with mystic flair." },
                { "role": "user", "content": prompt }
            ]
        }))
        .send()?;

    let raw_json: serde_json::Value = res.json()?;
    println!("🧪 DEBUG: {:?}", raw_json); // ← これ追加！

    let content = raw_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("No divine insight received.");

    Ok(content.to_string())
}
