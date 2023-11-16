#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_attr_element_js_format_1_decadd9b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("<Foo prop=<Bar><Baz /></Bar> />;\n<Foo prop=<><Bar><Baz /></Bar></> />;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<Foo\n  prop=<Bar>\n    <Baz />\n  </Bar>\n/>;\n<Foo\n  prop=<>\n    <Bar>\n      <Baz />\n    </Bar>\n  </>\n/>;");
}
