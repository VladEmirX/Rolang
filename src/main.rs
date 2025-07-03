use std::error::Error;
use std::rc::Rc;
use imstr::string::ImString;

mod lexer;

pub type ImStrData = Rc<String>;
pub type ImStr = ImString<ImStrData>;


fn main() -> Result<(), Box<dyn Error>> {
    todo!()
}
