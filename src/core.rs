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

pub fn namespace() -> Vec<(String, MalFn)> {
    vec![
        (String::from("+"), add),
        (String::from("*"), mult),
        (String::from("prn"), prn),
        (String::from("list"), list),
        (String::from("list?"), is_list),
        (String::from("empty?"), is_empty),
        (String::from("count"), count),
        (String::from("="), equals),
        (String::from("<"), less),
        (String::from("<="), less_equals),
        (String::from(">"), greater),
        (String::from(">="), greater_equals),
    ]
}
