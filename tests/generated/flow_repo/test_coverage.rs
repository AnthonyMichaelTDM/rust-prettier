#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_crash_js_format_1_c804f799() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// This file triggers a violation of the \"disjoint-or-nested ranges invariant\"\n// that we implicitly assume in type-at-pos and coverage implementations. In\n// particular, when unchecked it causes a crash with coverage --color.\n\ndeclare module foo {\n}\n\ndeclare module bar {\n}\n\n// TODO") ? ;
    assert_eq ! (formatted , "// This file triggers a violation of the \"disjoint-or-nested ranges invariant\"\n// that we implicitly assume in type-at-pos and coverage implementations. In\n// particular, when unchecked it causes a crash with coverage --color.\n\ndeclare module foo {\n}\n\ndeclare module bar {\n}\n\n// TODO");
    Ok(())
}
#[test]
fn test_declare_module_js_format_1_7583d8d2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// check coverage of declare module\n\ndeclare module foo {\n}")?;
    assert_eq!(
        formatted,
        "// check coverage of declare module\n\ndeclare module foo {\n}"
    );
    Ok(())
}
#[test]
fn test_no_pragma_js_format_1_26864d4b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let x = 0;\n(x: string);")?;
    assert_eq!(formatted, "let x = 0;\n(x: string);");
    Ok(())
}
#[test]
fn test_non_termination_js_format_1_8cc5f67b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// This file triggers a violation of the \"disjoint-or-nested ranges invariant\"\n// that we implicitly assume in type-at-pos and coverage implementations. In\n// particular, when unchecked it causes non-termination with coverage --color.\n\ndeclare module foo {\n}\n\ndeclare module bar {\n}\n\n// TODO\n\ndeclare class qux {\n}") ? ;
    assert_eq ! (formatted , "// This file triggers a violation of the \"disjoint-or-nested ranges invariant\"\n// that we implicitly assume in type-at-pos and coverage implementations. In\n// particular, when unchecked it causes non-termination with coverage --color.\n\ndeclare module foo {\n}\n\ndeclare module bar {\n}\n\n// TODO\n\ndeclare class qux {}");
    Ok(())
}
