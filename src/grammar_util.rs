use crate::program::*;

use itertools::Itertools;
use std::collections::HashMap;

// TODO: Use rust's built-in never type ! once it is stabilized.
pub enum NeverType {}

/// Erases all line comments (i.e., those starting with `//`).
///
/// Since we're only deleting suffixes of lines, positions given by a (line, column) pair into the
/// source with comments erased are also valid for the original source.
pub fn erase_comments(src: &str) -> String {
    src.lines()
        .map(|line| {
            let comment_begin: Option<usize> = line.find("//");
            match comment_begin {
                Some(comment_begin) => &line[..comment_begin],
                None => line,
            }
        })
        .join("\n")
}

#[derive(Clone, Debug)]
pub struct Literals {
    pub vars: HashMap<String, Var>,
    pub strings: HashMap<String, StringLiteral>,
    pub numbers: HashMap<String, NumberLiteral>,
}

impl Literals {
    pub fn new() -> Literals {
        Literals {
            vars: HashMap::new(),
            strings: HashMap::new(),
            numbers: HashMap::new(),
        }
    }
}

pub fn expr_list_node(nodes: &[ExprNode], p: &mut Program) -> ExprListNode {
    let mut l = p.new_expr_list_node();
    p.insert_nil_expr_list_node(l);
    for node in nodes.iter().rev() {
        let cons = p.new_expr_list_node();
        p.insert_cons_expr_list_node(cons, *node, l);
        l = cons;
    }
    l
}

pub fn stmt_list_node(nodes: &[StmtNode], p: &mut Program) -> StmtListNode {
    let mut l = p.new_stmt_list_node();
    p.insert_nil_stmt_list_node(l);
    for node in nodes.iter().rev() {
        let cons = p.new_stmt_list_node();
        p.insert_cons_stmt_list_node(cons, *node, l);
        l = cons;
    }
    l
}

pub fn opt_type_node(node: Option<TypeNode>, p: &mut Program) -> OptTypeNode {
    let otn = p.new_opt_type_node();
    match node {
        Some(node) => {
            p.insert_some_opt_type_node(otn, node);
        }
        None => {
            p.insert_none_opt_type_node(otn);
        }
    }
    otn
}

pub fn arg_list_node(args: &[(Var, OptTypeNode)], p: &mut Program) -> ArgListNode {
    let mut l = p.new_arg_list_node();
    p.insert_nil_arg_list_node(l);
    for (var, otn) in args.iter().rev() {
        let cons = p.new_arg_list_node();
        p.insert_cons_arg_list_node(cons, *var, *otn, l);
        l = cons;
    }
    l
}
