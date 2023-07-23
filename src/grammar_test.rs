use crate::check_source;

use indoc::indoc;

#[test]
fn bad_missing_semicolon() {
    let err = check_source(&indoc! {"
        let k = 5
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:10");
}

#[test]
fn bad_variable_initialization_missing() {
    let err = check_source(&indoc! {"
        let x;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:6");
}

#[test]
fn bad_variable_initialization_missing_expr() {
    let err = check_source(&indoc! {"
        let x =;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:8");
}

#[test]
fn bad_variable_name_digit() {
    let err = check_source(&indoc! {"
        let 53x;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:5");
}

#[test]
fn bad_unterminated_string_literal() {
    let err = check_source(&indoc! {"
        let x = 'sdflkjs_djlksdf;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:9");
}

#[test]
fn bad_function_args_missing() {
    let err = check_source(&indoc! {"
        function asdf {}
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:15");
}

#[test]
fn bad_function_statement_name_missing() {
    let err = check_source(&indoc! {"
        function () {}
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:10");
}

#[test]
fn bad_function_body_missing() {
    let err = check_source(&indoc! {"
        function asdf ();
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:17");
}

#[test]
fn bad_function_unterminated_body() {
    let err = check_source(&indoc! {"
        function asdf () { 
          let x =;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:10");
}

#[test]
fn bad_function_unterminated_arg_list() {
    let err = check_source(&indoc! {"
        function asdf (x: number { 
          let x =;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:26");
}

#[test]
fn bad_if_bad_condition_expr() {
    let err = check_source(&indoc! {"
        function asdf (y: string) { 
          if (---) {
          } else {
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:7");
}

#[test]
fn bad_if_missing_condition() {
    let err = check_source(&indoc! {"
        function asdf (y: string) { 
          if {
          } else {
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:6");
}

#[test]
fn bad_if_missing_true_block() {
    let err = check_source(&indoc! {"
        function asdf (y: string) { 
          if (true)
          else {
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 3:3");
}

#[test]
fn bad_if_unterminated_true_block() {
    let err = check_source(&indoc! {"
        function asdf (y: string) { 
          if (true) {
          else {
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 3:3");
}

#[test]
fn bad_if_missing_else() {
    let err = check_source(&indoc! {"
        function asdf (y: (x : void) => boolean) { 
          if (false) {
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 3:4");
}

#[test]
fn bad_if_missing_else_block() {
    let err = check_source(&indoc! {"
        function asdf (y: string) { 
          if (true) {
          } else
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 3:9");
}

#[test]
fn bad_if_unterminated_else_block() {
    let err = check_source(&indoc! {"
        function asdf (y: string) { 
          if (true) {
            let y: string = '123';
          } else {
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 5:2");
}

#[test]
fn bad_while_missing_body() {
    let err = check_source(&indoc! {"
        function asdf () { 
          while (false)
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:16");
}

#[test]
fn bad_while_unterminated_body() {
    let err = check_source(&indoc! {"
        function asdf () { 
          while (false) {
            let x = false;
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 4:2");
}

#[test]
fn bad_while_missing_condition() {
    let err = check_source(&indoc! {"
        function asdf () { 
          while {
            let x = false;
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:9");
}

#[test]
fn bad_while_missing_condition_expr() {
    let err = check_source(&indoc! {"
        function asdf () { 
          while () {
            let x = false;
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:10");
}

#[test]
fn bad_while_bad_condition_expr() {
    let err = check_source(&indoc! {"
        function asdf () { 
          while (---) {
            let x = false;
          }
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:10");
}

#[test]
fn bad_expr_return() {
    let err = check_source(&indoc! {"
        function asdf () { 
          let x = return;
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:11");
}

#[test]
fn bad_return_expr_type() {
    let err = check_source(&indoc! {"
        function asdf () { 
          return number;
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:10");
}

#[test]
fn bad_app_expr_unterminated_args() {
    let err = check_source(&indoc! {"
        function asdf () { 
          asdf(;
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 2:8");
}

#[test]
fn bad_app_expr_missing_arg_comma() {
    let err = check_source(&indoc! {"
        function asdf () { 
        }
        let x = 5;
        asdf(x x);
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 4:8");
}

#[test]
fn bad_equals_missing_lhs() {
    let err = check_source(&indoc! {"
        if (== 5) {
        } else {
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:5");
}

#[test]
fn bad_equals_missing_rhs() {
    let err = check_source(&indoc! {"
        if (5 == ) {
        } else {
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:10");
}

#[test]
fn bad_equals_doesnt_associate() {
    let err = check_source(&indoc! {"
        if (5 == 3 == 2) {
        } else {
        }
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Syntax error at 1:12");
}
