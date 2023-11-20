#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basic_html_format_1_f29f88bf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template>\n  <i class.bind=\"icon\"></i>\n</template>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<template>\n  <i class.bind=\"icon\"></i>\n</template>"
    );
}
