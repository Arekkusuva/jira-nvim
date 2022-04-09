pub fn wrap_text<'a>(text: &'a str, max_chars: usize) -> Vec<&'a str> {
    let mut result = Vec::new();
    if text.is_empty() {
        return result;
    }

    let mut start = 0;
    let mut len = 1;
    let mut prev_idx = 0;
    let mut prev_len = 0;
    for (idx, char) in text.char_indices() {
        len += 1;
        if len >= max_chars && prev_idx != 0 {
            result.push(&text[start..prev_idx]);
            len -= prev_len;
            start = prev_idx;
            prev_idx = 0;
            prev_len = 0;
        }
        if char.is_whitespace() {
            if len >= max_chars {
                result.push(&text[start..idx]);
                len = 0;
                start = idx;
                prev_idx = 0;
                prev_len = 0;
            } else {
                prev_idx = idx;
                prev_len = len;
            }
        }
    }

    if len != 0 {
        result.push(&text[start..]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_text_empty() {
        let wrapped = wrap_text("", 5);
        assert!(wrapped.is_empty());
    }

    #[test]
    fn wrap_text_ok() {
        let wrapped = wrap_text(" Text need to be wrapped", 5);
        assert_eq!(wrapped, vec![" Text", " need", " to", " be", " wrapped"]);
    }
}
