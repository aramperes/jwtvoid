use anyhow::anyhow;
use anyhow::Context;
use json::JsonValue;
use std::io::{stdin, BufRead};

fn main() -> anyhow::Result<()> {
    // TODO: -s/--strict flag: exit when invalid token is found
    // TODO: -p/--preserve flag: preserves the signature portion in the output

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = line.with_context(|| "Failed to get line")?;
        match apply(line) {
            Ok(line) => println!("{}", line),
            Err(e) => eprintln!("Invalid token: {}", e),
        }
    }
    Ok(())
}

fn apply(line: String) -> anyhow::Result<String> {
    if let Some((header, payload)) = get_header_payload_str(&line) {
        let header = decode(&header).with_context(|| "Failed to decode header")?;
        let header = set_alg(&header).with_context(|| "Failed to parse header")?;
        let header = encode(&header);
        Ok(format!("{}.{}", header, payload))
    } else {
        Err(anyhow!("Not enough parts (header, payload)"))
    }
}

/// Tries to split a string split by a '.' into two parts.
fn get_header_payload_str(token: &str) -> Option<(String, String)> {
    let mut splits = [Option::<&str>::None; 2];

    token
        .split('.')
        .take(2)
        .enumerate()
        .for_each(|(i, s)| splits[i] = Some(s));

    if splits.iter().all(Option::is_some) {
        Some((
            splits[0].unwrap().to_string(),
            splits[1].unwrap().to_string(),
        ))
    } else {
        None
    }
}

/// Tries to decode a base64url string.
fn decode(part: &str) -> Option<String> {
    base64_url::decode(part)
        .ok()
        .map(String::from_utf8)
        .map(Result::ok)
        .flatten()
}

/// Encodes to a base64url string.
fn encode(part: &str) -> String {
    base64_url::encode(part)
}

/// Parses the string as JSON and sets the 'alg' to 'none'.
fn set_alg(header: &str) -> Option<String> {
    let mut j = json::parse(header).ok()?;
    if !j.is_object() {
        return None;
    }
    j["alg"] = JsonValue::String("none".into());
    Some(j.dump())
}
