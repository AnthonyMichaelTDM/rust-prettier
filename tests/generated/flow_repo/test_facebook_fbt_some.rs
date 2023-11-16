#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_main_js_format_1_4ccfaf2d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n(<fbt />: number);\n(<fbt />: string); // Error (the libdef in this test marks fbt as number)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n(<fbt />: number);\n(<fbt />: string); // Error (the libdef in this test marks fbt as number)");
}
