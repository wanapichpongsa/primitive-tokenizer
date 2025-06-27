mod bytepair;
use crate::bytepair::{encode, most_frequent_codepoint};

fn main() {
    let text: &str = "สวัสดีครับ 👋 Rust is fun to learn";

    let tokens = encode(text);
    // us: 33; gpt4o: 12
    let most_frequent = most_frequent_codepoint(tokens);
    println!("{:?}", most_frequent);
}
