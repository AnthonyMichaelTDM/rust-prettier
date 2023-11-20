#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_eed3687e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule A\n * @flow\n */\n\nmodule.exports = {\n  numberValueA: 1,\n  stringValueA: \"someString\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule A\n * @flow\n */\n\nmodule.exports = {\n  numberValueA: 1,\n  stringValueA: \"someString\",\n};");
}
#[test]
fn test_b_js_format_1_6e334f34() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule B\n * @flow\n */\n\nmodule.exports = {\n  numberValueB: 1,\n  stringValueB: \"someString\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule B\n * @flow\n */\n\nmodule.exports = {\n  numberValueB: 1,\n  stringValueB: \"someString\",\n};");
}
#[test]
fn test_require_lazy_js_format_1_91c220ce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nrequireLazy(['A', 'B'], function(A, B) {\n  var num1: number = A.numberValueA;\n  var str1: string = A.stringValueA;\n  var num2: number = A.stringValueA; // Error: string ~> number\n  var str2: string = A.numberValueA; // Error: number ~> string\n\n  var num3: number = B.numberValueB;\n  var str3: string = B.stringValueB;\n  var num4: number = B.stringValueB; // Error: string ~> number\n  var str4: string = B.numberValueB; // Error: number ~> string\n});\n\nvar notA: Object = A;\nvar notB: Object = B;\n\nrequireLazy(); // Error: No args\nrequireLazy([nope], function() {}); // Error: Non-stringliteral args\nrequireLazy(['A']); // Error: No callback expression") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nrequireLazy([\"A\", \"B\"], function (A, B) {\n  var num1: number = A.numberValueA;\n  var str1: string = A.stringValueA;\n  var num2: number = A.stringValueA; // Error: string ~> number\n  var str2: string = A.numberValueA; // Error: number ~> string\n\n  var num3: number = B.numberValueB;\n  var str3: string = B.stringValueB;\n  var num4: number = B.stringValueB; // Error: string ~> number\n  var str4: string = B.numberValueB; // Error: number ~> string\n});\n\nvar notA: Object = A;\nvar notB: Object = B;\n\nrequireLazy(); // Error: No args\nrequireLazy([nope], function () {}); // Error: Non-stringliteral args\nrequireLazy([\"A\"]); // Error: No callback expression");
}
