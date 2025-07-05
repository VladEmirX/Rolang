use crate::lexer::{is_keyword, skip_while_alnum, State, Token, TokenIterator};
use crate::lexer::lex_char::lex_char;
use crate::lexer::lex_string::lex_string;
use crate::lexer::TokenType::{Error, Keyword, Symbol};


pub fn lex_symbol(it: &mut TokenIterator) -> Token {
    let Some(start @ State { num, col, char: _ }) = it.current else { unreachable!() };
    let row = it.row;
    let line = it.lines[row].clone();

    let slice 
        = |&mb_state|  
        line.slice(num
       ..if let Some(State{ num: curr_num, ..})
            = mb_state
        { curr_num } else
        { line.len()});
    
    skip_while_alnum(it);

    match it.current {
        Some(State {char:'"', ..}) => {
            lex_string(it, start)
        }
        Some(State {char:'\'', ..}) => {
            lex_char(it, start)
        }
        Some(State {char:'#', num: sharp_num, ..}) => {
            skip_while_alnum(it);

            let slice = slice(&it.current) ;
            let identifier = slice.slice(sharp_num + 1 - num ..);
            let prefix = &line[num..sharp_num];
            Token {
                ty: match prefix {
                    "k" => Keyword (identifier),
                    "r" => Symbol (identifier),
                    _ => Error
                },
                slice,
                row,
                col,
                num,
            }
        }
        _ => {
            let slice = slice(&it.current) ;
            Token{
                ty: if is_keyword(slice.as_str()) {
                    Keyword( slice.clone())
                } else {Symbol( slice.clone())},
                slice,
                row,
                col,
                num,
            }
        }
    }
}