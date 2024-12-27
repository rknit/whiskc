use core::fmt;

use crate::ast::parsing::token::Delimiter;

#[derive(Clone)]
pub struct Punctuated<T> {
    pub items: Vec<T>,
    pub sep: Delimiter,
}
impl<T: fmt::Debug> fmt::Debug for Punctuated<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{:#?}", self.items).replace(',', format!("{}", self.sep).as_str());
        write!(f, "{}", s)
    }
}
