pub mod lexer;
pub mod parser;
pub mod parsing;

pub const LEXEME_SPACE_FZ: u32 = 522;

#[derive(Debug)]
pub enum Token {
    IncrementPointer,
    DecrementPointer,
    IncrementCell,
    DecrementCell,
    RJumpBracket,
    LJumpBracket,
    PrintVal,
}

#[derive(Debug)]
pub enum FrontendError {
    NotRecognizedLexeme(u32),
    SrcCodeIndexOutOfBounds,
}
