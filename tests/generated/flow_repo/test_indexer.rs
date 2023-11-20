#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_c9676e8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// No indexer should be fine\nfunction foo0(): {} {\n  return { foo: \"bar\" }\n}\n\n// Matching indexer should be fine\nfunction foo1(): {[key: string]: string} {\n  return { foo: \"bar\" }\n}\n\n// Indexer with different key type is an error when it matches\nfunction foo2(): {[key: number]: string} {\n  return { foo: \"bar\" }\n}\n\n// Matching indexer with different value type is an error\nfunction foo3(): {[key: string]: number} {\n  return { foo: \"bar\" }\n}\n\n// Indexer with different key type and different value type is twice an error\nfunction foo4(): {[key: number]: number} {\n  return { foo: \"bar\" }\n}\n\n// If key exists in object type then indexer is not matched\nfunction foo5(): {[key: string]: number; foo: string} {\n  return { foo: \"bar\" }\n}\n\n// If key exists in object type then indexer is not matched\nfunction foo6(): {[key: number]: number; foo: string} {\n  return { foo: \"bar\" }\n}\n\n// Should still complain about mistyped properties\nfunction foo7(): {[key: string]: number; foo: number} {\n  return { foo: \"bar\" }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// No indexer should be fine\nfunction foo0(): {} {\n  return { foo: \"bar\" };\n}\n\n// Matching indexer should be fine\nfunction foo1(): { [key: string]: string } {\n  return { foo: \"bar\" };\n}\n\n// Indexer with different key type is an error when it matches\nfunction foo2(): { [key: number]: string } {\n  return { foo: \"bar\" };\n}\n\n// Matching indexer with different value type is an error\nfunction foo3(): { [key: string]: number } {\n  return { foo: \"bar\" };\n}\n\n// Indexer with different key type and different value type is twice an error\nfunction foo4(): { [key: number]: number } {\n  return { foo: \"bar\" };\n}\n\n// If key exists in object type then indexer is not matched\nfunction foo5(): { [key: string]: number, foo: string } {\n  return { foo: \"bar\" };\n}\n\n// If key exists in object type then indexer is not matched\nfunction foo6(): { [key: number]: number, foo: string } {\n  return { foo: \"bar\" };\n}\n\n// Should still complain about mistyped properties\nfunction foo7(): { [key: string]: number, foo: number } {\n  return { foo: \"bar\" };\n}");
}
#[test]
fn test_multiple_js_format_1_19c60e96() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  function() {\n    ({}: {\n      [k1: string]: string,\n      [k2: number]: number, // error: not supported (yet)\n    });\n  }\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  function () {\n    ({}: {\n      [k1: string]: string,\n      [k2: number]: number, // error: not supported (yet)\n    });\n  },\n];");
}
