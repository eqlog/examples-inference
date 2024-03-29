use crate::check_source;
use crate::grammar_util::*;
use crate::program::*;

use indoc::indoc;

fn var_has_type(var: &str, ty: Type, p: &Program, lits: &Literals) -> bool {
    let var: Var = *lits.vars.get(var).expect("variable should be in literals");
    let ty = p.mono_type(ty).unwrap();
    p.iter_var_type_in_stmts()
        .any(|(var0, _, ty0)| p.are_equal_var(var0, var) && p.are_equal_general_type(ty, ty0))
}

#[test]
fn void_literal() {
    let (p, lits, _) = check_source(&indoc! {"
        let b = ();
    "})
    .unwrap();
    let void_type = p.void_type().unwrap();
    assert!(var_has_type("b", void_type, &p, &lits));
}

#[test]
fn number_literal() {
    let (p, lits, _) = check_source(&indoc! {"
        let b = 5;
    "})
    .unwrap();
    let number_type = p.number_type().unwrap();
    assert!(var_has_type("b", number_type, &p, &lits));
}

#[test]
fn string_literal() {
    let (p, lits, _) = check_source(&indoc! {"
        let b = 'xyz';
    "})
    .unwrap();
    let string_type = p.string_type().unwrap();
    assert!(var_has_type("b", string_type, &p, &lits));
}

#[test]
fn boolean_true() {
    let (p, lits, _) = check_source(&indoc! {"
        let b = true;
    "})
    .unwrap();
    let boolean_type = p.boolean_type().unwrap();
    assert!(var_has_type("b", boolean_type, &p, &lits));
}

#[test]
fn boolean_false() {
    let (p, lits, _) = check_source(&indoc! {"
        let b = false;
    "})
    .unwrap();
    let boolean_type = p.boolean_type().unwrap();
    assert!(var_has_type("b", boolean_type, &p, &lits));
}

#[test]
fn bad_let_boolean_number_expr() {
    let err = check_source(&indoc! {"
        let b: boolean = 5;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Conflicting type constraints");
}

#[test]
fn bad_let_number_string_expr() {
    let err = check_source(&indoc! {"
        let b: number = 'xyz';
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Conflicting type constraints");
}

#[test]
fn let_variable_variable() {
    let (p, lits, _) = check_source(&indoc! {"
        let a = 4;
        let b = a;
    "})
    .unwrap();
    let number_type = p.number_type().unwrap();
    assert!(var_has_type("b", number_type, &p, &lits));
}

#[test]
fn if_cond() {
    let (p, lits, _) = check_source(&indoc! {"
        function xyz (a) {
            if (a) {
            } else {
            }
        };
    "})
    .unwrap();
    let boolean_type = p.boolean_type().unwrap();
    assert!(var_has_type("a", boolean_type, &p, &lits));
}

#[test]
fn while_cond() {
    let (p, lits, _) = check_source(&indoc! {"
        function xyz (a) {
            while (a) {}
        };
    "})
    .unwrap();
    let boolean_type = p.boolean_type().unwrap();
    assert!(var_has_type("a", boolean_type, &p, &lits));
}

#[test]
fn equals_expr_type() {
    let (p, lits, _) = check_source(&indoc! {"
        let a = 5 == 5;
    "})
    .unwrap();
    let boolean_type = p.boolean_type().unwrap();
    assert!(var_has_type("a", boolean_type, &p, &lits));
}

#[test]
fn bad_equals_arg_types() {
    let err = check_source(&indoc! {"
        let a = 5 == 'xyz';
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Conflicting type constraints");
}

#[test]
fn app_dom_cod_to_func() {
    let (p, lits, _) = check_source(&indoc! {"
        function xyz (a) {
            let b: boolean = a('asdf', 123);
        };
    "})
    .unwrap();
    let number_type = p.number_type().unwrap();
    let string_type = p.string_type().unwrap();
    let boolean_type = p.boolean_type().unwrap();
    let dom = p
        .cons_type_list(
            string_type,
            p.cons_type_list(number_type, p.nil_type_list().unwrap())
                .unwrap(),
        )
        .unwrap();
    let func_type = p.function_type(dom, boolean_type).unwrap();
    assert!(var_has_type("a", func_type, &p, &lits));
}

#[test]
fn app_func_to_dom_cod() {
    let (p, lits, _) = check_source(&indoc! {"
        function xyz (a, b, c) {
            let d: boolean = a('asdf', 123);
            let e = a(b, c);
        };
    "})
    .unwrap();
    let number_type = p.number_type().unwrap();
    let string_type = p.string_type().unwrap();
    let boolean_type = p.boolean_type().unwrap();

    assert!(var_has_type("b", string_type, &p, &lits));
    assert!(var_has_type("c", number_type, &p, &lits));
    assert!(var_has_type("e", boolean_type, &p, &lits));
}

#[test]
fn determined_return_type_by_call_site() {
    let (p, lits, _) = check_source(&indoc! {"
        function asdf () {
          return asdf();
        }
        let x: number = asdf();
    "})
    .unwrap();
    let number_type = p.number_type().unwrap();
    assert!(var_has_type("x", number_type, &p, &lits));
}

#[test]
fn bad_conflicting_return_types() {
    let err = check_source(&indoc! {"
        function asdf (cond) {
          if (cond) {
            return 'asdf';
          } else {
            return 5;
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Conflicting type constraints");
}

#[test]
fn bad_conflicting_return_types_implicit() {
    let err = check_source(&indoc! {"
        function asdf (cond) {
          if (cond) {
            return 'asdf';
          } else {
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Conflicting type constraints");
}

#[test]
fn bad_non_well_founded_type() {
    let err = check_source(&indoc! {"
        function foo (x) {
            x(x);
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Conflicting type constraints");
}

#[test]
fn bad_type_usage_function() {
    let err = check_source(&indoc! {"
        let x: number = 5;
        function foo() {
            let y: string = x;
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Conflicting type constraints");
}

#[test]
fn bad_nested_function_in_equals() {
    // If we forget to propagate type contexts into lhs and rhs of an == expression, then the type
    // of baz will be `forall b. () => b`, since the type `a` of `x` will not be in the type
    // context of `bar`. Thus, `foo` will have type `forall a b. (a, b) => void` when it should
    // actually have type `forall a. (a, a) => void`.
    let err = check_source(&indoc! {"
        function foo (x, y) {
            y ==
                (function bar() {
                    function baz() {
                        return x;
                    }
                    return baz();
                })();
        }
        foo(1, 'xyz');
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Conflicting type constraints");
}
