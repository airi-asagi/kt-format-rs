use crate::kt;
use std::fmt::Write;

#[test]
fn format() {
    assert_eq!(kt::format!("hello"), "hello");
    assert_eq!(kt::format!("value = ${}", 42), "value = 42");
    assert_eq!(kt::format!("value = ${value}", value = 42), "value = 42");
    assert_eq!(kt::format!("hex = ${:#x}", 255), "hex = 0xff");
}

#[test]
fn write() {
    let mut output = String::new();
    kt::write!(output, "value = ${}", 42).unwrap();
    assert_eq!(output, "value = 42");
}

#[test]
fn writeln() {
    let mut output = String::new();
    kt::writeln!(output, "value = ${}", 42).unwrap();
    assert_eq!(output, "value = 42\n");
}

#[test]
fn multi_write() {
    let mut output = String::new();
    kt::writeln!(output, "value1 = ${}", 42).unwrap();
    kt::write!(output, "value2 = ${}", 24).unwrap();
    assert_eq!(output, "value1 = 42\nvalue2 = 24");
}

#[test]
fn print() {
    kt::print!("value = ${}", 42);
}

#[test]
fn eprint() {
    kt::eprint!("value = ${}", 42);
}

#[test]
fn println() {
    kt::println!("value = ${}", 42);
}

#[test]
fn eprintln() {
    kt::eprintln!("value = ${}", 42);
}

#[test]
#[should_panic(expected = "test panic: 42")]
fn panic() {
    kt::panic!("test panic: ${}", 42);
}

#[test]
#[should_panic(expected = "test todo: 42")]
fn todo() {
    kt::todo!("test todo: ${}", 42);
}

#[test]
#[should_panic(expected = "test unimplemented")]
fn unimplemented() {
    kt::unimplemented!("test unimplemented");
}

#[test]
#[should_panic(expected = "test unreachable")]
fn unreachable() {
    kt::unreachable!("test unreachable");
}
