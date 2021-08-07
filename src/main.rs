use std::collections::HashMap;

enum Command {
    SetVar(String, Value),
    GetVar(String),
    PushVar(String),
    Push(Value),
    Pop,
    Add,
}

#[derive(Clone, PartialEq, Debug)]
enum Value {
    Nothing,
    Int(i64),
    String(String),
}

#[derive(Clone, PartialEq, Debug)]
enum Type {
    Int,
    String,
    Nothing,
}

#[derive(Debug)]
enum EngineError {
    MismatchNumParams,
    MimatchType,
    UnknownCommand(String),
    MissingVariable(String),
    EmptyStack,
}

struct Evaluator {
    vars: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl Evaluator {
    fn new() -> Evaluator {
        Self {
            vars: HashMap::new(),
            stack: vec![],
        }
    }

    fn pop(&mut self) -> Result<Value, EngineError> {
        let result = self.stack.pop();
        match result {
            Some(v) => Ok(v),
            None => return Err(EngineError::EmptyStack),
        }
    }

    fn add(&self, lhs: Value, rhs: Value) -> Result<Value, EngineError> {
        match (lhs, rhs) {
            (Value::Int(i1), Value::Int(i2)) => Ok(Value::Int(i1 + i2)),
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1 + &s2)),
            _ => Err(EngineError::MimatchType),
        }
    }

    fn evaluate(&mut self, commands: &[Command]) -> Result<Value, EngineError> {
        let mut output = Ok(Value::Nothing);
        for command in commands {
            match command {
                Command::SetVar(name, value) => {
                    self.vars.insert(name.into(), value.clone());
                }
                Command::GetVar(name) => match self.vars.get(name) {
                    Some(value) => output = Ok(value.clone()),
                    None => return Err(EngineError::MissingVariable(name.into())),
                },
                Command::PushVar(name) => match self.vars.get(name) {
                    Some(value) => self.stack.push(value.clone()),
                    None => return Err(EngineError::MissingVariable(name.into())),
                },
                Command::Push(v) => self.stack.push(v.clone()),
                Command::Pop => {
                    output = self.pop();
                }
                Command::Add => {
                    let lhs = self.pop()?;
                    let rhs = self.pop()?;

                    let result = self.add(lhs, rhs)?;
                    self.stack.push(result)
                }
            }
        }
        output
    }
}

fn parse_var_name(var_name: &str) -> Result<String, EngineError> {
    Ok(var_name.into())
}

fn parse_string(val: &str) -> Result<Value, EngineError> {
    if val.starts_with('\"') && val.ends_with('\"') && val.len() > 1 {
        let inner = val[1..(val.len() - 1)].to_string();

        Ok(Value::String(inner))
    } else {
        Err(EngineError::MimatchType)
    }
}

fn parse_int(val: &str) -> Result<Value, EngineError> {
    let result = val.parse::<i64>();

    match result {
        Ok(x) => Ok(Value::Int(x)),
        _ => Err(EngineError::MimatchType),
    }
}

fn parse_value(val: &str) -> Result<Value, EngineError> {
    if val.starts_with("\"") && val.ends_with("\"") && val.len() > 1 {
        // Parse the string
        parse_string(val)
    } else {
        // Parse the number
        parse_int(val)
    }
}

fn parse_set(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() != 3 {
        return Err(EngineError::MismatchNumParams);
    }

    let var_name = parse_var_name(input[1])?;
    let value = parse_value(input[2])?;

    Ok(Command::SetVar(var_name, value))
}

fn parse_get(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() != 2 {
        return Err(EngineError::MismatchNumParams);
    }

    let var_name = parse_var_name(input[1])?;

    Ok(Command::GetVar(var_name))
}

fn parse_pushvar(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() != 2 {
        return Err(EngineError::MismatchNumParams);
    }

    let var_name = parse_var_name(input[1])?;

    Ok(Command::PushVar(var_name))
}

fn parse_push(input: &[&str]) -> Result<Command, EngineError> {
    if input.len() != 2 {
        return Err(EngineError::MismatchNumParams);
    }

    let val = parse_value(input[1])?;

    Ok(Command::Push(val))
}

fn parse(input: &str) -> Result<Vec<Command>, EngineError> {
    // set a 100
    // get a

    let mut output = vec![];

    for line in input.lines() {
        let command: Vec<_> = line.split_ascii_whitespace().collect();

        match command.get(0) {
            Some(x) if *x == "set" => {
                output.push(parse_set(&command)?);
            }
            Some(x) if *x == "get" => {
                output.push(parse_get(&command)?);
            }
            Some(x) if *x == "push" => {
                output.push(parse_push(&command)?);
            }
            Some(x) if *x == "pushvar" => {
                output.push(parse_pushvar(&command)?);
            }
            Some(x) if *x == "pop" => {
                output.push(Command::Pop);
            }
            Some(x) if *x == "add" => {
                output.push(Command::Add);
            }
            Some(name) => return Err(EngineError::UnknownCommand(name.to_string())),
            None => {}
        }
    }

    Ok(output)
}

struct Typechecker {
    stack: Vec<Type>,
}

impl Typechecker {
    fn typecheck_command(&mut self, commands: &Command) -> Result<Type, EngineError> {
        Ok(Type::Nothing)
    }

    fn typecheck(&mut self, commands: &[Command]) -> Result<Type, EngineError> {
        for command in commands {
            self.typecheck_command(command)?;
        }
        Ok(Type::Nothing)
    }
}

#[test]
fn test1() -> Result<(), EngineError> {
    let commands = vec![
        Command::SetVar("a".into(), Value::Int(100)),
        Command::GetVar("a".into()),
    ];

    let mut evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(100));

    Ok(())
}

#[test]
fn eval_set_get() -> Result<(), EngineError> {
    let input = "set x 30\nget x";

    let commands = parse(input)?;

    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(30));

    Ok(())
}

#[test]
fn eval_set_get_string() -> Result<(), EngineError> {
    let input = "set x \"hello\"\nget x";

    let commands = parse(input)?;

    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::String("hello".into()));

    Ok(())
}

#[test]
fn eval_stack() -> Result<(), EngineError> {
    let input = "push 100\npush 30\nadd\npop";

    let commands = parse(input)?;

    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(130));

    Ok(())
}

#[test]
fn eval_pushvar() -> Result<(), EngineError> {
    let input = "set x 33\npushvar x\npush 100\nadd\npop";

    let commands = parse(input)?;

    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(133));

    Ok(())
}

fn main() -> Result<(), EngineError> {
    for arg in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(arg).unwrap();
        let mut engine = Evaluator::new();
        let commands = parse(&contents)?;
        let answer = engine.evaluate(&commands)?;

        println!("{:?}", answer);
    }

    Ok(())
}
