use self::lex::*;
use std::iter::Enumerate;
use crate::{ImStr, ImStrData};
use std::ops::Range;
use bitflags::bitflags;

mod lex;
mod lex_number;
mod lex_symbol;
mod lex_string;
mod lex_char;
mod lex_operator;

#[derive(Debug, Clone)]
pub struct Token{ 
    /// Token string representation
    pub slice: ImStr,
    /// Source code line number (from 0 to n)
    pub row: usize,
    /// First symbol number in line
    pub col: usize,
    /// First byte number in line
    pub num: usize,
    pub ty: TokenType,
}


#[derive(Debug, Clone)]
pub enum TokenType{
    Open, Close, Operator, Comma, Semicolon, Sharp,
    Symbol(ImStr),
    Keyword(ImStr),
    Argument(ImStr),
    Character{value: char, prefix_len: usize, suffix_len: usize, errors: CharErrorFlags},
    String{value: StringValue, quote_count: usize, prefix_len: usize, suffix_len: usize, errors: StringErrorFlags},
    Number{start_pos: usize, dot_pos: Option<usize>, exp_pos: Option<usize>, suf_pos: Option<usize>, errors: NumberErrorFlags},
    Indent,
    Other,
    Error,
}

#[derive(Debug, Clone)]
pub struct TokenIterator{
    lines: Vec<ImStr>,
    row: usize,
    iter: Enumerate<imstr::string::CharIndices<ImStrData>>,
    current: Option<State>,
}

#[derive(Debug, Clone, Copy)]
struct State {
    num: usize,
    col: usize,
    char: char,
}


#[derive(Debug, Clone, Default)]
pub enum StringValue
{
    #[default]
    Empty,
    SingleLine(ImStr),
    MultiLine{lines: Range<usize>, last_line_slice: ImStr},
}

pub fn is_keyword(str: &str) -> bool {
    matches!(str,
        |"_"
        |"and"
        |"as"
        |"by"
        |"class"
        |"const"
        |"else"
        |"fn"
        |"for"
        |"in"
        |"is"
        |"match"
        |"mod"
        |"mut"
        |"not"
        |"or"
        |"out"
        |"priv"
        |"pub"
        |"return"
        |"then"
        |"trait"
        |"type"
        |"use"
        |"with"
        |"while"
        |"yield"
    )
}

pub const NUMBER_DELIMITER: char = '\'';

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, )]
    pub struct NumberErrorFlags: u8 {
        const BAD_SUFFIX = 0b0000_0001;
        const BAD_EXPONENT = 0b0000_0010;
        const NO_START_DIGITS = 0b0000_0100;
        const FREE_DOT = 0b0000_1000;

        const _ = 0b0000_1111;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, )]
    pub struct StringErrorFlags: u8 {
        const UNCLOSED = 0b0000_0001;

        const _ = 0b0000_0001;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, )]
    pub struct CharErrorFlags: u8 {
        const UNCLOSED = 0b0000_0001;
        const EMPTY = 0b0000_0010;
        const BAD_ESC_SEQUENCE = 0b0000_0100;
        const INVALID_CODEPOINT = 0b0000_1000;

        const _ = 0b0000_1111;
    }
}


