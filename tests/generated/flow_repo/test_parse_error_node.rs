#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_client_js_format_1_e19d7c18() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Client imports some but not all modules,\n * triggering/suppressing parse errors.\n * @flow\n */\n\n// non-flow files should not give parse errors\nvar A = require(\"./Imported\");          // non-Flow file @providesModule Foo\n\nvar B = require(\"./ParseError\");        // Flow file") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * Client imports some but not all modules,\n * triggering/suppressing parse errors.\n * @flow\n */\n\n// non-flow files should not give parse errors\nvar A = require(\"./Imported\"); // non-Flow file @providesModule Foo\n\nvar B = require(\"./ParseError\"); // Flow file");
}
