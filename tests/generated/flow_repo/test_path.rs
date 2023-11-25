#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_while_js_format_1_bbe9cf09() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x = 1;\nwhile(typeof x == \"number\" || typeof x == \"string\") {\n    x = x + 1;\n    if (true) x = \"\";\n}\nvar z:number = x;") ? ;
    assert_eq ! (formatted , "var x = 1;\nwhile (typeof x == \"number\" || typeof x == \"string\") {\n  x = x + 1;\n  if (true) x = \"\";\n}\nvar z: number = x;");
    Ok(())
}
