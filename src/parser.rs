

#[derive(Debug, PartialEq)]
pub enum Token {
    Alpha(String),
    StringLiteral(String),
    Numeric(f32),
    LeftParen,
    RightParen,
    Operator(char),
    Semicolon,
    Equals,
    Empty,
}

pub struct Parser {
    data: String
}

impl Parser {
    pub fn new(data: String) -> Parser {
        Parser { data }
    }

    pub fn parse(&self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();
        
        let mut i = 0;
        let mut len = self.data.len();

        //Lots of helper functions
        let peek = |i: usize| -> char {
            self.data.chars().nth(i).unwrap()
        };
        let consume = |x:&mut usize| -> char {
            println!("consume at {} returning {}", *x, self.data.chars().nth(*x).unwrap_or('?'));
            let c = self.data.chars().nth(*x).unwrap();
            *x += 1;
            c
        };
        let consume_whitespace = || {
            while peek(i) == ' ' {
                consume(&mut i);
            }
        };
        let is_alpha = |s: &String| -> bool {
            for ch in s.chars() {
                if !ch.is_alphabetic() {return false;}
            }
            true
        };
        let is_numeric = |s: &String| -> bool {
            for ch in s.chars() {
                if ch.is_alphabetic() {return false;}
            }
            true
        };
        let is_newline = |c: char| -> bool {
            c == '\n'
        };
        let is_whitespace = |c: char| -> bool {
            c == ' ' || c == '\n'
        };

        let choose_token = |s: &String| -> Token {

            if *s == String::from(" ") || s.len() == 0 { return Token::Empty; }

            if is_alpha(s) { return Token::Alpha(s.clone()); }
            else if is_numeric(s) { return Token::Numeric(s.parse().unwrap()); }
            panic!();
        };



        let mut is_first_iter = true;
        let mut current_string = String::new();
        let mut inside_string_literal = false;

        loop {

            let mut was_oneliner = false; // Signifies we don't need to append to the string / the char is already tokenized

            let new_char = consume(&mut i);

            //If we are inside a string literal, add the current char regardless of type
            if inside_string_literal && new_char != '"' {
                current_string.push(new_char);
                continue;
            }
            if new_char == '"' {
                println!("Starting paren");
                if inside_string_literal {
                    //end it
                    inside_string_literal = false;
                    tokens.push(Token::StringLiteral(current_string.clone()));
                    current_string.clear();
                    continue;

                } else {
                    //start it

                    inside_string_literal = true;
                    continue;

                }
            }
            else if is_whitespace(new_char) { //Whitespace marks a break -> Abcde fgh = [abcde, fgh]
                tokens.push(choose_token(&current_string));
                current_string.clear();
                continue;
            }
            else if new_char == '(' { //Add individual characters: (, ), =, ;
                //make left paren token
                println!("{}", current_string);
                if !is_first_iter {tokens.push(choose_token(&current_string));}
                tokens.push(Token::LeftParen);

                current_string.clear();

                was_oneliner = true;
            }
            else if new_char == ')' {
                //make right paren token
                println!("{}", current_string);
                if !is_first_iter {tokens.push(choose_token(&current_string));}
                tokens.push(Token::RightParen);

                current_string.clear();

                was_oneliner = true;
            }
            else if new_char == ';' {
                //make semicolon token
                println!("{}", current_string);
                if !is_first_iter {tokens.push(choose_token(&current_string));}
                tokens.push(Token::Semicolon);

                current_string.clear();

                was_oneliner = true;
            }
            else if new_char == '=' {
                //make equals token
                println!("{}", current_string);
                if !is_first_iter {tokens.push(choose_token(&current_string));}
                tokens.push(Token::Equals);

                current_string.clear();

                was_oneliner = true;
            }
            else if new_char == '+' || new_char == '-' || new_char == '*' || new_char == '/' {
                //make operator token
                println!("{}", current_string);
                if !is_first_iter {tokens.push(choose_token(&current_string));}
                tokens.push(Token::Operator(new_char));

                current_string.clear();

                was_oneliner = true;
            }
            //Avoid putting a char into the string when it's already been handled
            else if !was_oneliner {
                println!("{}", current_string);
                current_string.push(new_char);
            }
            if is_first_iter {
                is_first_iter = false;
            }

            //Stop when at EOF
            if i >= len {
                break;
            }
        }
        //Remove empty tokens
        tokens.into_iter().filter(|x| *x != Token::Empty).collect()
    }
}
