mod languages;

use languages::typescript;

fn main() {
    let ast = typescript::ast::build_ast("  const a = 1;");

    println!("{:?}", ast);
}
