use std::vec::Vec;

// UTF-8 bytes encoding
pub fn encode_u8(text: &str) -> Vec<u8> {
  text.bytes().collect()
}

// lossy decodes as: U+FFFD REPLACEMENT CHARACTER � if byte not String
pub fn decode_u8(tokens: Vec<u8>) -> String {
  String::from_utf8_lossy(&tokens).to_string()
}

// UTF-16 code units encoding
pub fn encode_u16(text: &str) -> Vec<u16> {
  text.encode_utf16().collect()
}

pub fn decode_u16(tokens: Vec<u16>) -> String {
  String::from_utf16_lossy(&tokens)
}

// Unicode code points encoding
pub fn encode_u32(text: &str) -> Vec<u32> {
  text.chars().map(|c| c as u32).collect()
}

pub fn decode_u32(tokens: Vec<u32>) -> String {
  tokens
    .into_iter()
    .filter_map(|t| std::char::from_u32(t))
    .collect()
}