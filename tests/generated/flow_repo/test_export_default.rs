#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_p_js_format_1_cc03cbee() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("module.exports = require('M');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "module.exports = require(\"M\");");
}
#[test]
fn test_test_js_format_1_5f83bfc5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var M = require('M');\nvar N = require('N');\nN.x = M(N.x);\nvar P = require('./P'); // implementation of P redirects to module M\nN.y = P(N.y);\nvar Q = require('Q'); // declaration of Q redirects to module M\nN.z = Q(N.z);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var M = require(\"M\");\nvar N = require(\"N\");\nN.x = M(N.x);\nvar P = require(\"./P\"); // implementation of P redirects to module M\nN.y = P(N.y);\nvar Q = require(\"Q\"); // declaration of Q redirects to module M\nN.z = Q(N.z);");
}
