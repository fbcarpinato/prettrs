mod languages;

use languages::typescript;

fn main() {
    let ast = typescript::ast::build_ast(
        r#"
        const asd  = 213;
        
        if (asd === 213) {
            console.log("asd");
        }
        "#
        .to_string(),
    );

    println!("{:?}", ast);
}
