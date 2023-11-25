#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_declarations_ts_arrow_parensavoid_format_1_d89393f6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("test(\"does something really long and complicated so I have to write a very long name for the test\", <T>(done) => {\n  console.log(\"hello!\");\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "test(\"does something really long and complicated so I have to write a very long name for the test\", <T>(done) => {\n  console.log(\"hello!\");\n});");
}
#[test]
fn test_test_declarations_ts_format_1_d89393f6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("test(\"does something really long and complicated so I have to write a very long name for the test\", <T>(done) => {\n  console.log(\"hello!\");\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "test(\"does something really long and complicated so I have to write a very long name for the test\", <T>(done) => {\n  console.log(\"hello!\");\n});");
}
