use std::env;
use tydi_lang_ast::{AstNode, Root, Statement};
use tydi_lang_parser::parse;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let input = &args[0];

    let parse = parse(input);
    for error in &parse.errors {
        eprintln!("{}", error);
    }

    if let Some(root) = Root::cast(parse.syntax()) {
        for statement in root.statements() {
            match statement {
                Statement::TypeDefinition(x) => {
                    println!("type definition: {:?} -> {:?}", x.name(), x.value());
                }
            }
        }
    }
}
