mod parser;
mod expression_builder;

fn main() {

    let mock_data = "abc = \"123\";\ncba=222;";
    let p = parser::Parser::new(mock_data.to_string());
    let tokens = p.parse();
    println!("{:?}", tokens);

    let mut exp_builder = expression_builder::ExpressionBuilder::new(tokens);
    let statements = exp_builder.run();

}
