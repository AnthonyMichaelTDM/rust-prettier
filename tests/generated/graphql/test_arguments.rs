use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_hello_graphql_format_1_85646c06() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("graphql")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n  short(var: $var, int: 5, float: 5.6, bool: true, string: \"hello world!\")\n  long(var: $thisIsAReallyLongVariableNameRight, int: 52342342342, float: 5.6, bool: true, string: \"hello world this is a very long string!\")\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  short(var: $var, int: 5, float: 5.6, bool: true, string: \"hello world!\")\n  long(\n    var: $thisIsAReallyLongVariableNameRight\n    int: 52342342342\n    float: 5.6\n    bool: true\n    string: \"hello world this is a very long string!\"\n  )\n}");
}
