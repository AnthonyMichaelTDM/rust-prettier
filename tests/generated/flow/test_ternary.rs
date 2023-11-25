#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrows_js_babel_flow_format_1_dd25fc9d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Test cases from babel\n//   ref: https://github.com/babel/babel/blob/614b48678095746b83bbe517c4d6b30ba8cd5c04/packages/babel-parser/test/fixtures/flow/arrows-in-ternaries/issue-13644/input.js\n// `flow` cannot parse below codes\n//   ref: https://github.com/facebook/flow/issues/8731\n\n(a ? (b = c) : d => e); // a ? (b = c) : (d => e)\n(a ? (b += c) : d => e); // a ? (b += c) : (d => e)\n\n(a ? (b = c) : d => e : f); // a ? ((b = c): d => e) : f\n(a ? (b += c) : d => e : f); // ((a ? (b += c) : (d => e)) : f)\n\n(a ? b => (c = d) : e => f); // a ? (b => (c = d)) : (e => f)\n(a ? b => (c += d) : e => f); // a ? (b => (c += d)) : (e => f)\n\n(a ? b => (c = d) : e => f : g); // a ? (b => ((c = d): e => f)) : g\n(a ? b => (c += d) : e => f : g); // ((a ? (b => (c += d)) : (e => f)) : g)") ? ;
    assert_eq ! (formatted , "// Test cases from babel\n//   ref: https://github.com/babel/babel/blob/614b48678095746b83bbe517c4d6b30ba8cd5c04/packages/babel-parser/test/fixtures/flow/arrows-in-ternaries/issue-13644/input.js\n// `flow` cannot parse below codes\n//   ref: https://github.com/facebook/flow/issues/8731\n\na ? (b = c) : (d) => e; // a ? (b = c) : (d => e)\na ? (b += c) : (d) => e; // a ? (b += c) : (d => e)\n\na ? (b = c): d => e : f; // a ? ((b = c): d => e) : f\n((a ? (b += c) : (d) => e): f); // ((a ? (b += c) : (d => e)) : f)\n\na ? (b) => (c = d) : (e) => f; // a ? (b => (c = d)) : (e => f)\na ? (b) => (c += d) : (e) => f; // a ? (b => (c += d)) : (e => f)\n\na\n  ? (b) =>\n      (c = d): e =>\n        f\n  : g; // a ? (b => ((c = d): e => f)) : g\n((a ? (b) => (c += d) : (e) => f): g); // ((a ? (b => (c += d)) : (e => f)) : g)");
    Ok(())
}
#[test]
fn test_arrows_js_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
