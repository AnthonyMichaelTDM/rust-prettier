#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_extends_bound_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_extends_bound_js_format_1_30a052dc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\ntype F = <T extends string>(t: T) => T;")?;
    assert_eq!(
        formatted,
        "// @flow\n\ntype F = <T extends string>(t: T) => T;"
    );
    Ok(())
}
#[test]
fn test_issue_5283_js_format_1_25a3123b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nlet f: <A>((\n  (?A) => B\n)) => B;\n\n\nexport const testFunctionOnOptionsAsArgument: <T1,a>(?a, ((?a) => T1)) => T1 = function _(Arg1, Arg2) { const result = TypesBS.testFunctionOnOptionsAsArgument((Arg1 == null ? undefined : Arg1), Arg2); return result };") ? ;
    assert_eq ! (formatted , "let f: <A>(((?A) => B)) => B;\n\nexport const testFunctionOnOptionsAsArgument: <T1, a>(?a, ((?a) => T1)) => T1 =\n  function _(Arg1, Arg2) {\n    const result = TypesBS.testFunctionOnOptionsAsArgument(\n      Arg1 == null ? undefined : Arg1,\n      Arg2,\n    );\n    return result;\n  };");
    Ok(())
}
