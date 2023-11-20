#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_enum_boolean_explicit_js_trailing_commaall_format_1_0d35eef3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nenum E of boolean {\n  A = true,\n  B = false,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nenum E of boolean {\n  A = true,\n  B = false,\n}"
    );
}
#[test]
fn test_enum_boolean_implicit_js_trailing_commaall_format_1_38227c5d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  A = true,\n  B = false,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nenum E {\n  A = true,\n  B = false,\n}"
    );
}
#[test]
fn test_enum_comments_js_trailing_commaall_format_1_0e6c3ddc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("all")
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("enum E1 {\n  A = 0,\n  // B = 1,\n  C = 2\n}\n\nenum E2 of number {\n  // AA = -1,\n  A = 0,\n  // B = 1,\n  C = 2\n  // D = 100\n}\n\nenum E3 {/*Q*/}\n\nenum E4 of /*Q*/ string {\n  Foo = \"foo\"\n}\n\nenum E5 of string { // Q\n  Bar = \"bar\"\n}\n\nenum /*Q*/ E6 of string {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "enum E1 {\n  A = 0,\n  // B = 1,\n  C = 2,\n}\n\nenum E2 of number {\n  // AA = -1,\n  A = 0,\n  // B = 1,\n  C = 2,\n  // D = 100\n}\n\nenum E3 {\n  /*Q*/\n}\n\nenum E4 of string {\n  /*Q*/ Foo = \"foo\",\n}\n\nenum E5 of string {\n  // Q\n  Bar = \"bar\",\n}\n\nenum /*Q*/ E6 of string {}");
}
#[test]
fn test_enum_empty_js_trailing_commaall_format_1_1abc680c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E { }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum E {}");
}
#[test]
fn test_enum_export_js_trailing_commaall_format_1_90ff4c2f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nexport enum A {}\n\nexport default enum B {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nexport enum A {}\n\nexport default enum B {}"
    );
}
#[test]
fn test_enum_name_js_trailing_commaall_format_1_1ff594a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum type {\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum type {}");
}
#[test]
fn test_enum_no_trailing_comma_js_trailing_commaall_format_1_cd7ed882() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  A\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum E {\n  A,\n}");
}
#[test]
fn test_enum_number_explicit_js_trailing_commaall_format_1_e02d13ed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E of number {\n  A = 1,\n  B = 2,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nenum E of number {\n  A = 1,\n  B = 2,\n}"
    );
}
#[test]
fn test_enum_number_implicit_js_trailing_commaall_format_1_9d6fdff5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  A = 0,\n  B = 1,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum E {\n  A = 0,\n  B = 1,\n}");
}
#[test]
fn test_enum_string_explicit_defaulted_js_trailing_commaall_format_1_6b8dd1c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E of string {\n  A,\n  B,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum E of string {\n  A,\n  B,\n}");
}
#[test]
fn test_enum_string_explicit_initialized_js_trailing_commaall_format_1_b98b1383() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nenum E of string {\n  A = \"a\",\n  B = \"b\",\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nenum E of string {\n  A = \"a\",\n  B = \"b\",\n}"
    );
}
#[test]
fn test_enum_string_implicit_defaulted_js_trailing_commaall_format_1_b0270421() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  A,\n  B,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum E {\n  A,\n  B,\n}");
}
#[test]
fn test_enum_string_implicit_initialized_js_trailing_commaall_format_1_6ff480ef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E {\n  A = \"a\",\n  B = \"b\",\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nenum E {\n  A = \"a\",\n  B = \"b\",\n}"
    );
}
#[test]
fn test_enum_symbol_js_trailing_commaall_format_1_6de34511() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nenum E of symbol {\n  A,\n  B,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nenum E of symbol {\n  A,\n  B,\n}");
}
