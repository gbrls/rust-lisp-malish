use crate::types::*;
use std::collections::HashMap;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Env {
    data: RefCell<HashMap<String, MalType>>,
    outer: Option<Rc<Env>>,
}

impl Env {
    pub fn new(outer: Option<Rc<Env>>, binds: Vec<(String, MalType)>) -> Env {
        let env = Env {
            data: RefCell::new(HashMap::new()),
            outer,
        };

        for (v, e) in binds {
            env.set(v.as_ref(), e);
        }

        env
    }

    pub fn find(&self, key: &str) -> Option<MalType> {
        match self.data.borrow_mut().get(key) {
            Some(x) => Some(x.to_owned()),
            None => match &self.outer {
                Some(out_env) => out_env.find(key),
                None => None,
            },
        }
    }

    pub fn set(&self, key: &str, val: MalType) {
        self.data.borrow_mut().insert(key.to_owned(), val);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find() {
        let outer_env = Rc::new(Env::new(None, Vec::new()));
        let inner_env = Rc::new(Env::new(Some(Rc::clone(&outer_env)), Vec::new()));

        inner_env.set("a", MalType::Number(1.0));
        outer_env.set("b", MalType::Number(2.0));

        println!("{:?}", inner_env.find("a"));
        println!("{:?}", inner_env.find("b"));
    }
}
