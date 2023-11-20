#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_spread_js_format_1_f52b57a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare var iterableOf123: Iterable<123>;\nfunction fun(x: 'hi', y: 123) {}\nfun('hi', ...iterableOf123); // No error - ignore the fact iterableOf123 could be empty\n\nfunction funWithRestArray(x: 'hi', y: 123, ...rest: Array<number>) {}\n\nfunWithRestArray('hi', 123, ...iterableOf123); // Ok\nfunWithRestArray('hi', ...iterableOf123); // No error - ignore the fact iterableOf123 could be empty\nfunWithRestArray('hi', ...iterableOf123, ...iterableOf123); // No error - ignore the fact iterableOf123 could be empty\n\n// 2 errors\n// 1. 'bye' ~> 123 in case the first spread is empty\n// 2. 'bye' ~> number in case the first spread is not empty\nfunWithRestArray('hi', ...iterableOf123, 'bye', ...iterableOf123);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare var iterableOf123: Iterable<123>;\nfunction fun(x: \"hi\", y: 123) {}\nfun(\"hi\", ...iterableOf123); // No error - ignore the fact iterableOf123 could be empty\n\nfunction funWithRestArray(x: \"hi\", y: 123, ...rest: Array<number>) {}\n\nfunWithRestArray(\"hi\", 123, ...iterableOf123); // Ok\nfunWithRestArray(\"hi\", ...iterableOf123); // No error - ignore the fact iterableOf123 could be empty\nfunWithRestArray(\"hi\", ...iterableOf123, ...iterableOf123); // No error - ignore the fact iterableOf123 could be empty\n\n// 2 errors\n// 1. 'bye' ~> 123 in case the first spread is empty\n// 2. 'bye' ~> number in case the first spread is not empty\nfunWithRestArray(\"hi\", ...iterableOf123, \"bye\", ...iterableOf123);");
}
