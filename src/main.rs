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

/// This is the structure that represents a single token
#[derive(PartialEq, Debug)]
struct Token {
    part: String,
    token: Tokens,
    line_num: i64,
    char_num: i64,
}

/// Given a string reference that has been identified as a single token, find what token it is
fn tokenize(part: &str) -> Tokens {
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
    return token;
}

/// Given a string, find what tokens it's made up of
trait Lex {
    fn move_pointer(&mut self);
    fn next(&mut self);
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
    fn next(&mut self) {
        self.move_pointer();
        self.index += 1;
    }
    /// Takes the contents and pushes what the tokenizer returns for each part
    fn lexer(&mut self) {
        // Get all the chars from the contents of the file
        self.chars = self.contents.chars().collect();
        let mut current_part: String = String::new();

        self.index = 0;
        let mut line_num: i64 = 1;
        let chars_len: usize = self.contents.len();

        while self.index + 1 <= chars_len {
            // Check for newlines
            if self.current_char == '\n' {
                line_num += 1;
                self.next();
                continue;
            }
            // If the character is not whitespace, push it to the current part
            if !is_char_whitespace(self.current_char) {
                current_part.push(self.current_char);
                // If the current character or the next ends the token
                // push the current part as a token, then reset the part
                if ends_token(self.current_char, self.next_char) {
                    let token_type: Tokens = tokenize(&current_part);
                    // Get size of the part for character num
                    let char_num = self.index as i64 - current_part.len() as i64;
                    let token: Token = Token {
                        token: token_type,
                        part: current_part,
                        line_num,
                        char_num,
                    };
                    self.tokens.push(token);
                    current_part = String::new();
                }
            }
            self.next();
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

fn spacer(num: usize, ch: char) -> String {
    let mut space: String = String::new();
    for _ in 0..num {
        space.push(ch);
    }
    return space;
}

fn print(tok: &Token) {
    let token_text: String = format!("{:?}", tok.token);
    let first: String = spacer(14 - token_text.len(), ' ');
    let second: String = spacer(10 - tok.part.len(), ' ');
    println!(
        "{}{}{}{}{}:{}",
        token_text, first, tok.part, second, tok.line_num, tok.char_num
    );
}

fn main() {
    let args: Opt = Opt::from_args();

    let contents: String = fs::read_to_string(args.filename).expect("Error reading file");
    let mut lexer: Lexer = new_lexer(&contents);
    lexer.lexer();

    // Print source code header
    println!("Source code:");
    println!("{}", spacer(28, '-'));
    print!("{}", contents);
    println!("{}\n", spacer(28, '-'));

    // Print the table column names
    let label: String = format!(
        "Type{}Part{}Line",
        spacer(14 - "Type".len(), ' '),
        spacer(10 - "Part".len(), ' ')
    );
    println!("{}", label);
    println!("{}", spacer(28, '-'));

    // Print all the tokens
    for tok in lexer.tokens.iter() {
        print(tok);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let mut lexer: Lexer = new_lexer("set a");
        lexer.lexer();
        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    part: "set".to_string(),
                    token: Tokens::Set,
                    line_num: 1,
                    char_num: 1,
                },
                Token {
                    part: "a".to_string(),
                    token: Tokens::Identifier,
                    line_num: 1,
                    char_num: 5,
                },
            ]
        );

        let mut lexer: Lexer = new_lexer("jump -2");
        lexer.lexer();
        assert_eq!(
            lexer.tokens,
            vec![
                Token {
                    part: "jump".to_string(),
                    token: Tokens::Jump,
                    line_num: 1,
                    char_num: 1,
                },
                Token {
                    part: "-".to_string(),
                    token: Tokens::Minus,
                    line_num: 1,
                    char_num: 6,
                },
                Token {
                    part: "2".to_string(),
                    token: Tokens::Numeric,
                    line_num: 1,
                    char_num: 7,
                },
            ]
        );
    }

    #[test]
    fn tokenize_test() {
        assert_eq!(tokenize("set"), Tokens::Set);
        assert_eq!(tokenize("+"), Tokens::Plus);
        assert_eq!(tokenize("1"), Tokens::Numeric);
        assert_eq!(tokenize("a"), Tokens::Identifier);
    }
}
