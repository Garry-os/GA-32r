use crate::asm::asm::asm_err_info;

pub enum TokenType {
    Number,
    Text,
    Comma
}

pub struct Token {
    pub t_type: TokenType,
    pub content: String,
    pub number: i32,

    // Origins
    pub o_file: String,
    pub o_line: usize,
    pub o_index: usize
}

pub fn tokenize(src: &String, file: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut buffer = String::new();

    // Origin source
    let mut line: usize = 0;
    let mut index: usize = 0;

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
                number: 0,

                o_file: file.to_string(),
                o_line: line,
                o_index: index
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
                number: buffer.clone().parse::<i32>().expect("Not a number!"),

                o_file: file.to_string(),
                o_line: line,
                o_index: index
            });

            buffer.clear();
        }
        else if c == b'\n' {
            // Increase line
            line += 1;
            index = 0;
            i += 1;
        }
        else if c == b',' {
            i += 1;

            // Create a new token and push into token list
            tokens.push(Token {
                t_type: TokenType::Comma,
                content: ",".to_string(),
                number: 0,

                o_file: file.to_string(),
                o_line: line,
                o_index: index
            });
        }
        else if c.is_ascii_whitespace() {
            // Ignore whitespace
            i += 1;
        }
        else {
            // An invalid token
            asm_err_info(file, &line, &index, "Invalid token");
            i += 1;
        }

        index += 1;
    }

    tokens
}

