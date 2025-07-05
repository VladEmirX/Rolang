use crate::lexer::{State, Token, TokenIterator};
use crate::lexer::lex::skip_while_op;
use crate::lexer::TokenType::Operator;

pub fn lex_operator(it: &mut TokenIterator) -> Token {
    let Some(State{num, col, ..}) = it.current
        else { unreachable!() };
    let row = it.row;
    let line = it.lines[row].clone();
    
    skip_while_op(it);
    
    let end_pos= if let Some(State{num : end_pos @ 1.., ..}) = it.current { 
        end_pos
    } else {
        line.len()
    };
    
    //todo: mb add some more difficult behaviour if needed
    
    Token{
        slice: line.slice(num..end_pos),
        row,
        col,
        num,
        ty: Operator,
    }
}