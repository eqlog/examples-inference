use eqlog_runtime::eqlog_mod;
use lalrpop_util::lalrpop_mod;
eqlog_mod!(program);
mod grammar_util;
lalrpop_mod!(grammar);
#[cfg(test)]
mod binding_test;
mod error;
#[cfg(test)]
mod grammar_test;

use crate::error::LanguageError;
use crate::grammar::ModuleParser;
use crate::grammar_util::{erase_comments, Literals};
use crate::program::*;
use std::env;
use std::fs;
use std::process::ExitCode;

fn has_undeclared_variables(p: &Program) -> bool {
    p.iter_variable_expr_node()
        .any(|(expr, var)| p.var_type_in_expr(var, expr).is_none())
}

fn check_source(src: &str) -> Result<(Program, Literals, ModuleNode), LanguageError> {
    let no_comments_src = erase_comments(src);

    let mut p = Program::new();
    let mut lits = Literals::new();

    let module = ModuleParser::new()
        .parse(&mut p, &mut lits, &no_comments_src)
        .map_err(|err| LanguageError::from_parse_error(err, &no_comments_src))?;

    p.close();

    if p.variable_shadowing() {
        return Err(LanguageError::VariableShadowing);
    }

    if has_undeclared_variables(&p) {
        return Err(LanguageError::UndeclaredVariable);
    }

    if p.conflicting_types() {
        return Err(LanguageError::ConflictingTypes);
    }

    // TODO: At the moment, almost all `Type` elements are undetermined because we haven't encoded
    // the typing rules yet. We'll enable this check after adding those rules.
    //
    // if p.iter_type().any(|sigma| !p.determined_type(sigma)) {
    //     return Err(LanguageError::UndeterminedType);
    // }

    Ok((p, lits, module))
}

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

    let src: String = match fs::read_to_string(&file_name) {
        Ok(contents) => contents,
        Err(err) => {
            eprintln!("Error reading file {file_name}: {err}");
            return ExitCode::FAILURE;
        }
    };

    match check_source(&src) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        }
    };

    ExitCode::SUCCESS
}
