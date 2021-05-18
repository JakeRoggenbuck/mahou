use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mahou", about = "A programming language")]
struct Opt {
    #[structopt(short, long)]
    verbose: bool,

    /// The input file to be interpreted
    file: String,
}

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

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tokens {
    Assignment,
    Var,
    None,
}

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

fn tokenize(part: &str) -> Token {
    let token: Tokens = match part {
        "=" => Tokens::Assignment,
        "$" => Tokens::Var,
        _ => Tokens::None,
    };

    return Token {
        part: part.to_owned(),
        token,
    };
}

trait Lex {
    fn move_pointer(&mut self);
    fn next(&mut self, increment: Increment);
    fn lexer(&mut self);
}

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
    fn move_pointer(&mut self) {
        self.previous_char = self.current_char;
        self.current_char = self.next_char;
        self.next_char = self.chars[self.index];
    }
    fn next(&mut self, increment: Increment) {
        if increment == Increment::Before {
            self.index += 1;
        }
        self.move_pointer();
        if increment == Increment::After {
            self.index += 1;
        }
    }
    fn lexer(&mut self) {
        self.chars = self.contents.chars().collect();
        let mut current_part: String = String::new();

        self.index = 0;
        let chars_len: usize = self.contents.len();

        while self.index + 1 <= chars_len {}
    }
}

fn main() {
    println!("Hello, world!");
}
