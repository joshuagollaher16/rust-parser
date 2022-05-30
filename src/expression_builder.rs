use crate::parser::Token;

//Data types of our variables
#[derive(Clone, PartialEq)]
pub enum VariableType {
    Str(String),
    Number(f32)
}

//Represents a variable
#[derive(Clone, PartialEq)]
pub struct Variable {
    variable_type: VariableType
}
impl Variable {
    pub fn new(variable_type: VariableType) -> Self {
        Variable {
            variable_type
        }
    }
}

//A static value
#[derive(Clone, PartialEq)]
pub enum Literal {
    Str(String),
    Number(f32)
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
    Divide
}

#[derive(Clone, PartialEq)]
pub enum StatementType {
    VariableAssignment(Variable, Value), // Variable, new value
    BinaryOp(Value, OperationType, Value), //An operation between two values
    FunctionCall(String, Vec<Value>), // A function call with a vector of parameters
    FunctionDeclaration(String, Vec<Variable>, Vec<StatementType>), // A function declaration with a vec of parameter and a vec of the body
    Return(Literal) // We will only support returning literals to start
}

pub struct ExpressionBuilder {
    tokens: Vec<Token>,
}

impl ExpressionBuilder {
    pub fn new(tokens: Vec<Token>) -> Self {
        ExpressionBuilder {
            tokens: tokens
        }
    }

    pub fn separate_tokens(&mut self) {



        for t in &mut self.tokens {

        }

    }

    pub fn run(&mut self) -> Vec<StatementType> {
        self.separate_tokens();
        vec![]
    }
}