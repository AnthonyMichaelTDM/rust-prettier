#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_extends_bound_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_extends_bound_js_format_1_30a052dc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\ntype F = <T extends string>(t: T) => T;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\ntype F = <T extends string>(t: T) => T;"
    );
}
#[test]
fn test_issue_5283_js_format_1_25a3123b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nlet f: <A>((\n  (?A) => B\n)) => B;\n\n\nexport const testFunctionOnOptionsAsArgument: <T1,a>(?a, ((?a) => T1)) => T1 = function _(Arg1, Arg2) { const result = TypesBS.testFunctionOnOptionsAsArgument((Arg1 == null ? undefined : Arg1), Arg2); return result };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let f: <A>(((?A) => B)) => B;\n\nexport const testFunctionOnOptionsAsArgument: <T1, a>(?a, ((?a) => T1)) => T1 =\n  function _(Arg1, Arg2) {\n    const result = TypesBS.testFunctionOnOptionsAsArgument(\n      Arg1 == null ? undefined : Arg1,\n      Arg2,\n    );\n    return result;\n  };");
}
