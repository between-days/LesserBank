use core::fmt;

#[derive(Debug)]
pub enum RepoError {
    NotFound,
    Other,
}
impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}