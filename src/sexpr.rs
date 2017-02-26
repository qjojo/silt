enum Atom {
    Nil,
    Int(i32),
    Fun(fn(Sexpr) -> Sexpr),
}

struct Sexpr {
    list: Vec<Atom>,
}

impl Sexpr {
    fn new(a: Atom) -> Sexpr {
        Sexpr {list: vec![]}
    }

    fn add(&self, a: Atom) {
        self.list.push(a);
    }
}

macro_rules! sexpr {
    ( $( $x:expr),* ) => {
        {
            let mut temp_sexpr = Sexpr::new();
            $(
                temp_sexpr.list.push($x);
            )*
            temp_sexpr
        }
    };
}
