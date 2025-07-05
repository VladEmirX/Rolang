use std::ops::Index;
use crate::lexer::{Token, TokenIterator, State};
use unicode_properties::*;
use crate::ImStr;
use crate::lexer::lex_char::lex_char;
use crate::lexer::lex_number::lex_number;
use crate::lexer::lex_operator::lex_operator;
use crate::lexer::lex_string::lex_string;
use crate::lexer::lex_symbol::lex_symbol;
use crate::lexer::TokenType::*;



impl TokenIterator {
    pub fn new(mut str: &str) -> Self {     
        if str.is_empty() {
            str = " ";
        }
        
        let lines : Vec<_> = str
            .lines()
            .map(|str| ImStr::from(" ".to_string() + str.trim_end()))
            .collect();
        
        
        let mut result = TokenIterator{
            row: 0,
            iter: lines[0].char_indices().enumerate(),
            lines,
            current: None,
        };
        result.next_char();
        result
    }
    
    pub(super) fn next_char(&mut self) -> &mut Self {
        loop {
            if let Some((col, (num, char))) = self.iter.next() {
                self.current = Some(State { num, col, char });
                break;
            }
            if self.row + 1 == self.lines.len() {
                self.current = None;
                break;
            }
            self.row += 1;
            self.iter = self.lines[self.row].char_indices().enumerate();
        };
        self
    }

    pub(super) fn next_line(&mut self) -> &mut Self {
        loop {
            if self.row + 1 == self.lines.len() {
                self.current = None;
                self.iter = ImStr::default().char_indices().enumerate();
                break;
            }
            self.row += 1;
            self.iter = self.lines[self.row].char_indices().enumerate();
            if let Some((_, (_, char))) = self.iter.next() {
                self.current = Some(State { num: 0, col: 0, char });
                break;
            }
        };
        self
    }
    
    #[inline]
    #[allow(unused)]
    pub(super) fn lines(&self) -> &(impl IntoIterator + Index<usize>) {
        &self.lines
    }
}

impl Iterator for TokenIterator {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let start @ State { num, col, char } = self.current?;
        let row = self.row;
        
        if num == 0 {
            return Some(lex_indent(self));
        }
        
        match (char, char.general_category(), char.general_category_group()) {
            (' ' | '\t', _, _) => {
                skip_whitespaces(self);
                self.next()
            },
            ('_', ..) | (_, _, GeneralCategoryGroup::Letter) => { Some(lex_symbol(self)) }
            ('"', ..) => { Some(lex_string(self, start)) }
            ('\'', ..) => { Some(lex_char(self, start)) }
            ('#', ..) => {
                self.next_char();
                Some(Token {
                    slice: '#'.into(),
                    row,
                    col,
                    num,
                    ty: Sharp,
                })
            }
            ('0'..='9', ..) => { Some(lex_number(self)) }
            (',', ..) => {
                self.next_char();
                Some(Token {
                    slice: ','.into(),
                    row,
                    col,
                    num,
                    ty: Comma,
                })
            }
            (';', ..) => {
                self.next_char();
                Some(Token {
                    slice: ';'.into(),
                    row,
                    col,
                    num,
                    ty: Semicolon,
                })
            }
            (_, GeneralCategory::OpenPunctuation | GeneralCategory::InitialPunctuation, GeneralCategoryGroup::Punctuation)
            => {
                self.next_char();
                Some(Token {
                    slice: char.into(),
                    row,
                    col,
                    num,
                    ty: Open,
                })
            }
            (_, GeneralCategory::ClosePunctuation | GeneralCategory::FinalPunctuation, GeneralCategoryGroup::Punctuation)
            => {
                self.next_char();
                Some(Token {
                    slice: char.into(),
                    row,
                    col,
                    num,
                    ty: Close,
                })
            }
            (_, _, GeneralCategoryGroup::Punctuation | GeneralCategoryGroup::Symbol)
            => { Some(lex_operator(self)) }
            _ => {
                self.next_char();
                Some(Token {
                    slice: char.into(),
                    row,
                    col,
                    num,
                    ty: Error,
                })
            }
        }        
    }
}

fn lex_indent(it: &mut TokenIterator) -> Token {
    skip_whitespaces(it);
    let Some(State {num, ..}) = it.current else { unreachable!() };
    
    Token{
        slice: it.lines[it.row].slice(0..num),
        row: it.row,
        num: 0,
        col: 0,
        ty: Indent,
    }
}

pub(super) fn skip_while(f: fn(char) -> bool, it: &mut TokenIterator) {
    while let Some(State { char, num: 1.., .. }) = it.current {
        if !f(char) {
            return;
        }
        it.next_char();
    }
}

pub(super) fn is_alnum(char: char) -> bool {
    char == '_' || matches!(
        char.general_category_group(),
        GeneralCategoryGroup::Letter | GeneralCategoryGroup::Number | GeneralCategoryGroup::Mark
    )
}

#[inline]
pub(super) fn skip_while_alnum(it: &mut TokenIterator) {
    skip_while(is_alnum, it)
}

pub(super) fn is_op(char: char) -> bool {
    !matches!(
        char,
        |';' |',' |'#' |'\'' |'"' |'_'
    ) && matches!(
        char.general_category_group(),
        GeneralCategoryGroup::Punctuation | GeneralCategoryGroup::Symbol
    )
}

#[inline]
pub(super) fn skip_while_op(it: &mut TokenIterator) {
    skip_while(is_op, it)
}

pub(super) fn skip_whitespaces(it: &mut TokenIterator) {
    while let Some(State {char: ' ' | '\t', ..}) = it.current {
        it.next_char();
    }
}


