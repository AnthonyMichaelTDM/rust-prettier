#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_variable_definitions_graphql_format_1_534f76dc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["graphql"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("query short($foo:ComplexType, $site   : Site =         MOBILE,     $nonNull: Int!) {\n  hello\n}\n\nquery long($foo: ComplexType, $site: Float = 124241.12312,\n$bar: String = \"Long string here\", $arg: String = \"Hello world!\",,,,,\n$nonNull: String!) {\n  hello\n}\n\nquery lists($foo: [Int  ], $bar:    [Int!], $arg: [ Int! ]!) {\n  ok\n}\n\nquery listslong($foo: [String  ], $bar:    [String!], $arg: [ Int! ]!, $veryLongName: [ Int! ]) {\n  ok\n}\n\nquery withvariabledirective($foo:   Int   @directive) {\n  ok\n}\n\nquery withvariabledirectives($foo:   Int   @directive   @another) {\n  ok\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "query short($foo: ComplexType, $site: Site = MOBILE, $nonNull: Int!) {\n  hello\n}\n\nquery long(\n  $foo: ComplexType\n  $site: Float = 124241.12312\n  $bar: String = \"Long string here\"\n  $arg: String = \"Hello world!\"\n  $nonNull: String!\n) {\n  hello\n}\n\nquery lists($foo: [Int], $bar: [Int!], $arg: [Int!]!) {\n  ok\n}\n\nquery listslong(\n  $foo: [String]\n  $bar: [String!]\n  $arg: [Int!]!\n  $veryLongName: [Int!]\n) {\n  ok\n}\n\nquery withvariabledirective($foo: Int @directive) {\n  ok\n}\n\nquery withvariabledirectives($foo: Int @directive @another) {\n  ok\n}");
}
