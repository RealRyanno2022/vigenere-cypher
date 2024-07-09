use std::fmt::Display;
use std::clone::Clone;
use std::marker::Copy;

pub struct Hello {
    name: String,
}
pub fn new_hello() -> Hello {
    Hello {
        name: "Rodrigo".to_string(),
    }
}
impl Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub(crate) const SIZE: usize = 192;

#[derive(Clone,Copy)]
pub(crate) struct DictWrap(pub(crate) [char; SIZE]);

