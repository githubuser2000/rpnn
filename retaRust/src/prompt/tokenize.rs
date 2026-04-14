#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TokenizedLine {
    pub tokens: Vec<String>,
}

pub fn split_shell_like(input: &str) -> Result<TokenizedLine, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escape = false;

    for ch in input.chars() {
        if escape {
            current.push(ch);
            escape = false;
            continue;
        }
        match ch {
            '\\' => {
                escape = true;
            }
            '\'' | '"' => {
                if let Some(active) = quote {
                    if active == ch {
                        quote = None;
                    } else {
                        current.push(ch);
                    }
                } else {
                    quote = Some(ch);
                }
            }
            c if c.is_whitespace() && quote.is_none() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if escape {
        return Err("Zeile endet mit einem offenen Escape-Zeichen".to_string());
    }
    if quote.is_some() {
        return Err("Zeile enthält ein nicht geschlossenes Quote-Zeichen".to_string());
    }
    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(TokenizedLine { tokens })
}
