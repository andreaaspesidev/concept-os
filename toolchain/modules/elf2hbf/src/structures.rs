use std::fmt;

/*
    Structures
*/
#[derive(Debug, Clone)]
pub struct HbfError {
    pub msg: String
}
impl fmt::Display for HbfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl std::error::Error for HbfError {}

