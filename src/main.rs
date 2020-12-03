use env::Env;
use std::{collections::HashMap, rc::Rc};
use types::{MalFn, MalType};

#[macro_use]
extern crate lazy_static;

mod env;
mod reader;
mod types;

fn read(input: &str) -> MalType {
    reader::read_str(input)
}

fn eval_ast(expr: MalType, env: Rc<Env>) -> MalType {
    match expr {
        MalType::Symbol(name) => match env.find(name.as_str()) {
            Some(v) => v,
            None => MalType::Nil,
        },

        MalType::List(v) => {
            let ev = v.into_iter().map(|x| eval(x, Rc::clone(&env))).collect();
            MalType::List(ev)
        }

        _ => expr,
    }
}

fn eval(expr: MalType, env: Rc<Env>) -> MalType {
    match &expr {
        MalType::List(list) => {
            if list.is_empty() {
                expr
            } else {
                let res = eval_ast(expr, env);
                match res {
                    MalType::List(v) => match v.first() {
                        Some(MalType::BuiltinFn(func)) => func(v.into_iter().skip(1).collect()),
                        None | Some(_) => {
                            panic!("expected first element to be a function");
                        }
                    },
                    _ => {
                        panic!("expected list");
                    }
                }
            }
        }

        x => eval_ast(x.to_owned(), env),
    }
}

fn print(expr: MalType) -> String {
    expr.to_string()
}

fn rep(input: String, env: Rc<Env>) -> String {
    print(eval(read(input.as_str()), env))
}

fn builtin_add(args: Vec<MalType>) -> MalType {
    fn to_f64(v: &MalType) -> f64 {
        match v {
            MalType::Number(x) => *x,
            _ => panic!("Expected number to add"),
        }
    }
    let sum = args.iter().map(|x| to_f64(x)).fold(0.0, |acc, v| acc + v);

    MalType::Number(sum)
}

fn builtin_mult(args: Vec<MalType>) -> MalType {
    fn to_f64(v: &MalType) -> f64 {
        match v {
            MalType::Number(x) => *x,
            _ => panic!("Expected number to multiply"),
        }
    }
    let sum = args.iter().map(|x| to_f64(x)).fold(1.0, |acc, v| acc * v);

    MalType::Number(sum)
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    let env = Rc::new(env::Env::new(None));
    println!("new env: {:?}", env);

    env.set("+", MalType::BuiltinFn(builtin_add));
    env.set("*", MalType::BuiltinFn(builtin_mult));

    loop {
        let line_result = rl.readline("user> ");
        match line_result {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("<{}>", rep(line, Rc::clone(&env)));
            }

            Err(rustyline::error::ReadlineError::Eof) => {
                println!("Xau!");
                break;
            }

            Err(rustyline::error::ReadlineError::Interrupted) => {
                continue;
            }

            Err(e) => panic!("Unhandled error {:?}", e),
        }
    }
}
