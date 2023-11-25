#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_no_semi_ts_semifalse_format_1_05c264a3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  bar: A;\n  [baz]\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  bar: A;\n  private [baz]\n}") ? ;
    assert_eq ! (formatted , "class A {\n  bar: A;\n  [baz]\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  bar: A\n  private [baz]\n}");
    Ok(())
}
#[test]
fn test_no_semi_ts_format_1_05c264a3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  bar: A;\n  [baz]\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  bar: A;\n  private [baz]\n}") ? ;
    assert_eq ! (formatted , "class A {\n  bar: A;\n  [baz];\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  bar: A;\n  private [baz];\n}");
    Ok(())
}
#[test]
fn test_non_null_ts_semifalse_format_1_31c23aa0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// the 2nd line needs ASI protection\nconst el = ReactDOM.findDOMNode(ref)\n;(el as HTMLElement)!.style.cursor = 'pointer'") ? ;
    assert_eq ! (formatted , "// the 2nd line needs ASI protection\nconst el = ReactDOM.findDOMNode(ref)\n;(el as HTMLElement)!.style.cursor = \"pointer\"");
    Ok(())
}
#[test]
fn test_non_null_ts_format_1_31c23aa0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// the 2nd line needs ASI protection\nconst el = ReactDOM.findDOMNode(ref)\n;(el as HTMLElement)!.style.cursor = 'pointer'") ? ;
    assert_eq ! (formatted , "// the 2nd line needs ASI protection\nconst el = ReactDOM.findDOMNode(ref);\n(el as HTMLElement)!.style.cursor = \"pointer\";");
    Ok(())
}
