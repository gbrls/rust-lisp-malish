use std::rc::Rc;

//TODO: check if partial eq compares the List as its expected.
//TODO: string
//TODO: macros
//TODO: vector
//TODO: keywords
//TODO: hashmap

#[derive(Clone)]
pub enum MalType {
    Number(f64),
    Symbol(String),
    List(Vec<MalType>),
    Nil,
    Bool(bool),
    BuiltinFn(MalFn),
    UserFn(Rc<dyn Fn(Vec<MalType>) -> MalType>),
}

pub type MalFn = fn(Vec<MalType>) -> MalType;

//TODO: implement Display and Debug in the same way.
impl std::fmt::Display for MalType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MalType::Number(x) => write!(f, "{}", x),
            MalType::Symbol(name) => write!(f, "{}", name),
            MalType::Nil => write!(f, "nil"),
            MalType::Bool(v) => write!(f, "{}", v),
            MalType::BuiltinFn(_) => write!(f, "#<builtin_function>"),
            MalType::List(l) => {
                write!(f, "(").unwrap();

                let mut it = l.iter().peekable();

                while let Some(el) = it.next() {
                    el.fmt(f).unwrap();
                    if it.peek().is_some() {
                        write!(f, " ").unwrap();
                    }
                }

                write!(f, ")")
            }

            MalType::UserFn(_) => write!(f, "#<fn>"),
        }
    }
}

impl std::fmt::Debug for MalType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MalType::Number(x) => write!(f, "{}", x),
            MalType::Symbol(name) => write!(f, "{}", name),
            MalType::Nil => write!(f, "nil"),
            MalType::Bool(v) => write!(f, "{}", v),
            MalType::BuiltinFn(_) => write!(f, "#<builtin_function>"),
            MalType::List(l) => {
                write!(f, "(").unwrap();

                let mut it = l.iter().peekable();

                while let Some(el) = it.next() {
                    el.fmt(f).unwrap();
                    if it.peek().is_some() {
                        write!(f, " ").unwrap();
                    }
                }

                write!(f, ")")
            }

            MalType::UserFn(_) => write!(f, "#<fn>"),
        }
    }
}

impl MalType {
    pub fn to_bool(&self) -> bool {
        !matches!(&self, MalType::Nil | MalType::Bool(false))
    }
}
