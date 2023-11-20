#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_fragment_variables_graphql_format_1_2a7a7965() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["graphql"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("fragment F($var: Int) on Type {\n  field\n}\n\nfragment G($first_variable: Int,   $second_variable: String =  \"Very complex default argument value\") on Type {\n  field\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "fragment F($var: Int) on Type {\n  field\n}\n\nfragment G(\n  $first_variable: Int\n  $second_variable: String = \"Very complex default argument value\"\n) on Type {\n  field\n}");
}
