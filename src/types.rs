//TODO: check if partial eq compares the List as its expected.
//TODO: string
//TODO: macros
//TODO: vector
//TODO: keywords
//TODO: hashmap

#[derive(Debug, PartialEq, Clone)]
pub enum MalType {
    Number(f64),
    Symbol(String),
    List(Vec<MalType>),
    Nil,
    Bool(bool),
    BuiltinFn(MalFn),
}

pub type MalFn = fn(Vec<MalType>) -> MalType;

impl std::fmt::Display for MalType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MalType::Number(x) => write!(f, "{}", x),
            MalType::Symbol(name) => write!(f, "{}", name),
            MalType::Nil => write!(f, "nil"),
            MalType::Bool(v) => write!(f, "{}", v),
            MalType::BuiltinFn(_) => write!(f, "builtin function"),
            MalType::List(l) => {
                write!(f, "(").unwrap();

                for el in l {
                    el.fmt(f).unwrap();
                    // not very elegant, but works
                    if el != l.last().unwrap() {
                        write!(f, " ").unwrap();
                    }
                }

                write!(f, ")")
            }
        }
    }
}

impl MalType {
    pub fn to_bool(&self) -> bool {
        !matches!(&self, MalType::Nil | MalType::Bool(false))
    }
}
