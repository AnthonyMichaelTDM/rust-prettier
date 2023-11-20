#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_7e1ca87d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nfunction f(): number {\n    throw new Error(); // OK to not return\n}\n\nfunction g(a: ?string) {\n    if (a == null) {\n        throw new Error();\n    }\n    return a*1; // a is not null\n}\n\nfunction h(x: number): string {\n  if (x) {\n    return 'foo';\n  } else {\n    throw new Error();\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nfunction f(): number {\n  throw new Error(); // OK to not return\n}\n\nfunction g(a: ?string) {\n  if (a == null) {\n    throw new Error();\n  }\n  return a * 1; // a is not null\n}\n\nfunction h(x: number): string {\n  if (x) {\n    return \"foo\";\n  } else {\n    throw new Error();\n  }\n}");
}
