#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_traces_js_format_1_bbdae1ed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// arg/param type mismatch on arg 0\nfunction g0(y:string) { }\nfunction f0(x) { g0(x) }\nf0(0);\n\n// ...on arg n\nfunction g1(a:string, b:string) { }\nfunction f1(x, y) { g1(x, y) }\nf1(\"hey\", 0);\n\n// h/o call with function expr\nfunction g2(ylam: (s:string) => number) { }\nfunction f2(xlam) { g2(xlam) }\nf2(function(x) { return x * x });\n\n// h/o call with function def\nfunction g3(ylam: (s:string) => number) { }\nfunction f3(xlam) { g3(xlam) }\nfunction double(n) { return n * 2 }\nf3(double);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// arg/param type mismatch on arg 0\nfunction g0(y: string) {}\nfunction f0(x) {\n  g0(x);\n}\nf0(0);\n\n// ...on arg n\nfunction g1(a: string, b: string) {}\nfunction f1(x, y) {\n  g1(x, y);\n}\nf1(\"hey\", 0);\n\n// h/o call with function expr\nfunction g2(ylam: (s: string) => number) {}\nfunction f2(xlam) {\n  g2(xlam);\n}\nf2(function (x) {\n  return x * x;\n});\n\n// h/o call with function def\nfunction g3(ylam: (s: string) => number) {}\nfunction f3(xlam) {\n  g3(xlam);\n}\nfunction double(n) {\n  return n * 2;\n}\nf3(double);");
}
#[test]
fn test_traces_2_js_format_1_facf26f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nvar React = require('react');\n\nvar A = React.createClass({\n  propTypes: { foo: React.PropTypes.string.isRequired }\n});\n\nvar B = React.createClass({\n  propTypes: { bar: React.PropTypes.string.isRequired }\n});\n\nfunction f(b): React.Element<*> {\n  if (b) {\n    return <A foo=\"hey\"/>;\n  } else {\n    return <B bar=\"hey\"/>;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nvar React = require(\"react\");\n\nvar A = React.createClass({\n  propTypes: { foo: React.PropTypes.string.isRequired },\n});\n\nvar B = React.createClass({\n  propTypes: { bar: React.PropTypes.string.isRequired },\n});\n\nfunction f(b): React.Element<*> {\n  if (b) {\n    return <A foo=\"hey\" />;\n  } else {\n    return <B bar=\"hey\" />;\n  }\n}");
}
