use crate::kt;

#[test]
fn escaped_braces() {
    assert_eq!(kt::format!("{${}", 1), "{1");
    assert_eq!(kt::format!("}${}", 1), "}1");
    assert_eq!(kt::format!("{{${}}}", 1), "{{1}}");
}

#[test]
fn dollar_levels() {
    assert_eq!(kt::format!("$${}", 1), "$1");
    assert_eq!(kt::format!($$"${}"), "${}");
    assert_eq!(kt::format!($$"{$${}", 1), "{1");
    assert_eq!(kt::format!($$"${${$${}", 1), "${${1");
}

#[test]
fn raw_string() {
    assert_eq!(kt::format!(r"\${}", 42), r"\42");
}

#[test]
fn format_specifier() {
    assert_eq!(kt::format!("${:>5}", 42), "   42");
    assert_eq!(kt::format!("${:#x}", 255), "0xff");
    assert_eq!(kt::format!("${:05}", 42), "00042");
    assert_eq!(kt::format!("${:.2}", 1.234), "1.23");
}

#[test]
fn named_arguments() {
    let value = 42;

    assert_eq!(kt::format!("value = ${value}", value = value), "value = 42");
}

#[test]
fn positional_arguments() {
    assert_eq!(kt::format!("${} ${}", 1, 2), "1 2");
}
