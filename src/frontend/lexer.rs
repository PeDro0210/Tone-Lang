use std::iter::Peekable;

use crate::frontend::{FrontendError, Token, parsing::SrcCodeIter};

pub struct Lexer {
    src_code_iter: Peekable<SrcCodeIter>,
}

impl Lexer {
    /// Initialize lexer struct
    pub fn new(src_code_iter: SrcCodeIter) -> Self {
        Lexer {
            src_code_iter: src_code_iter.peekable(),
        }
    }
    /// get's the actual token for the lexeme corresponding
    pub fn get_tkn(&mut self) -> Result<Token, FrontendError> {
        if let Some(lexeme) = self.src_code_iter.peek() {
            return match lexeme {
                261 => Ok(Token::IncrementPointer),
                294 => Ok(Token::DecrementPointer),
                329 => Ok(Token::IncrementCell),
                349 => Ok(Token::DecrementCell),
                392 => Ok(Token::RJumpBracket),
                440 => Ok(Token::LJumpBracket),
                493 => Ok(Token::PrintVal),
                _ => Err(FrontendError::NotRecognizedLexeme(*lexeme)),
            };
        }
        Err(FrontendError::SrcCodeIndexOutOfBounds)
    }
    /// get's the next token for the lexeme corresponding
    pub fn get_next_tkn(&mut self) -> Result<Token, FrontendError> {
        loop {
            if let Some(lexeme) = self.src_code_iter.next() {
                return match lexeme {
                    261 => Ok(Token::IncrementPointer),
                    294 => Ok(Token::DecrementPointer),
                    329 => Ok(Token::IncrementCell),
                    349 => Ok(Token::DecrementCell),
                    392 => Ok(Token::RJumpBracket),
                    440 => Ok(Token::LJumpBracket),
                    493 => Ok(Token::PrintVal),
                    _ => Err(FrontendError::NotRecognizedLexeme(lexeme)),
                };
            }
            return Err(FrontendError::SrcCodeIndexOutOfBounds);
        }
    }
}
