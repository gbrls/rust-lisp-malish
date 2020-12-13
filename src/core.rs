//TODO: variadic function parameters
//TODO: string functions

use std::vec;

use crate::types::MalType::*;
use crate::types::*;

fn add(args: Vec<MalType>) -> MalType {
    fn to_f64(v: &MalType) -> f64 {
        match v {
            Number(x) => *x,
            _ => panic!("Expected number to add"),
        }
    }
    let sum = args.iter().map(|x| to_f64(x)).fold(0.0, |acc, v| acc + v);

    Number(sum)
}

fn mult(args: Vec<MalType>) -> MalType {
    fn to_f64(v: &MalType) -> f64 {
        match v {
            Number(x) => *x,
            _ => panic!("Expected number to multiply"),
        }
    }
    let sum = args.iter().map(|x| to_f64(x)).fold(1.0, |acc, v| acc * v);

    Number(sum)
}

fn prn(args: Vec<MalType>) -> MalType {
    magenta_ln!("{}", args.first().expect("Expected a parameter to prn"));

    Nil
}

fn list(args: Vec<MalType>) -> MalType {
    List(args)
}

fn is_list(args: Vec<MalType>) -> MalType {
    Bool(matches!(
        args.first().expect("Expected an argument"),
        List(_)
    ))
}

fn is_empty(args: Vec<MalType>) -> MalType {
    match args.first() {
        Some(List(list)) => Bool(matches!(&list[..], [])),
        _ => panic!("Expected list as input"),
    }
}

fn count(args: Vec<MalType>) -> MalType {
    match args.first() {
        Some(List(list)) => Number(list.len() as f64),
        _ => panic!("Expected list as input"),
    }
}

fn equals(args: Vec<MalType>) -> MalType {
    Bool(args[0] == args[1])
}

use std::cmp::Ordering;

fn less(args: Vec<MalType>) -> MalType {
    Bool(matches!(
        args[0].partial_cmp(&args[1]),
        Some(Ordering::Less)
    ))
}

fn less_equals(args: Vec<MalType>) -> MalType {
    Bool(matches!(
        args[0].partial_cmp(&args[1]),
        Some(Ordering::Less) | Some(Ordering::Equal)
    ))
}

fn greater(args: Vec<MalType>) -> MalType {
    Bool(matches!(
        args[0].partial_cmp(&args[1]),
        Some(Ordering::Greater)
    ))
}

fn greater_equals(args: Vec<MalType>) -> MalType {
    Bool(matches!(
        args[0].partial_cmp(&args[1]),
        Some(Ordering::Greater) | Some(Ordering::Equal)
    ))
}

fn read_str(args: Vec<MalType>) -> MalType {
    match args.first() {
        Some(Str(s)) => crate::reader::read_str(s.as_ref()),
        _ => panic!("Expected first argument to read-string to be a string"),
    }
}

fn read_file(args: Vec<MalType>) -> MalType {
    let name = match args.first() {
        Some(Str(s)) => s,
        _ => panic!("expected file as argument!"),
    };
    let contents = std::fs::read_to_string(name).expect("File not found");

    Str(contents)
}

fn many_to_str(args: Vec<MalType>) -> MalType {
    let fmt = args
        .iter()
        .map(|x| format!("{}", x))
        .fold(String::from(""), |acc, x| {
            if !acc.is_empty() {
                format!("{} {}", acc, x)
            } else {
                x
            }
        });

    Str(fmt)
}

pub fn namespace() -> Vec<(&'static str, MalFn)> {
    vec![
        ("+", add),
        ("*", mult),
        ("prn", prn),
        ("list", list),
        ("list?", is_list),
        ("empty?", is_empty),
        ("count", count),
        ("=", equals),
        ("<", less),
        ("<=", less_equals),
        (">", greater),
        (">=", greater_equals),
        ("read-string", read_str),
        ("read-file", read_file),
        ("slurp", read_file),
        ("str", many_to_str),
    ]
}
