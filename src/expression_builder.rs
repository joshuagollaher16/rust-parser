use std::collections::HashMap;
use std::iter::Map;
use crate::parser::Token;

//Data types of our variables
#[derive(Clone, PartialEq)]
pub enum VariableType {
    Str(String),
    Number(f32),
    Bool(bool)
}

//Represents a variable
#[derive(Clone, PartialEq)]
pub struct Variable {
    value: Literal
}
impl Variable {
    pub fn new(variable_type: Literal) -> Self {
        Variable {
            value
        }
    }
}

//A static value
#[derive(Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Number(f32),
    Bool(bool)
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Variable,
    Literal
}

#[derive(Clone, PartialEq)]
pub enum OperationType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal
}

#[derive(Clone, PartialEq)]
pub struct Scope {
    statement: Vec<StatementType>
}

#[derive(Clone, PartialEq)]
pub enum StatementType {
    Scope(Vec<StatementType>),
    VariableAssignment(Variable, Value), // Variable, new value
    BinaryOp(Value, OperationType, Value), //An operation between two values
    FunctionCall(String, Vec<Value>), // A function call with a vector of parameters
    FunctionDeclaration(String, Vec<Variable>, Scope), // A function declaration with a vec of parameter and a vec of the body
    Return(Literal), // We will only support returning literals to start
    None
}

pub struct VM {
    variables: HashMap<String, Variable>
}

impl VM {
    fn new() -> VM {
        //Predefine builtin functions
        VM { variables: HashMap::new() }
    }
}

pub struct ExpressionBuilder {
    tokens: Vec<Token>,
    vm: VM
}

impl ExpressionBuilder {
    pub fn new(tokens: Vec<Token>) -> Self {
        ExpressionBuilder {
            tokens,
            vm: VM::new()
        }
    }

    fn split_tokens(&self) -> Vec<Vec<Token>> {

        let mut output = self.tokens.clone().into_iter().fold(Vec::new(), |mut acc, tok| {
            if tok == Token::Semicolon || acc.is_empty() {
                acc.push(Vec::new());
            }
            if tok == Token::Semicolon { return acc; }
            acc.last_mut().unwrap().push(tok);
            acc
        });

        output = output.into_iter().filter(|x| {
            x.len() > 0
        }).collect();

        output
    }

    fn parse_expression(&self, tokens: &Vec<Token>) -> Literal {

        let mut accumulator = Literal::Str(String::from(""));

        for (i, tok) in tokens.iter().enumerate() {

            //Find the type of the first element
            if i == 0 {

                match tok {
                    Token::Alpha(str) => {
                        //Find type of variable
                        accumulator = self.vm.variables[str].value.clone();
                    },
                    Token::StringLiteral(s) => {
                        accumulator = Literal::Str(s.clone());
                    },
                    Token::Numeric(num) => {
                        accumulator = Literal::Number(*num);
                    },
                    _ => {panic!();}
                }

                continue;
            }

            //Combine the rest, one after another (this doesn't work with order of operations yet)

            todo!();

        }

        accumulator
    }

    fn handle_variable_declaration(&self, vec: &Vec<Token>) -> StatementType {

        let variable_name: String;

        if let Token::Alpha(s) = &vec[1] {
            variable_name = s.clone();
        } else {panic!();}

        StatementType::None
    }
    fn handle_variable_assignment(&self, vec: &Vec<Token>) -> StatementType {
        StatementType::None
    }
    fn handle_function_declaration(&self, vec: &Vec<Token>) -> StatementType {
        StatementType::None
    }
    fn handle_function_call(&self, vec: &Vec<Token>) -> StatementType {
        StatementType::None
    }

    pub fn run(&mut self) -> Vec<StatementType> {

        let token_list = self.split_tokens();

        println!("{:?}", token_list);

        for line in &token_list {

            match &line[0] {

                //Probably variable assignment
                Token::Alpha(s) => {

                    match &line[1] {

                        Token::Equals => {
                            self.handle_variable_assignment(line);
                        }

                        _ => {}
                    }

                },

                Token::Var => {
                    self.handle_variable_declaration(line);
                },

                Token::Call => {
                    self.handle_function_call(line);
                },

                _ => {
                    panic!("Unrecognized token");
                }
            }

        }

        vec![]
    }
}