#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_p_js_format_1_cc03cbee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("module.exports = require('M');")?;
    assert_eq!(formatted, "module.exports = require(\"M\");");
    Ok(())
}
#[test]
fn test_test_js_format_1_5f83bfc5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var M = require('M');\nvar N = require('N');\nN.x = M(N.x);\nvar P = require('./P'); // implementation of P redirects to module M\nN.y = P(N.y);\nvar Q = require('Q'); // declaration of Q redirects to module M\nN.z = Q(N.z);") ? ;
    assert_eq ! (formatted , "var M = require(\"M\");\nvar N = require(\"N\");\nN.x = M(N.x);\nvar P = require(\"./P\"); // implementation of P redirects to module M\nN.y = P(N.y);\nvar Q = require(\"Q\"); // declaration of Q redirects to module M\nN.z = Q(N.z);");
    Ok(())
}
