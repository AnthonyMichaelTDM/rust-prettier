#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_embedded_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_embedded_js_format_1_cde30fa7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const GQL_QUERY_WITH_CONST = /* GraphQL */ `\n  query S { shop }\n` as const;\n\nconst HTML_WITH_CONST = /* HTML */ `\n<div>\n<h1>foo</h1>\n  <p>foo</p>\n</div>\n` as const;") ? ;
    assert_eq ! (formatted , "const GQL_QUERY_WITH_CONST = /* GraphQL */ `\n  query S {\n    shop\n  }\n` as const;\n\nconst HTML_WITH_CONST = /* HTML */ `\n  <div>\n    <h1>foo</h1>\n    <p>foo</p>\n  </div>\n` as const;");
    Ok(())
}
