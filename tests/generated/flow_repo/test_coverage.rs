#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_crash_js_format_1_c804f799() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// This file triggers a violation of the \"disjoint-or-nested ranges invariant\"\n// that we implicitly assume in type-at-pos and coverage implementations. In\n// particular, when unchecked it causes a crash with coverage --color.\n\ndeclare module foo {\n}\n\ndeclare module bar {\n}\n\n// TODO") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// This file triggers a violation of the \"disjoint-or-nested ranges invariant\"\n// that we implicitly assume in type-at-pos and coverage implementations. In\n// particular, when unchecked it causes a crash with coverage --color.\n\ndeclare module foo {\n}\n\ndeclare module bar {\n}\n\n// TODO");
}
#[test]
fn test_declare_module_js_format_1_7583d8d2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// check coverage of declare module\n\ndeclare module foo {\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// check coverage of declare module\n\ndeclare module foo {\n}"
    );
}
#[test]
fn test_no_pragma_js_format_1_26864d4b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let x = 0;\n(x: string);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let x = 0;\n(x: string);");
}
#[test]
fn test_non_termination_js_format_1_8cc5f67b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// This file triggers a violation of the \"disjoint-or-nested ranges invariant\"\n// that we implicitly assume in type-at-pos and coverage implementations. In\n// particular, when unchecked it causes non-termination with coverage --color.\n\ndeclare module foo {\n}\n\ndeclare module bar {\n}\n\n// TODO\n\ndeclare class qux {\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// This file triggers a violation of the \"disjoint-or-nested ranges invariant\"\n// that we implicitly assume in type-at-pos and coverage implementations. In\n// particular, when unchecked it causes non-termination with coverage --color.\n\ndeclare module foo {\n}\n\ndeclare module bar {\n}\n\n// TODO\n\ndeclare class qux {}");
}
