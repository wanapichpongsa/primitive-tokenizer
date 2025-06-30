use std::vec::Vec;

/*
  "Language Models are Unsupervised Multitask Learners"
  points out how it is inefficient to bytepair encode
  trivial variations such as 'dog!', 'dog?'.

  I find Regex unreadable, and Rust is incompatible with the Regex used by OpenAI: 
  self.pat = re.compile(r"""
  's|'t|'re|'ve|'m|'ll|'d
  | ?\p{L}+| ?\p{N}+
  | ?[^\s\p{L}\p{N}]+
  |\s+(?!\S)
  |\s+
  """)
  
  So I'm using simpler Eq assertions.

  NOTICE: .map_or() is used to default to false 
  when word.chars().last() -> None, 
  which would cause program to panick

  word.len() > 0 check is essential because pushing empty string will cause program to fail.
*/
pub fn split_english_grammar(text: &str) -> Vec<String> {
    let mut cleaned_strings: Vec<String> = Vec::new();
    let mut word = String::new();

    let chars: Vec<char> = text.chars().map(|c| c).collect();
    let mut i: usize = 0;
    while i < chars.len() {
        let ch = chars[i];
        // push as " word" to indicate word in sentence.
        if word == " " {
            word.push(ch);
        }
        else if ch.is_ascii_alphabetic() {
            if word.len() > 0 && word.chars().last().map_or(false, |l| !l.is_ascii_alphabetic()) {
                cleaned_strings.push(word.clone());
                word.clear();
                word.push(ch);
            } else {
                word.push(ch);
            }
        }
        else if ch == ' ' {
              // if contiguous to alphanumerical/unicode, always push prev and clear slate for new " word".
            if word.len() > 0 && (chars[i+1] != ' ' || word.chars().last().map_or(false, |l| l != ' ')) {
                cleaned_strings.push(word.clone());
                word.clear();
                word.push(ch);
            }
            // if not contiguous with alphanumerical/unicode, form contiguous whitespace (\s+)
            else {
              word.push(ch);
            }
        }
        else if ch.is_ascii_digit() {
            if word.len() > 0 && word.chars().last().map_or(false, |l| !l.is_ascii_digit()) {
                cleaned_strings.push(word.clone());
                word.clear();
                word.push(ch);
            } else {
                word.push(ch);
            }
        }
                              // curly unicode quote
        else if ch == '\'' || ch == '\u{2019}' {
            let mut _first_condition_matched = false;
            if i + 2 < chars.len() {
                match (chars[i+1], chars[i+2]) {
                    ('l', 'l') => {
                        if word.len() > 0 {
                            cleaned_strings.push(word.clone());
                            word.clear();
                        }
                        cleaned_strings.push("'ll".to_string());
                        _first_condition_matched = true;
                        i += 2;
                    },
                    ('v', 'e') => {
                        if word.len() > 0 {
                            cleaned_strings.push(word.clone());
                            word.clear();
                        }
                        cleaned_strings.push("'ve".to_string());
                        _first_condition_matched = true;
                        i += 2;
                    },
                    ('r', 'e') => {
                        if word.len() > 0 {
                            cleaned_strings.push(word.clone());
                            word.clear();
                        }
                        cleaned_strings.push("'re".to_string());
                        _first_condition_matched = true;
                        i += 2;
                    },
                    _ => {}
                }
            }
            if !_first_condition_matched && i + 1 < chars.len() {
                match chars[i+1] {
                    's' => {
                        if word.len() > 0 {
                            cleaned_strings.push(word.clone());
                            word.clear();
                        }
                        cleaned_strings.push("'s".to_string());
                        i += 1; // skip the 's'
                    },
                    'd' => {
                        if word.len() > 0 {
                            cleaned_strings.push(word.clone());
                            word.clear();
                        }
                        cleaned_strings.push("'d".to_string());
                        i += 1;
                    },
                    'm' => {
                        if word.len() > 0 {
                            cleaned_strings.push(word.clone());
                            word.clear();
                        }
                        cleaned_strings.push("'m".to_string());
                        i += 1;
                    },
                    't' => {
                        if word.len() > 0 {
                            cleaned_strings.push(word.clone());
                            word.clear();
                        }
                        cleaned_strings.push("'t".to_string());
                        i += 1;
                    },
                     _ => {
                         word.push(ch);
                    }
                }
            } else if !_first_condition_matched {
                word.push(ch);
            }
            _first_condition_matched = false;
        }
        // any special characters or non-ASCII unicode
        else {
            // I believe doesn't need length check because both truthy conditions
            // can't ever be satisfied under an empty string.
            if word.chars().last().map_or(
                false,
                |l| l.is_ascii_alphabetic() || l.is_ascii_digit() 
            ) {
                cleaned_strings.push(word.clone());
                word.clear();
                word.push(ch);
            } else {
              word.push(ch);
            }
        }

        // if final char push word
        if word.len() > 0 && i + 1 == chars.len() {
            cleaned_strings.push(word.clone());
        }
        i += 1;
    }
    cleaned_strings
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_english_grammar() {
        let input = "How're you World123?     Rust LLMs are so FUN!!!!";
        let result = split_english_grammar(input);
        println!("Result: {:?}", result);
        
        // Expected tokens based on the regex pattern
        let expected = vec!["How", "'re", " you", " World", "123", "?", "    ", " Rust", " LLMs", " are", " so", " FUN", "!!!!"];
        assert_eq!(result, expected);
    }
} 