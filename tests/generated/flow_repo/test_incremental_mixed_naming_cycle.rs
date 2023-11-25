use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_c265ff23() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule A\n * @flow\n */\n\n(require('./b'): void);\n(require('C'): void);\n\nmodule.exports = 'A';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule A\n * @flow\n */\n\n(require(\"./b\"): void);\n(require(\"C\"): void);\n\nmodule.exports = \"A\";");
}
#[test]
fn test_b_js_format_1_4938203d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule B\n * @flow\n */\n\n(require('A'): void);\n(require('D'): void);\n\nmodule.exports = 'B';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule B\n * @flow\n */\n\n(require(\"A\"): void);\n(require(\"D\"): void);\n\nmodule.exports = \"B\";");
}
#[test]
fn test_c_js_format_1_14d1835b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule C\n * @flow\n */\n\nrequire('Root');\n(require('./b'): void);\n\nmodule.exports = 'C';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule C\n * @flow\n */\n\nrequire(\"Root\");\n(require(\"./b\"): void);\n\nmodule.exports = \"C\";");
}
#[test]
fn test_d_js_format_1_14dbbd17() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule D\n * @flow\n */\n\n(require('./b'): void);\n\nmodule.exports = 'D';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule D\n * @flow\n */\n\n(require(\"./b\"): void);\n\nmodule.exports = \"D\";");
}
#[test]
fn test_root_js_format_1_2ee46486() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule Root\n * @flow\n */\n\nmodule.exports = 'Root';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @providesModule Root\n * @flow\n */\n\nmodule.exports = \"Root\";"
    );
}
