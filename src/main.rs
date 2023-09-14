use ansi_term::Color;
use solang::sema::ast::{Expression, Function, Namespace, Statement};
use solang::sema::Recurse;
use solang::{file_resolver::FileResolver, parse_and_resolve, Target};
use solang_parser::pt::CodeLocation;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;

fn parse_filename(filename: &String) -> (Namespace, FileResolver, Option<Arc<str>>) {
    let mut resolver = FileResolver::default();

    resolver.add_import_path(&PathBuf::from("."));
    let target = Target::EVM;
    let ns = parse_and_resolve(OsStr::new(filename), &mut resolver, target);

    let fileno = 0;
    if let Some(source) = resolver.get_contents_of_file_no(fileno) {
        return (ns, resolver, Some(source.clone()));
    }
    (ns, resolver, None)
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
        | Expression::BitwiseAnd { left, right, .. }
        | Expression::BitwiseOr { left, right, .. } => {
            let left_end = left.loc().end();
            let right_start = right.loc().start();

            if left_end >= right_start {
                error(format!(
                    "start: {} >= end {} in expr {}",
                    left_end,
                    right_start,
                    &source[expr.loc().start()..expr.loc().end()],
                ));
                println!(
                    "    left: {}",
                    &source[left.loc().start()..left.loc().end()]
                );
                println!(
                    "    right: {}",
                    &source[right.loc().start()..right.loc().end()]
                );
            } else {
                success();
            }
        }
        _ => (),
    }
    true
}

fn print_issue_banner(issue_num: usize, url: &str) {
    println!(
        "\n{}: {}\n",
        Color::Blue.bold().paint(format!("Issue {}", issue_num)),
        Color::White.bold().paint(url)
    )
}

fn error(msg: String) {
    println!(
        "{}: {}",
        Color::Red.bold().paint("Error"),
        Color::White.bold().paint(msg)
    )
}

fn success() {
    println!("{}", Color::Green.bold().paint("Success"))
}

/// a += b loc error
fn issue1521() {
    print_issue_banner(1521, "https://github.com/hyperledger/solang/issues/1521");
    let (ns, _, source) = parse_filename(&"issue1521.sol".to_string());
    if let Some(source) = source {
        ns.functions
            .iter()
            .for_each(|f| visit_function_body(f, &mut source.clone()));
    }
}

/// type(int256).min
fn issue1522() {
    print_issue_banner(1522, "TODO");
    let (ns, cache, _source) = parse_filename(&"issue1522.sol".to_string());
    if ns.diagnostics.any_errors() {
        error("issue1522.sol".to_string());
        println!("Solang diagnostics:");
        ns.print_diagnostics(&cache, false);
    } else {
        success();
    }
}

fn main() {
    issue1521();
    issue1522();
}
