#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_lists_graphql_format_1_319fdd43() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["graphql"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n  shortWithList(list: [1, 2, 3])\n  longWithList(list: [\"hello world this is a very long string!\",\"hello world this is a very long string!\", \"hello world this is a very long string!\"])\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  shortWithList(list: [1, 2, 3])\n  longWithList(\n    list: [\n      \"hello world this is a very long string!\"\n      \"hello world this is a very long string!\"\n      \"hello world this is a very long string!\"\n    ]\n  )\n}");
}
