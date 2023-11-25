#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_funrec_js_format_1_7f4e2dfa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function bar(x) { return x; }\nfunction foo() {\n    return function bound() {\n        return bar(bound);\n    };\n}") ? ;
    assert_eq ! (formatted , "function bar(x) {\n  return x;\n}\nfunction foo() {\n  return function bound() {\n    return bar(bound);\n  };\n}");
    Ok(())
}
