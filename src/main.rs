mod languages;

use languages::typescript;

fn main() {
    let tokens = typescript::tokenizer::tokenize("Hello, world!");

    let ast = typescript::ast::build_ast();

    println!("{:?}", tokens);

    println!("{:?}", ast);
}
