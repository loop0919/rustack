use std::collections::HashMap;

struct Vm<'src> {
    stack: Vec<Value<'src>>,
    vars: HashMap<&'src str, Value<'src>>,
}

impl<'src> Vm<'src> {
    fn new() -> Self {
        Self {
            stack: vec![],
            vars: HashMap::new(),
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Value<'src> {
    Num(i64),
    Op(&'src str),
    Sym(&'src str),
    Block(Vec<Value<'src>>),
}

impl<'src> Value<'src> {
    fn as_num(&self) -> i64 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("Value is not a number"),
        }
    }

    fn to_block(self) -> Vec<Value<'src>> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a block"),
        }
    }
}


fn main() {
    for line in std::io::stdin().lines() {
        if let Ok(line) = line {
            let stack = parse(&line);
            println!("stack: {stack:?}");
        }
    }
}

fn parse<'a>(line: &'a str) -> Vec<Value> {
    let mut vm = Vm::new();
    let input: Vec<_> = line.split(" ").collect();
    let mut words = &input[..];

    while let Some((&word, mut rest)) = &words.split_first() {
        if word.is_empty() {
            break;
        }
        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            vm.stack.push(value);
        } else {
            let code = if let Ok(num) = word.parse::<i64>() {
                Value::Num(num)
            } else if word.starts_with("$") { 
                Value::Sym(&word[1..])
            }else {
                Value::Op(word)
            };

            eval(code, &mut vm.stack);
        }
        words = rest;
    }
    vm.stack
}

fn parse_block<'src, 'a>(input: &'a [&'src str]) -> (Value<'src>, &'a [&'src str]) {
    let mut tokens = vec![];
    let mut words = input;

    while let Some((&word, mut rest)) = words.split_first() {
        if word.is_empty() {
            break;
        }

        if word == "{" {
            let value;
            (value, rest) = parse_block(rest);
            tokens.push(value);
        } else if word == "}" {
            return (Value::Block(tokens), rest);
        } else if let Ok(value) = word.parse::<i64>() {
            tokens.push(Value::Num(value));
        } else {
            tokens.push(Value::Op(word));
        }

        words = rest;
    }

    (Value::Block(tokens), words)
}

fn eval<'src>(code: Value<'src>, vm: &mut Vm<'src>) {
    match code {
        Value::Op(op) => match op {
            "+" => add(&mut vm.stack),
            "-" => sub(&mut vm.stack),
            "*" => mul(&mut vm.stack),
            "/" => div(&mut vm.stack),
            "if" => op_if(&mut vm.stack),
            _ => panic!("{op:?} could not be parsed"),
        }
        _ => vm.stack.push(code),
    }
}

fn add(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs + rhs));
}

fn sub(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs - rhs));
}

fn mul(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs * rhs));
}

fn div(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    stack.push(Value::Num(lhs / rhs));
}

fn def(stack: &mut Vec<Value>) {
    let lhs = stack.pop().unwrap().as_num();
    let rhs = stack.pop().unwrap().as_num();
    
}

fn op_if(stack: &mut Vec<Value>) {
    let false_branch = stack.pop().unwrap().to_block();
    let true_branch = stack.pop().unwrap().to_block();
    let cond = stack.pop().unwrap().to_block();

    for code in cond {
        eval(code, stack);
    }

    let cond_result = stack.pop().unwrap().as_num();

    if cond_result != 0 {
        for code in true_branch {
            eval(code, stack);
        }
    } else {
        for code in false_branch {
            eval(code, stack);
        }
    }
}


#[cfg(test)]
mod test {
    use super::{parse, Value::*};

    #[test]
    fn test_if_false() {
        assert_eq!(
            parse("{ 6 2 3 * - } { 100 } { -100 } if"),
            vec![Num(-100)]
        );
    }

    #[test]
    fn test_if_true() {
        assert_eq!(
            parse("{ 1 1 + } { 100 } { -100 } if"),
            vec![Num(100)]
        );
    }
}
