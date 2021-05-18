use std::fs;
use structopt::StructOpt;

#[doc = "Syntax"]
/**
    Example:
    - set a = 0;
    - $a += 1;
    - print $a;
    - jump -2;

    Commands:
    - set
    - jump
    - print

    Operators:
    - Plus (+)
    - Minus (-)
    - Divide (/)
    - Multiply (*)

    Constants:
    - $PI
    - $E
*/

#[derive(Debug, StructOpt)]
#[structopt(name = "mahou", about = "A programming language")]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    /// The input file to be interpreted
    filename: String,
}

/// Check if a given character is whitespace
fn is_char_whitespace(ch: char) -> bool {
    match ch {
        '\t' | ' ' | '\n' => true,
        _ => false,
    }
}

/// Check if a character is an symbol
fn is_char_symbol(ch: char) -> bool {
    match ch {
        '+' | '-' | '*' | '/' | '>' | '<' | '=' | ';' | '$' => true,
        _ => false,
    }
}

/// Check if a character is in between 0 and 9
fn is_char_numeric(ch: char) -> bool {
    return ch.is_digit(10);
}

/// Check if the current character or the next character will end the token
fn ends_token(cur: char, next: char) -> bool {
    if is_char_whitespace(next) {
        return true;
    }
    if is_char_symbol(cur) {
        return true;
    }
    if is_char_symbol(next) {
        return true;
    }
    if is_char_whitespace(cur) {
        return false;
    }
    return false;
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tokens {
    Assign,
    Var,
    Set,
    Jump,
    Print,
    Minus,
    Plus,
    Divide,
    Multiply,
    Semi,
    Identifier,
    Numeric,
}

/// Representative of the order the index needs to be incremented in
#[derive(PartialEq, Debug)]
enum Increment {
    Before,
    After,
}

#[derive(PartialEq, Debug)]
struct Token {
    part: String,
    token: Tokens,
}

/// Given a string reference that has been identified as a single token, find what token it is
fn tokenize(part: &str) -> Token {
    let mut token: Tokens = match part {
        "-" => Tokens::Minus,
        "+" => Tokens::Plus,
        "/" => Tokens::Divide,
        "*" => Tokens::Multiply,
        "=" => Tokens::Assign,
        "$" => Tokens::Var,
        ";" => Tokens::Semi,
        "set" => Tokens::Set,
        "jump" => Tokens::Jump,
        "print" => Tokens::Print,
        _ => Tokens::Identifier,
    };

    // Both identifiers and numeric literals get assigned Identifier
    // but this check if each character in the identifier is a number
    if token == Tokens::Identifier {
        for c in part.chars() {
            if is_char_numeric(c) {
                token = Tokens::Numeric;
                break;
            }
        }
    }

    return Token {
        part: part.to_owned(),
        token,
    };
}

/// Given a string, find what tokens it's made up of
trait Lex {
    fn move_pointer(&mut self);
    fn next(&mut self, increment: Increment);
    fn lexer(&mut self);
}

/// The parts of data needed to make tokens
struct Lexer {
    contents: String,
    chars: Vec<char>,
    index: usize,
    previous_char: char,
    current_char: char,
    next_char: char,
    tokens: Vec<Token>,
}

impl Lex for Lexer {
    /// Change what the character is by shifting them
    fn move_pointer(&mut self) {
        self.previous_char = self.current_char;
        self.current_char = self.next_char;
        self.next_char = self.chars[self.index];
    }
    /// Increment the index before or after the point moves depending on if increment is Before or After
    fn next(&mut self, increment: Increment) {
        if increment == Increment::Before {
            self.index += 1;
        }
        self.move_pointer();
        if increment == Increment::After {
            self.index += 1;
        }
    }
    /// Takes the contents and pushes what the tokenizer returns for each part
    fn lexer(&mut self) {
        // Get all the chars from the contents of the file
        self.chars = self.contents.chars().collect();
        let mut current_part: String = String::new();

        self.index = 0;
        let chars_len: usize = self.contents.len();

        while self.index + 1 <= chars_len {
            // If the character is not whitespace, push it to the current part
            if !is_char_whitespace(self.current_char) {
                current_part.push(self.current_char);
                // If the current character or the next ends the token
                // push the current part as a token, then reset the part
                if ends_token(self.current_char, self.next_char) {
                    self.tokens.push(tokenize(&current_part));
                    current_part = String::new();
                }
            }
            self.next(Increment::After);
        }
    }
}

/// Remove the boiler plate of making a lexer object
fn new_lexer(contents: &str) -> Lexer {
    let contents: String = contents.to_string() + "    ";
    let lexer: Lexer = Lexer {
        contents: contents.to_string(),
        chars: Vec::new(),
        index: 0,
        previous_char: ' ',
        current_char: ' ',
        next_char: ' ',
        tokens: Vec::new(),
    };

    return lexer;
}

fn main() {
    let args: Opt = Opt::from_args();

    let contents: String = fs::read_to_string(args.filename).expect("Error reading file");
    let mut lexer: Lexer = new_lexer(&contents);
    lexer.lexer();

    for tok in lexer.tokens.iter() {
        println!("{:?}:\t\t{}", tok.token, tok.part);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let mut lexer: Lexer = new_lexer("set a = 0;");
        lexer.lexer();
        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    part: "set".to_string(),
                    token: Tokens::Set
                },
                Token {
                    part: "a".to_string(),
                    token: Tokens::Identifier
                },
                Token {
                    part: "=".to_string(),
                    token: Tokens::Assign
                },
                Token {
                    part: "0".to_string(),
                    token: Tokens::Numeric
                },
                Token {
                    part: ";".to_string(),
                    token: Tokens::Semi
                }
            ]
        );

        let mut lexer: Lexer = new_lexer("jump -2;");
        lexer.lexer();
        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    part: "jump".to_string(),
                    token: Tokens::Jump
                },
                Token {
                    part: "-".to_string(),
                    token: Tokens::Minus
                },
                Token {
                    part: "2".to_string(),
                    token: Tokens::Numeric
                },
                Token {
                    part: ";".to_string(),
                    token: Tokens::Semi
                },
            ]
        );
    }

    #[test]
    fn tokenize_test() {
        assert_eq!(
            tokenize("set"),
            Token {
                token: Tokens::Set,
                part: "set".to_string()
            }
        );

        assert_eq!(
            tokenize("+"),
            Token {
                token: Tokens::Plus,
                part: "+".to_string()
            }
        );

        assert_eq!(
            tokenize("1"),
            Token {
                token: Tokens::Numeric,
                part: "1".to_string()
            }
        );

        assert_eq!(
            tokenize("a"),
            Token {
                token: Tokens::Identifier,
                part: "a".to_string()
            }
        );
    }
}
