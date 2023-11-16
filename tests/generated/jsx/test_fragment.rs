#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_fragment_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_fragment_js_format_1_13b1ee51() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<></>;\n\n<>\n   text\n</>;\n\n<>\n  <Component />\n  <Component />\n</>;\n\n<>\n  text\n  <h2>heading</h2>\n  text\n  <h2>heading</h2>\n  text\n</>;\n\n<div>\n  <>\n    <>\n      <span>Hello</span>\n      <span>world</span>\n    </>\n    <>\n      <span>Goodbye</span>\n      <span>world</span>\n    </>\n  </>\n</div>;\n\nfoo = (\n  // comment\n  <></>\n);\n\n</* open fragment */>\n  <Component />\n  <Component />\n</ /* close fragment */>;\n\n< // open fragment\n>\n  <Component />\n  <Component />\n</ // close fragment\n>;\n\n[<></>, <></>];\nconst fun1 = () => <></>;\nx = <></>\nfunction fun2(param = <></>) {}\n1 + <></>;\n1 || <></>;\nfun2(<></>);\ntest ? <></> : x;\n<></>;\n<a>\n  <></>\n</a>;\nconst obj = {\n  foo: <></>\n};\nconst fragmentVar = <></>;\nfunction fun3() {\n  return <></>;\n}\n(<></>).toString();\n(<></>).props;\n(<></>)[\"computed\"];\n(<></>)[\"computed\"]();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<></>;\n\n<>text</>;\n\n<>\n  <Component />\n  <Component />\n</>;\n\n<>\n  text\n  <h2>heading</h2>\n  text\n  <h2>heading</h2>\n  text\n</>;\n\n<div>\n  <>\n    <>\n      <span>Hello</span>\n      <span>world</span>\n    </>\n    <>\n      <span>Goodbye</span>\n      <span>world</span>\n    </>\n  </>\n</div>;\n\nfoo = (\n  // comment\n  <></>\n);\n\n</* open fragment */>\n  <Component />\n  <Component />\n</ /* close fragment */>;\n\n<\n  // open fragment\n>\n  <Component />\n  <Component />\n</\n  // close fragment\n>;\n\n[<></>, <></>];\nconst fun1 = () => <></>;\nx = <></>;\nfunction fun2(param = <></>) {}\n1 + <></>;\n1 || <></>;\nfun2(<></>);\ntest ? <></> : x;\n<></>;\n<a>\n  <></>\n</a>;\nconst obj = {\n  foo: <></>,\n};\nconst fragmentVar = <></>;\nfunction fun3() {\n  return <></>;\n}\n(<></>).toString();\n(<></>).props;\n(<></>)[\"computed\"];\n(<></>)[\"computed\"]();");
}
