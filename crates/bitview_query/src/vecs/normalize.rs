use std::mem::take;

pub fn normalize(text: &str) -> String {
    words(text).join(" ")
}

fn words(text: &str) -> Vec<String> {
    let bytes = text.as_bytes();
    let mut words = Vec::new();
    let mut word = String::new();

    for (index, char) in text.char_indices() {
        let is_decimal_point = char == '.'
            && index > 0
            && bytes[index - 1].is_ascii_digit()
            && bytes
                .get(index + 1)
                .is_some_and(|byte| byte.is_ascii_digit());
        if char.is_ascii_alphanumeric()
            || matches!(char, '<' | '>' | '=' | '+' | '%')
            || is_decimal_point
        {
            word.push(char.to_ascii_lowercase());
        } else if !word.is_empty() {
            words.push(take(&mut word));
        }
    }

    if !word.is_empty() {
        words.push(word);
    }
    words
}

#[cfg(test)]
#[path = "../../tests/unit/vecs/normalize.rs"]
mod tests;
