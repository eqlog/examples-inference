use eqlog_runtime::eqlog_mod;
use lalrpop_util::lalrpop_mod;
eqlog_mod!(program);
mod grammar_util;
lalrpop_mod!(grammar);
#[allow(unused)]
mod debugging;

#[allow(unused_imports)]
use crate::debugging::*;
use crate::grammar::ModuleParser;
use crate::grammar_util::Literals;
use crate::program::*;
use std::env;
use std::fs;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args();

    // The first argument is the path to this executable; we ignore it.
    args.next();

    let file_name: String = match args.next() {
        Some(file_name) => file_name,
        None => {
            eprintln!("Usage: ts <FILE_NAME>");
            return ExitCode::FAILURE;
        }
    };

    let contents: String = match fs::read_to_string(&file_name) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file {file_name}: {err}");
            return ExitCode::FAILURE;
        }
    };

    let mut p = Program::new();
    let mut literals = Literals::new();

    match ModuleParser::new().parse(&mut p, &mut literals, &contents) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Syntax error: {err}");
            return ExitCode::FAILURE;
        }
    };

    p.close();

    if p.absurd() {
        eprintln!("Type checking error");
        return ExitCode::FAILURE;
    }

    for (_, var) in p.iter_variable_expr_node() {
        if p.iter_var_type_in_expr()
            .find(|(var0, _, _)| *var0 == var)
            .is_none()
        {
            eprintln!("Usage of Undeclared variable");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
