pub enum TokenType {
    Number,
    Text,
    Comma
}

pub struct Token {
    pub t_type: TokenType,
    pub content: String,
    pub number: i32
}

pub fn tokenize(src: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();

    let mut i: usize = 0;
    let src8 = src.as_bytes();
    while i < src8.len() {
        let c = src8[i];

        if c.is_ascii_alphabetic() {
            // Save to the buffer
            buffer.push(c as char);
            i += 1;

            // Keep saving until the end of the token
            while i < src8.len() && src8[i].is_ascii_alphabetic() {
                buffer.push(src8[i] as char);
                i += 1;
            }

            // Create a new token and push into token list
            tokens.push(Token {
                t_type: TokenType::Text,
                content: buffer.clone(),
                number: 0
            });

            buffer.clear();
        }
        else if c.is_ascii_digit() {
            // Save to the buffer
            buffer.push(c as char);
            i += 1;

            // Keep saving until the end of the token
            while i < src8.len() && src8[i].is_ascii_digit() {
                buffer.push(src8[i] as char);
                i += 1;
            }

            // Create a new token and push into token list
            tokens.push(Token {
                t_type: TokenType::Number,
                content: buffer.clone(),
                number: buffer.clone().parse::<i32>().expect("Not a number!")
            });

            buffer.clear();
        }
        else if c.is_ascii_whitespace() {
            // Ignore whitespace
            i += 1;
        }
    }

    tokens
}

