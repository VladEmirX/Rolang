extern crate core;

use std::rc::Rc;
use imstr::string::ImString;

pub mod lexer;
pub mod parser;

type ImStrData = Rc<String>;
type ImStr = ImString<ImStrData>;

