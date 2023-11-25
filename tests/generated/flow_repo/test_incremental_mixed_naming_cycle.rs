#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_c265ff23() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule A\n * @flow\n */\n\n(require('./b'): void);\n(require('C'): void);\n\nmodule.exports = 'A';") ? ;
    assert_eq ! (formatted , "/**\n * @providesModule A\n * @flow\n */\n\n(require(\"./b\"): void);\n(require(\"C\"): void);\n\nmodule.exports = \"A\";");
    Ok(())
}
#[test]
fn test_b_js_format_1_4938203d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule B\n * @flow\n */\n\n(require('A'): void);\n(require('D'): void);\n\nmodule.exports = 'B';") ? ;
    assert_eq ! (formatted , "/**\n * @providesModule B\n * @flow\n */\n\n(require(\"A\"): void);\n(require(\"D\"): void);\n\nmodule.exports = \"B\";");
    Ok(())
}
#[test]
fn test_c_js_format_1_14d1835b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule C\n * @flow\n */\n\nrequire('Root');\n(require('./b'): void);\n\nmodule.exports = 'C';") ? ;
    assert_eq ! (formatted , "/**\n * @providesModule C\n * @flow\n */\n\nrequire(\"Root\");\n(require(\"./b\"): void);\n\nmodule.exports = \"C\";");
    Ok(())
}
#[test]
fn test_d_js_format_1_14dbbd17() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule D\n * @flow\n */\n\n(require('./b'): void);\n\nmodule.exports = 'D';") ? ;
    assert_eq ! (formatted , "/**\n * @providesModule D\n * @flow\n */\n\n(require(\"./b\"): void);\n\nmodule.exports = \"D\";");
    Ok(())
}
#[test]
fn test_root_js_format_1_2ee46486() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule Root\n * @flow\n */\n\nmodule.exports = 'Root';")?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule Root\n * @flow\n */\n\nmodule.exports = \"Root\";"
    );
    Ok(())
}
