mod parser;
mod expression_builder;

fn main() {

    let mock_data = "var abc = \"123\";\nabc=222;";
    let p = parser::Parser::new(mock_data.to_string());
    let tokens = p.parse();
    println!("{:?}", tokens);

    let mut exp_builder = expression_builder::ExpressionBuilder::new(tokens);
    let statements = exp_builder.run();

}
