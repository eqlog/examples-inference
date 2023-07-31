use std::fmt;

use crate::grammar_util::NeverType;

#[derive(Debug, Clone)]
pub enum LanguageError {
    /// Line and column (both 0-based) of a parse error.
    ParseError { line: usize, column: usize },
    /// A variable is declared more than once for the same scope.
    VariableShadowing,
    /// A variable is used without prior declaration.
    UndeclaredVariable,
}

impl LanguageError {
    pub fn from_parse_error<T>(
        err: lalrpop_util::ParseError<usize, T, NeverType>,
        src: &str,
    ) -> Self {
        use lalrpop_util::ParseError::*;
        let loc: usize = match err {
            InvalidToken { location } => location,
            UnrecognizedEof { location, .. } => location,
            UnrecognizedToken {
                token: (location, _, _),
                ..
            } => location,
            ExtraToken {
                token: (location, _, _),
                ..
            } => location,
            User { error } => match error {},
        };

        let consumed_src = &src[..loc];
        let line_index: usize = consumed_src.lines().count().saturating_sub(1);
        let last_line: Option<&str> = consumed_src.lines().rev().next();
        let column_index = last_line.map(|l| l.len()).unwrap_or(0);
        Self::ParseError {
            line: line_index,
            column: column_index,
        }
    }
}

impl fmt::Display for LanguageError {
    fn fmt<'a>(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use LanguageError::*;
        match self {
            ParseError { line, column } => {
                let line_number = line + 1;
                let column_number = column + 1;
                write!(f, "Syntax error at {line_number}:{column_number}")?;
            }
            VariableShadowing => {
                write!(f, "Variable declared more than once")?;
            }
            UndeclaredVariable => {
                write!(f, "Usage of undeclared variable")?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for LanguageError {}
