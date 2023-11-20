#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_forof_js_format_1_0a032bcc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nfunction testArray(arr: Array<number>): void {\n  for (var x of arr) {\n    (x: string); // Error - number ~> string\n  }\n}\n\nfunction testIterable1(iterable: Iterable<number>): void {\n  for (var x of iterable) {\n    (x: string); // Error - number ~> string\n  }\n}\n\nfunction testIterable2(iterable: Iterable<*>): void {\n  for (var x of iterable) {\n    (x: string);\n  }\n}\n\nfunction testString(str: string): void {\n  for (var x of str) {\n    (x: number); // Error - string ~> number\n  }\n}\n\nfunction testMap1(map: Map<string, number>): void {\n  for (var elem of map) {\n    (elem: [string, number]);\n    (elem: number); // Error - tuple ~> number\n  }\n}\n\nfunction testMap2(map: Map<*, *>): void {\n  for (var elem of map) {\n    (elem: [number, string]); // Any tuple is fine\n    (elem: number); // Error - tuple ~> number\n  }\n}\n\nfunction testSet1(set: Set<string>): void {\n  for (var x of set) {\n    (x: number); // Error - string ~> number\n  }\n}\n\nfunction testSet2(set: Set<*>): void {\n  for (var x of set) {\n    (x: number); // Anything goes\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nfunction testArray(arr: Array<number>): void {\n  for (var x of arr) {\n    (x: string); // Error - number ~> string\n  }\n}\n\nfunction testIterable1(iterable: Iterable<number>): void {\n  for (var x of iterable) {\n    (x: string); // Error - number ~> string\n  }\n}\n\nfunction testIterable2(iterable: Iterable<*>): void {\n  for (var x of iterable) {\n    (x: string);\n  }\n}\n\nfunction testString(str: string): void {\n  for (var x of str) {\n    (x: number); // Error - string ~> number\n  }\n}\n\nfunction testMap1(map: Map<string, number>): void {\n  for (var elem of map) {\n    (elem: [string, number]);\n    (elem: number); // Error - tuple ~> number\n  }\n}\n\nfunction testMap2(map: Map<*, *>): void {\n  for (var elem of map) {\n    (elem: [number, string]); // Any tuple is fine\n    (elem: number); // Error - tuple ~> number\n  }\n}\n\nfunction testSet1(set: Set<string>): void {\n  for (var x of set) {\n    (x: number); // Error - string ~> number\n  }\n}\n\nfunction testSet2(set: Set<*>): void {\n  for (var x of set) {\n    (x: number); // Anything goes\n  }\n}");
}
