#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_regexp_js_format_1_97139cf5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var patt=/Hello/g\nvar match:number = patt.test(\"Hello world!\");");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var patt = /Hello/g;\nvar match: number = patt.test(\"Hello world!\");"
    );
}
