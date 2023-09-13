use solang::sema::ast::{Expression, Function, Statement};
use solang::sema::Recurse;
use solang::{file_resolver::FileResolver, parse_and_resolve, Target};
use solang_parser::pt::CodeLocation;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;

fn parse_filename(filename: &String) {
    let mut resolver = FileResolver::default();

    resolver.add_import_path(&PathBuf::from("."));
    let target = Target::EVM;
    let ns = parse_and_resolve(&OsStr::new(filename), &mut resolver, target);

    let fileno = 0;
    if let Some(source) = resolver.get_contents_of_file_no(fileno) {
        for c in &ns.contracts {
            for f_idx in c.functions.iter() {
                let f = ns.functions.get(*f_idx).unwrap();
                visit_function_body(f, &mut source.clone());
            }
        }
    }
}

pub fn visit_function_body(function: &Function, source: &mut Arc<str>) {
    println!("   === Function: {} ===\n", function.signature);
    visit_statements(&function.body, source)
}

pub fn visit_statements(statements: &[Statement], source: &mut Arc<str>) {
    for statement in statements {
        if let Statement::Expression(_, _, expr) = statement {
            expr.recurse(source, print_arith_op_info)
        }
    }
}

pub fn print_arith_op_info(expr: &Expression, source: &mut Arc<str>) -> bool {
    match expr {
        Expression::Add { left, right, .. }
        | Expression::Subtract { left, right, .. }
        | Expression::Multiply { left, right, .. }
        | Expression::Modulo { left, right, .. }
        | Expression::Divide { left, right, .. }
        | Expression::And { left, right, .. }
        | Expression::Or { left, right, .. } => {
            let left_end = left.loc().end();
            let right_start = right.loc().start();

            if left_end >= right_start {
                println!(
                    "Error: start: {} >= end {} in expression {}",
                    left_end,
                    right_start,
                    &source[expr.loc().start()..expr.loc().end()],
                );
                println!(
                    "    left: {}",
                    &source[left.loc().start()..left.loc().end()]
                );
                println!(
                    "    right: {}",
                    &source[right.loc().start()..right.loc().end()]
                );
            } else {
                println!("Success")
            }
            println!("");
        }
        _ => (),
    }
    true
}

fn main() {
    parse_filename(&"bug.sol".to_string());
}
