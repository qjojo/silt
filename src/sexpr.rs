use std::fmt;

#[allow(dead_code)]
pub struct Sexpr<T, S> {
    first: T,
    last: S,
    atomic: bool,
}

#[allow(dead_code)]
impl<T, S> Sexpr<T, S> {
    pub fn new(a: T, b: S, at: bool) -> Sexpr<T, S> {
        Sexpr {first: a, last: b, atomic: at}
    }

    pub fn is_atom(&self) -> bool {
        self.atomic
    }
}

impl<T, S> fmt::Display for Sexpr<T, S> where T: fmt::Display, S: fmt::Display{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.atomic {
            write!(f, "{}", self.first)
        } else {
            write!(f, "({}, {})", self.first, self.last)
        }
    }
}
