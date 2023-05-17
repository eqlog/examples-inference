use crate::ast;
use crate::type_system;
use crate::type_system::TypeSystem;
use std::collections::HashMap;

pub type Bindings = HashMap<String, type_system::Var>;

fn populate_function(
    sys: &mut TypeSystem,
    bindings: &mut Bindings,
    func: &ast::Function,
) -> (type_system::Function, Option<type_system::Var>) {
    let func_var: Option<type_system::Var> = func.signature.name.as_ref().map(|name| {
        let func_var = sys.new_var();
        bindings.insert(name.to_string(), func_var);
        func_var
    });

    // Clone bindings before inserting function body-local names.
    let mut bindings: Bindings = bindings.clone();

    let arg_vars: Vec<type_system::Var> = func
        .signature
        .args
        .iter()
        .map(|arg| {
            let var = sys.new_var();
            bindings.insert(arg.name.to_string(), var);
            var
        })
        .collect();

    let mut arg_var_list: type_system::VarList = sys.define_nil_var_list();
    for var in arg_vars {
        arg_var_list = sys.define_cons_var_list(var, arg_var_list);
    }

    let body = populate_stmts(sys, &mut bindings, func.body.as_slice());
    let func = sys.define_function_def(arg_var_list, body);
    (func, func_var)
}

fn populate_stmt(
    sys: &mut TypeSystem,
    bindings: &mut Bindings,
    stmt: &ast::Stmt,
) -> type_system::Stmt {
    use ast::Stmt::*;
    match stmt {
        Expr(expr) => {
            let expr = populate_expr(sys, bindings, expr);
            sys.define_expr_stmt(expr)
        }
        Let { name, value } => {
            let value: type_system::Expr = populate_expr(sys, bindings, value);
            let var = sys.new_var();
            bindings.insert(name.to_string(), var);
            sys.define_let_stmt(var, value)
        }
        Return(expr) => {
            let expr = populate_expr(sys, bindings, expr);
            sys.define_return_stmt(expr)
        }
        ReturnVoid => sys.define_return_void_stmt(),
        Function(func) => {
            let (func, func_var) = populate_function(sys, bindings, func);
            let func_var: type_system::Var = match func_var {
                Some(func_var) => func_var,
                None => panic!("Function without name used as statement"),
            };
            sys.define_function_stmt(func_var, func)
        }
        FunctionDecl(ast::FunctionSignature {
            name,
            args,
            codomain,
        }) => {
            let mut domain_tys: type_system::TyList = sys.define_nil_ty_list();
            for ast::FunctionArg { name: _, ty } in args {
                let ty: type_system::Ty = match ty {
                    None => sys.new_ty(),
                    Some(ty) => populate_ty(sys, ty),
                };
                domain_tys = sys.define_cons_ty_list(ty, domain_tys);
            }

            let codomain_ty = match codomain {
                None => sys.new_ty(),
                Some(codomain) => populate_ty(sys, codomain),
            };
            let function_ty = sys.define_function_ty(domain_tys, codomain_ty);

            let function_var = sys.new_var();
            let function_expr = sys.define_variable_expr(function_var);
            sys.insert_expr_ty(function_expr, function_ty);

            if let Some(name) = name {
                bindings.insert(name.to_string(), function_var);
            }

            sys.define_decl_stmt(function_var, function_ty)
        }
        If {
            cond,
            true_branch,
            false_branch,
        } => {
            let cond = populate_expr(sys, bindings, cond);
            let true_branch = populate_stmts(sys, &mut bindings.clone(), true_branch);
            let false_branch = populate_stmts(sys, &mut bindings.clone(), false_branch);
            sys.define_if_stmt(cond, true_branch, false_branch)
        }
        While { cond, body } => {
            let cond = populate_expr(sys, bindings, cond);
            let body = populate_stmts(sys, &mut bindings.clone(), body);
            sys.define_while_stmt(cond, body)
        }
    }
}

pub fn populate_stmts<'a>(
    sys: &mut TypeSystem,
    bindings: &mut Bindings,
    stmts: impl IntoIterator<Item = &'a ast::Stmt>,
) -> type_system::StmtList {
    let mut stmt_list: type_system::StmtList = sys.define_nil_stmt_list();
    for stmt in stmts {
        let stmt = populate_stmt(sys, bindings, stmt);
        stmt_list = sys.define_cons_stmt_list(stmt, stmt_list);
    }
    stmt_list
}

fn populate_expr(sys: &mut TypeSystem, bindings: &Bindings, expr: &ast::Expr) -> type_system::Expr {
    use ast::Expr::*;
    match expr {
        Variable(name) => {
            let var: type_system::Var = match bindings.get(name) {
                Some(var) => *var,
                None => {
                    panic!("Variable not declared: {name}");
                }
            };
            sys.define_variable_expr(var)
        }
        Void => sys.define_void_expr(),
        False => sys.define_false_expr(),
        True => sys.define_true_expr(),
        StringLiteral(_) => sys.define_string_literal(),
        NumberLiteral(_) => sys.define_number_literal(),
        Tuple(exprs) => {
            let exprs = populate_exprs(sys, bindings, exprs.as_slice());
            sys.define_tuple(exprs)
        }
        App { function, args } => {
            let function = populate_expr(sys, bindings, function);
            let args = populate_exprs(sys, bindings, args);
            sys.define_app(function, args)
        }
        Function(function) => {
            let mut bindings = bindings.clone();
            let (func, _) = populate_function(sys, &mut bindings, function);
            sys.define_function_expr(func)
        }
    }
}

fn populate_exprs(
    sys: &mut TypeSystem,
    bindings: &Bindings,
    exprs: &[ast::Expr],
) -> type_system::ExprList {
    let mut expr_list: type_system::ExprList = sys.define_nil_expr_list();
    for expr in exprs {
        let expr = populate_expr(sys, bindings, expr);
        expr_list = sys.define_cons_expr_list(expr, expr_list);
    }
    expr_list
}

fn populate_ty(sys: &mut TypeSystem, ty: &ast::Type) -> type_system::Ty {
    use ast::Type::*;
    match ty {
        Void => sys.define_void_ty(),
        Boolean => sys.define_boolean_ty(),
        Number => sys.define_number_ty(),
        String => sys.define_string_ty(),
        Product(tys) => {
            let tys = populate_tys(sys, tys.as_slice());
            sys.define_product_ty(tys)
        }
        Function { domain, codomain } => {
            let mut domain_tys: type_system::TyList = sys.define_nil_ty_list();
            for ast::FunctionArg { name: _, ty } in domain {
                let ty: type_system::Ty = match ty {
                    None => sys.new_ty(),
                    Some(ty) => populate_ty(sys, ty),
                };
                domain_tys = sys.define_cons_ty_list(ty, domain_tys);
            }

            let codomain_ty = populate_ty(sys, codomain.as_ref());
            sys.define_function_ty(domain_tys, codomain_ty)
        }
    }
}

fn populate_tys(sys: &mut TypeSystem, tys: &[ast::Type]) -> type_system::TyList {
    let mut ty_list: type_system::TyList = sys.define_nil_ty_list();
    for ty in tys {
        let ty = populate_ty(sys, ty);
        ty_list = sys.define_cons_ty_list(ty, ty_list);
    }
    ty_list
}
