#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_client_js_format_1_9502458d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Client imports some but not all modules,\n * triggering/suppressing parse errors.\n * @flow\n */\n\n// non-flow files should not show parse errors\nvar A = require(\"Foo\");          // non-Flow file @providesModule Foo\nvar B = require(\"./NoProvides\"); // non-Flow file\n\nvar C = require(\"./ParseError\"); // Flow file") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * Client imports some but not all modules,\n * triggering/suppressing parse errors.\n * @flow\n */\n\n// non-flow files should not show parse errors\nvar A = require(\"Foo\"); // non-Flow file @providesModule Foo\nvar B = require(\"./NoProvides\"); // non-Flow file\n\nvar C = require(\"./ParseError\"); // Flow file");
}
