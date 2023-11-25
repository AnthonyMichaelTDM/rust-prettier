#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_7e1ca87d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nfunction f(): number {\n    throw new Error(); // OK to not return\n}\n\nfunction g(a: ?string) {\n    if (a == null) {\n        throw new Error();\n    }\n    return a*1; // a is not null\n}\n\nfunction h(x: number): string {\n  if (x) {\n    return 'foo';\n  } else {\n    throw new Error();\n  }\n}") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nfunction f(): number {\n  throw new Error(); // OK to not return\n}\n\nfunction g(a: ?string) {\n  if (a == null) {\n    throw new Error();\n  }\n  return a * 1; // a is not null\n}\n\nfunction h(x: number): string {\n  if (x) {\n    return \"foo\";\n  } else {\n    throw new Error();\n  }\n}");
    Ok(())
}
