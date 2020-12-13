use crate::types;

pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn new(input: Vec<String>) -> Reader {
        Reader {
            tokens: input,
            position: 0,
        }
    }

    pub fn next(&mut self) -> &str {
        let t = self.tokens[self.position].as_str();

        self.position += 1;

        t
    }

    pub fn peek(&self) -> &str {
        self.tokens[self.position].as_str()
    }

    pub fn end(&self) -> bool {
        self.position == self.tokens.len()
    }
}

pub fn read_str(input: &str) -> types::MalType {
    let tokens = tokenize(input);
    let mut reader = Reader::new(tokens);

    read_form(&mut reader)
}

fn tokenize(input: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###,
        )
        .unwrap();
    }

    let mut tokens: Vec<String> = Vec::new();

    for item in RE.find_iter(input) {
        let it = item.as_str().trim();

        if !it.starts_with(';') {
            tokens.push(it.to_owned())
        }
    }

    tokens
}

fn read_form(reader: &mut Reader) -> types::MalType {
    if reader.end() {
        types::MalType::Nil
    } else if reader.peek().starts_with('(') {
        reader.next();
        read_list(reader)
    } else {
        read_atom(reader)
    }
}

fn read_list(reader: &mut Reader) -> types::MalType {
    use types::MalType::*;

    let mut list: Vec<types::MalType> = Vec::new();

    while !reader.peek().starts_with(')') {
        list.push(read_form(reader));
    }

    // consume the )
    reader.next();

    List(list)
}

fn read_atom(reader: &mut Reader) -> types::MalType {
    use types::MalType::*;

    let s = reader.next();

    // keywords
    match s {
        "true" => return Bool(true),
        "false" => return Bool(false),
        "nil" => return Nil,
        _ => (),
    }

    if s.starts_with('"') && s.ends_with('"') {
        return Str(s[1..(s.len() - 1)].to_owned());
    }

    // number
    match s.parse::<f64>() {
        Ok(number) => Number(number),
        Err(_) => Symbol(s.to_owned()),
    }
}
