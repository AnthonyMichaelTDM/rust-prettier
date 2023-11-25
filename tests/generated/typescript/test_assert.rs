#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_ts_format_1_026d40c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a = (x: any): asserts x is string/* comment */ => {}\na = (x: any): asserts x is /* comment */string => {}\na = (x: any): asserts x is/* comment */ string => {}\na = (x: any): asserts x /* comment */is string => {}\na = (x: any): asserts x/* comment */ is string => {}\na = (x: any): asserts /* comment */x is string => {}\na = (x: any): asserts/* comment */ x is string => {}\na = (x: any): /* comment */asserts x is string => {}") ? ;
    assert_eq ! (formatted , "a = (x: any): asserts x is string /* comment */ => {};\na = (x: any): asserts x is /* comment */ string => {};\na = (x: any): asserts x is /* comment */ string => {};\na = (x: any): asserts x /* comment */ is string => {};\na = (x: any): asserts x /* comment */ is string => {};\na = (x: any): asserts /* comment */ x is string => {};\na = (x: any): asserts /* comment */ x is string => {};\na = (x: any): /* comment */ asserts x is string => {};");
    Ok(())
}
#[test]
fn test_index_ts_format_1_ffeeceb3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const assertString = (x: any): asserts x => {\n  console.assert(typeof x === 'string');\n}\n\nfunction assertsString(x: any): asserts x {\n  console.assert(typeof x === 'string');\n}\n\nconst assertStringWithGuard = (x: any): asserts x is string => {\n  console.assert(typeof x === 'string');\n}\n\nfunction assertsStringWithGuard(x: any): asserts x is string {\n  console.assert(typeof x === 'string');\n}\n\ninterface AssertFoo {\n  isString(node: any): asserts node;\n}\n\nclass AssertsFoo {\n  isBar(): asserts this {\n    return;\n  }\n  isBaz = (): asserts this => {\n    return;\n  }\n") ? ;
    assert_eq ! (formatted , "const assertString = (x: any): asserts x => {\n  console.assert(typeof x === \"string\");\n};\n\nfunction assertsString(x: any): asserts x {\n  console.assert(typeof x === \"string\");\n}\n\nconst assertStringWithGuard = (x: any): asserts x is string => {\n  console.assert(typeof x === \"string\");\n};\n\nfunction assertsStringWithGuard(x: any): asserts x is string {\n  console.assert(typeof x === \"string\");\n}\n\ninterface AssertFoo {\n  isString(node: any): asserts node;\n}\n\nclass AssertsFoo {\n  isBar(): asserts this {\n    return;\n  }\n  isBaz = (): asserts this => {\n    return;\n  };\n}");
    Ok(())
}
