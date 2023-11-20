#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_enum_js_format_1_5bf4e24c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "var APIKeys = {\n    AGE: 'age',\n    NAME: 'name',\n};\n\nmodule.exports = APIKeys;",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var APIKeys = {\n  AGE: \"age\",\n  NAME: \"name\",\n};\n\nmodule.exports = APIKeys;"
    );
}
#[test]
fn test_enum_client_js_format_1_0ea414cd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var APIKeys = require('./enum');\n// object that maps \"AGE\" to \"age\", \"NAME\" to \"name\"\n\nfunction foo(x: $Keys<typeof APIKeys>) { }\nfoo(\"AGE\");\nfoo(\"LOCATION\"); // error\n\nfunction bar(x: $Keys<{age: number}>) { }\nbar(APIKeys.AGE); // not an error: APIKeys.AGE = \"age\"\nbar(APIKeys.NAME); // error: since \"NAME\" is not in the smaller enum\n\nvar object = {};\nobject[APIKeys.AGE] = 123; // i.e., object.age = 123\nobject[APIKeys.NAME] = \"FOO\"; // i.e., object.name = \"FOO\"\n\nvar age:number = object[APIKeys.AGE];\nvar name:number = object[APIKeys.NAME]; // error: object.name is a string\n\nvar indices = { red: 0, green: 1, blue: 2 };\nvar tuple = [42, \"hello\", false];\nvar red:string = tuple[indices.red]; // error: tuple[0] is a number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var APIKeys = require(\"./enum\");\n// object that maps \"AGE\" to \"age\", \"NAME\" to \"name\"\n\nfunction foo(x: $Keys<typeof APIKeys>) {}\nfoo(\"AGE\");\nfoo(\"LOCATION\"); // error\n\nfunction bar(x: $Keys<{ age: number }>) {}\nbar(APIKeys.AGE); // not an error: APIKeys.AGE = \"age\"\nbar(APIKeys.NAME); // error: since \"NAME\" is not in the smaller enum\n\nvar object = {};\nobject[APIKeys.AGE] = 123; // i.e., object.age = 123\nobject[APIKeys.NAME] = \"FOO\"; // i.e., object.name = \"FOO\"\n\nvar age: number = object[APIKeys.AGE];\nvar name: number = object[APIKeys.NAME]; // error: object.name is a string\n\nvar indices = { red: 0, green: 1, blue: 2 };\nvar tuple = [42, \"hello\", false];\nvar red: string = tuple[indices.red]; // error: tuple[0] is a number");
}
#[test]
fn test_number_js_format_1_86cca418() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function test1(x: number): number {\n  return -x;\n}\n\nfunction test2(x: string): number {\n  return -x;\n}\n\n// sanity checks to make sure merging envs doesn't keep creating new NumT's\n// because of the UnaryMinusT's, causing nontermination\nfunction test3(x: number, flip_times: number): number {\n  for (var i = 0; i < flip_times; i++) {\n    x = -x;\n  }\n  return x;\n}\nfunction test4(flip_times: number): number {\n  var x = 1;\n  for (var i = 0; i < flip_times; i++) {\n    x = -x;\n  }\n  return x;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function test1(x: number): number {\n  return -x;\n}\n\nfunction test2(x: string): number {\n  return -x;\n}\n\n// sanity checks to make sure merging envs doesn't keep creating new NumT's\n// because of the UnaryMinusT's, causing nontermination\nfunction test3(x: number, flip_times: number): number {\n  for (var i = 0; i < flip_times; i++) {\n    x = -x;\n  }\n  return x;\n}\nfunction test4(flip_times: number): number {\n  var x = 1;\n  for (var i = 0; i < flip_times; i++) {\n    x = -x;\n  }\n  return x;\n}");
}
