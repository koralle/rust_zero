use std::{
    error::Error,
    fmt::{self, Display},
};

/// 抽象構文木を表現するための型
#[derive(Debug)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

/// パースエラーを表すための型
#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char),
    InvalidRightParen(usize),
    NoPrev(usize),
    NorightParen,
    Empty,
}

/// parse_plus_star_question関数で利用するための列挙型
pub enum PSQ {
    Plus,
    Star,
    Question,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "ParseError: invalid escape: pos = {pos}, char = '{c}")
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "ParseError: invalid right parenthesis: pos = {pos}")
            }
            ParseError::NoPrev(pos) => {
                write!(f, "ParseError: invalid previous expression: pos = {pos}")
            }
            ParseError::NorightParen => {
                write!(f, "ParseError: invalid no right parenthesis")
            }
            ParseError::Empty => write!(f, "ParseError: empty expression"),
        }
    }
}

impl Error for ParseError {}

/// 特殊文字のエスケープ
fn parse_escape(pos: usize, c: char) -> std::result::Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '|' | '+' | '*' | '?' => Ok(AST::Char(c)),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

/// +, *, ?をASTに変換
///
/// 後置記法で、+, *, ?の前にパターンがない場合はエラー
fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> std::result::Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}
