#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_inline_js_format_1_3c0fb379() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(): {\n  someLong: boolean,\n  t: boolean\n} => \\`\n  a\n\\`;\n\n(): {\n  someLong: boolean,\n  t: boolean\n} =>\n  \\`\n    a\n  \\`;\n\n(\n  someLong: boolean,\n  t: boolean\n) => \\`\n    a\n  \\`;\n\n(\n  someLong: boolean,\n  t: boolean\n) =>\n  \\`\n    a\n  \\`;") ? ;
    assert_eq ! (formatted , "(): {\n  someLong: boolean,\n  t: boolean,\n} => \\`\n  a\n\\`;\n\n(): {\n  someLong: boolean,\n  t: boolean,\n} =>\n  \\`\n    a\n  \\`;\n\n(someLong: boolean, t: boolean) => \\`\n    a\n  \\`;\n\n(someLong: boolean, t: boolean) =>\n  \\`\n    a\n  \\`;");
    Ok(())
}
