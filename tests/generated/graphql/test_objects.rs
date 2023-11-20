#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_objects_graphql_format_1_4f6cda0e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["graphql"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n  shortWithObj(obj: { hello: \"world\", x: 5 })\n  multilineObj(obj: {\n    hello: \"world\",\n    x: 5\n  })\n  longWithObj(obj: { ,,longString: \"hello world this is a very long string!\", list: [1, 2, 3, 4, 5, 6, 7] })\n  emptyObj(arg: {})\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  shortWithObj(obj: { hello: \"world\", x: 5 })\n  multilineObj(obj: { hello: \"world\", x: 5 })\n  longWithObj(\n    obj: {\n      longString: \"hello world this is a very long string!\"\n      list: [1, 2, 3, 4, 5, 6, 7]\n    }\n  )\n  emptyObj(arg: {})\n}");
}
