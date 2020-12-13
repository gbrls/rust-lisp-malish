use env::Env;
use std::rc::Rc;
use types::MalType;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate colour;

mod core;
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
    match expr.to_owned() {
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
                            let ev: Vec<MalType> = list
                                .iter()
                                .skip(1)
                                .map(|x| eval(x.to_owned(), Rc::clone(&env))) //TODO: maybe eval_ast here? (Check if this is right)
                                .collect();

                            let e = ev.last().unwrap();
                            Some(e.to_owned())
                        }

                        "if" => {
                            if eval(list[1].to_owned(), Rc::clone(&env)).to_bool() {
                                Some(eval((&list[2]).to_owned(), Rc::clone(&env)))
                            } else {
                                Some(eval((&list[3]).to_owned(), Rc::clone(&env)))
                            }
                        }

                        "fn*" => match list[1].to_owned() {
                            MalType::List(binds) => {
                                let env_cpy = Rc::clone(&env);
                                let closure = move |args: Vec<MalType>| {
                                    let mut new_binds: Vec<(String, MalType)> = Vec::new();

                                    for (k, v) in binds.iter().zip(args.iter()) {
                                        if let MalType::Symbol(sym) = k {
                                            new_binds.push((sym.to_owned(), v.to_owned()));
                                        } else {
                                            panic!(
                                                "cant pass {:?} as a symbol in function arguments",
                                                k
                                            );
                                        }
                                    }

                                    let nenv =
                                        Rc::new(Env::new(Some(Rc::clone(&env_cpy)), new_binds));

                                    let fn_expr = list[2].to_owned();

                                    eval(fn_expr, Rc::clone(&nenv))
                                };

                                Some(MalType::UserFn(Rc::new(closure)))
                            }
                            _ => panic!("Expected argument list, found {:?}", &list[1]),
                        },

                        "eval" => {
                            // The semantics of this expression are a little bit tricky.
                            // first we evaluate the argment in the current enviroment.
                            // then, we evaluate the expression in the REPL enviroment.
                            let expr = eval(list[1].to_owned(), Rc::clone(&env));
                            let mut toplevel = &env;
                            while let Some(e) = &toplevel.outer {
                                toplevel = e;
                            }
                            Some(eval(expr, Rc::clone(&toplevel)))
                        }

                        _ => None,
                    },
                    _ => None,
                };

                // we first test if the typed expression is a builtin macro
                if let Some(e) = special_symbol {
                    e
                } else {
                    match eval_ast(expr, Rc::clone(&env)) {
                        MalType::List(v) => match v.first() {
                            Some(MalType::BuiltinFn(func)) => {
                                func(v.clone().into_iter().skip(1).collect())
                            }
                            Some(MalType::UserFn(func)) => {
                                func(v.clone().into_iter().skip(1).collect())
                            }
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

        x => eval_ast(x, env),
    }
}

fn print(expr: MalType) -> String {
    expr.to_string()
}

fn rep(input: String, env: Rc<Env>) -> String {
    print(eval(read(input.as_str()), env))
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    let env = Rc::new(env::Env::new(None, Vec::new()));

    // adicionando a função not
    rep(
        "(def! not (fn* (a) (if a false true)))".to_owned(),
        Rc::clone(&env),
    );

    // função para carregar arquivos
    rep(
        "(def! load-file (fn* (f) (eval (read-string (str \"(do\" (slurp f)\"nil\"\")\"))))))"
            .to_owned(),
        Rc::clone(&env),
    );

    // loading builtin functions in the outermost enviroment
    for (name, fun) in core::namespace() {
        env.set(name.as_ref(), MalType::BuiltinFn(fun));
    }

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
