mod lex_checking;
mod tokenizer;

enum Token {
    IncrementPointer,
    DecrementPointer,
    IncrementCell,
    DecrementCell,
    RJumpBracket,
    LJumpBracket,
    PrintVal,
}

pub struct Lexer;

impl Lexer {
    pub fn tokenize() {}
}
