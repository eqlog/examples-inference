use crate::check_source;

use indoc::indoc;

#[test]
fn bad_undeclared_variable() {
    let err = check_source(&indoc! {"
        let b = 5;
        a();
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Usage of undeclared variable");
}

#[test]
fn bad_duplicate_let_let() {
    let err = check_source(&indoc! {"
        let k = 5;
        let k = 6;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_function_function() {
    let err = check_source(&indoc! {"
        function k () {}
        function k () {}
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_let_function() {
    let err = check_source(&indoc! {"
        let k = 5;
        function k () {}
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_function_let() {
    let err = check_source(&indoc! {"
        function k () {}
        let k = 5;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_let_function_arg() {
    let err = check_source(&indoc! {"
        let k = 5;
        function asdf (k: number) {}
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_function_name_arg() {
    let err = check_source(&indoc! {"
        function k (k: number) {}
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_function_arg_arg() {
    let err = check_source(&indoc! {"
        function asdf (k: number, k: number) {}
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_ambient_function_local() {
    let err = check_source(&indoc! {"
        let k = 5;
        function asdf () {
            let k = 6;
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_function_name_local() {
    let err = check_source(&indoc! {"
        function k () {
            let k = 6;
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_function_arg_local() {
    let err = check_source(&indoc! {"
        function asdf (k: number) {
            let k = 6;
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_nested_if_true() {
    let err = check_source(&indoc! {"
        function asdf () {
            let k = 5;
            if (true) {
                let k = 3;
            } else {
            }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_nested_if_false() {
    let err = check_source(&indoc! {"
        function asdf () {
            let k = 5;
            if (true) {
            } else {
                let k = 3;
            }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_nested_while() {
    let err = check_source(&indoc! {"
        function asdf () {
            let k = 5;
            while (true) {
                let k = 3;
            }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_nested_function_expr_name() {
    let err = check_source(&indoc! {"
        function asdf () {
            let k = 5;
            (function k () {})();
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_nested_function_expr_arg() {
    let err = check_source(&indoc! {"
        function asdf () {
            let k = 5;
            (function xyz (k: number) {})(k);
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_nested_function_expr_body() {
    let err = check_source(&indoc! {"
        function asdf () {
            let k = 5;
            (function xyz () {
                let k = 6; 
            })();
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}

#[test]
fn bad_duplicate_deeply_nested() {
    let err = check_source(&indoc! {"
        let k = 5;
        function asdf () {
            while (true) {
                function xyz() {
                    if (false) {
                        let k = 6;
                    } else {
                    }
                }
            }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}
