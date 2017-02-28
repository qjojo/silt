use std::fmt;
use std::str::FromStr;


/// lisp according to McCarthy's predicates
/// F-function implementation status
///
/// atom    - true if argument is an atom           [x]
/// eq      - true if atoms are equal               [x]
/// car     - returns first half of cons cell       [X]
/// cdr     - returns second half of cons cell      [X]
/// cons    - makes a new cons cell                 [X]
///
/// S-function implementation status
///
/// read    - read an expression from string
/// quote   - represent an expression               [ ]
/// cond    - conditional branch                    [ ]
/// lambda  - define function                       [ ]
/// label   - works like goto                       [ ]
/// eval    - evaluates a form                      [ ]

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub struct Sexpr<T, S> {
    first: T,
    last: S,
    atomic: bool,
}

pub enum Atom<T> {
    Nil,
    Int(i32),
    Function(fn(T) -> T)
}

#[allow(dead_code)]
impl<T, S> Sexpr<T, S> {
    pub fn new(a: T, b: S, at: bool) -> Sexpr<T, S> {
        Sexpr {first: a, last: b, atomic: at}
    }

    pub fn atom(&self) -> bool {
        self.atomic
    }

    pub fn car(&self) -> &T {
        &(self.first)
    }

    pub fn cdr(&self) -> &S {
        &(self.last)
    }
}

impl<T, S> fmt::Display for Sexpr<T, S> where T: fmt::Display, S: fmt::Display{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.atomic {
            write!(f, "{}", self.first)
        } else {
            write!(f, "({} {})", self.first, self.last)
        }
    }
}

#[allow(dead_code)]
pub fn cons<T, S, U, V>(a: &Sexpr<T, S>, b: &Sexpr<U, V>) -> Sexpr<T, U> where
    T: Clone,
    U: Clone {
    if !a.atom() || !b.atom() {
        panic!("attempted to use cons on non-atoms");
    }
    Sexpr::new(a.car().clone(), b.car().clone(), false)
}

#[allow(dead_code, unused_imports, unused_assignments)]
pub fn read(instr: String) -> Sexpr<String, String> {
    let mut t_car = String::new();
    let mut t_cdr = String::new();
    scan!(instr.bytes() => "({} {})", t_car, t_cdr);
    Sexpr::new(t_car, t_cdr, false)
}

fn is_numeric<T: FromStr>(s: &str) -> bool {
    s.parse::<T>().is_ok()
}

/*
pub fn to_symbol(str_sexpr: Sexpr<String, String>) {
    let s_car = str_sexpr.car();
    let s_cdr = str_sexpr.cdr();
    let r_car;
    let r_cdr;
    if is_numeric(s_car) {
        let r_car = s_car.parse().unwrap();
    } else {
        let r_car = s_car.Clone();
    }

    if str_sexpr.atom() {
        return Sexpr::new(r_car, 0, true)
    }
    if is_numeric(s_cdr) {
        let r_cdr = s_cdr.parse().unwrap();
    } else {
        let r_cdr = s_cdr.Clone();
    }
}
*/
