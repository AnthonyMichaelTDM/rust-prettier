#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_comment_ts_format_1_026d40c9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a = (x: any): asserts x is string/* comment */ => {}\na = (x: any): asserts x is /* comment */string => {}\na = (x: any): asserts x is/* comment */ string => {}\na = (x: any): asserts x /* comment */is string => {}\na = (x: any): asserts x/* comment */ is string => {}\na = (x: any): asserts /* comment */x is string => {}\na = (x: any): asserts/* comment */ x is string => {}\na = (x: any): /* comment */asserts x is string => {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a = (x: any): asserts x is string /* comment */ => {};\na = (x: any): asserts x is /* comment */ string => {};\na = (x: any): asserts x is /* comment */ string => {};\na = (x: any): asserts x /* comment */ is string => {};\na = (x: any): asserts x /* comment */ is string => {};\na = (x: any): asserts /* comment */ x is string => {};\na = (x: any): asserts /* comment */ x is string => {};\na = (x: any): /* comment */ asserts x is string => {};");
}
#[test]
fn test_index_ts_format_1_ffeeceb3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const assertString = (x: any): asserts x => {\n  console.assert(typeof x === 'string');\n}\n\nfunction assertsString(x: any): asserts x {\n  console.assert(typeof x === 'string');\n}\n\nconst assertStringWithGuard = (x: any): asserts x is string => {\n  console.assert(typeof x === 'string');\n}\n\nfunction assertsStringWithGuard(x: any): asserts x is string {\n  console.assert(typeof x === 'string');\n}\n\ninterface AssertFoo {\n  isString(node: any): asserts node;\n}\n\nclass AssertsFoo {\n  isBar(): asserts this {\n    return;\n  }\n  isBaz = (): asserts this => {\n    return;\n  }\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const assertString = (x: any): asserts x => {\n  console.assert(typeof x === \"string\");\n};\n\nfunction assertsString(x: any): asserts x {\n  console.assert(typeof x === \"string\");\n}\n\nconst assertStringWithGuard = (x: any): asserts x is string => {\n  console.assert(typeof x === \"string\");\n};\n\nfunction assertsStringWithGuard(x: any): asserts x is string {\n  console.assert(typeof x === \"string\");\n}\n\ninterface AssertFoo {\n  isString(node: any): asserts node;\n}\n\nclass AssertsFoo {\n  isBar(): asserts this {\n    return;\n  }\n  isBaz = (): asserts this => {\n    return;\n  };\n}");
}
