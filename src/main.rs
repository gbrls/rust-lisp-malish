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
                let special_symbol = match list.first() {
                    Some(MalType::Symbol(sym)) => match sym.as_ref() {
                        "def!" => {
                            if let MalType::Symbol(a_name) = &list[1] {
                                env.set(a_name.as_ref(), eval(list[2].to_owned(), Rc::clone(&env)));
                            } else {
                                panic!("Expected symbol for def!, received {:?}", &list[1])
                            }
                            Some(MalType::Nil)
                        }

                        "let*" => {
                            if let MalType::List(bindings) = &list[1] {
                                let inner_env =
                                    Rc::new(Env::new(Some(Rc::clone(&env)), Vec::new()));
                                for binding in bindings {
                                    if let MalType::List(list) = binding {
                                        if let [MalType::Symbol(k), v] = &list[..] {
                                            inner_env
                                                .set(k, eval(v.to_owned(), Rc::clone(&inner_env)));
                                        }
                                    } else {
                                        panic!("Expected a list for binding, found {:?}", binding);
                                    }
                                }

                                Some(eval(list[2].to_owned(), Rc::clone(&inner_env)))
                            } else {
                                panic!("Expected list of bindings, found {:?}", &list[1]);
                            }
                        }

                        "do" => {
                            //TODO: check if this is right
                            let ev: Vec<MalType> = list
                                .iter()
                                .skip(1)
                                .map(|x| eval(x.to_owned(), Rc::clone(&env))) //TODO: maybe eval_ast here?
                                .collect();

                            let e = ev.last().unwrap();
                            Some(e.to_owned())
                        }

                        "if" => {
                            if list[1].to_bool() {
                                Some(eval((&list[2]).to_owned(), Rc::clone(&env)))
                            } else {
                                Some(eval((&list[3]).to_owned(), Rc::clone(&env)))
                            }
                        }

                        "fn*" => {
                            if let MalType::List(binds) = &list[1] {
                                let nenv = Rc::new(Env::new(Some(Rc::clone(&env)), Vec::new()));

                                let closure = |args: Vec<MalType>| {
                                    println!("{:?}", args);
                                    MalType::Nil
                                };

                                Some(MalType::BuiltinFn(closure))
                            } else {
                                panic!("Expected argument list, found {:?}", &list[1])
                            }
                        }

                        _ => None,
                    },
                    _ => None,
                };

                // we first test if the typed expression is a builtin macro
                if let Some(e) = special_symbol {
                    e
                } else {
                    match eval_ast(expr, env) {
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

    let env = Rc::new(env::Env::new(None, Vec::new()));
    println!("new env: {:?}", env);

    env.set("+", MalType::BuiltinFn(builtin_add));
    env.set("*", MalType::BuiltinFn(builtin_mult));

    loop {
        let line_result = rl.readline("user> ");
        match line_result {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", rep(line, Rc::clone(&env)));
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
