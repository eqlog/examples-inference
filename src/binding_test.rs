use crate::check_source;

use indoc::indoc;

#[test]
fn bad_top_level_duplicate_let() {
    let err = check_source(&indoc! {"
        let k = 5;
        let k = 6;
    "})
    .unwrap_err();
    assert_eq!(&err.to_string(), "Variable declared more than once");
}
