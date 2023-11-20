#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_fields_graphql_format_1_6af4822f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["graphql"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("query MyFirstQuery {\n  hello\n}\n\nmutation\nMyFirstMutation {\n  world\n}\n\nsubscription, ThisIsASub, {\n  excellent\n}\n\n, query, ThisIsASub, {\n  excellent\n}\n\nquery {\n  noName\n}\n\n{\n  noOperationType\n}\n\nquery ($unnamed: String) {\n  id\n}\n\nquery Named($var: String) {\n  id\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "query MyFirstQuery {\n  hello\n}\n\nmutation MyFirstMutation {\n  world\n}\n\nsubscription ThisIsASub {\n  excellent\n}\n\nquery ThisIsASub {\n  excellent\n}\n\nquery {\n  noName\n}\n\n{\n  noOperationType\n}\n\nquery ($unnamed: String) {\n  id\n}\n\nquery Named($var: String) {\n  id\n}");
}
