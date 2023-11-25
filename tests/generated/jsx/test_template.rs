use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_styled_components_js_format_1_dc949d50() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<style jsx>{\\`\n  p {\n    color: red;\n  }\n\\`}</style>;\n\n<style jsx>{tpl\\`\n  p {\n    color: red;\n  }\n\\`}</style>;\n\n<style jsx>\n  {\\`p {\n     color: red;\n     }\n  \\`}\n</style>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<style jsx>{\\`\n  p {\n    color: red;\n  }\n\\`}</style>;\n\n<style jsx>{tpl\\`\n  p {\n    color: red;\n  }\n\\`}</style>;\n\n<style jsx>\n  {\\`\n    p {\n      color: red;\n    }\n  \\`}\n</style>;");
}
