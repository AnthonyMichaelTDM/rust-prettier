#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_param_js_trailing_commaall_format_1_cd1d258e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f = ({}: MyVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongType) => {};\nfunction g({}: Foo) {}") ? ;
    assert_eq ! (formatted , "const f =\n  ({}: MyVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongType) => {};\nfunction g({}: Foo) {}");
    Ok(())
}
#[test]
fn test_param_js_trailing_commaes_5_format_1_cd1d258e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f = ({}: MyVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongType) => {};\nfunction g({}: Foo) {}") ? ;
    assert_eq ! (formatted , "const f =\n  ({}: MyVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongType) => {};\nfunction g({}: Foo) {}");
    Ok(())
}
