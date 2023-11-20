#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_use_scss_trailing_commanone_format_1_8b229369() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@use \"@fylgja/base\" with (\n  $family-main: (\n  Rubik,\n    Roboto,\n    system-ui,\n    sans-serif,\n  ),\n  $font-weight: 350,\n  $h1-font-weight: 500,\n  $h2-font-weight: 500,\n  $h3-font-weight: 500\n);\n\n@use 'library' with (\n  $black: #222,\n  $border-radius: 0.1rem\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@use \"@fylgja/base\" with (\n  $family-main: (\n    Rubik,\n    Roboto,\n    system-ui,\n    sans-serif\n  ),\n  $font-weight: 350,\n  $h1-font-weight: 500,\n  $h2-font-weight: 500,\n  $h3-font-weight: 500\n);\n\n@use \"library\" with (\n  $black: #222,\n  $border-radius: 0.1rem\n);");
}
#[test]
fn test_use_scss_format_1_8b229369() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@use \"@fylgja/base\" with (\n  $family-main: (\n  Rubik,\n    Roboto,\n    system-ui,\n    sans-serif,\n  ),\n  $font-weight: 350,\n  $h1-font-weight: 500,\n  $h2-font-weight: 500,\n  $h3-font-weight: 500\n);\n\n@use 'library' with (\n  $black: #222,\n  $border-radius: 0.1rem\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@use \"@fylgja/base\" with (\n  $family-main: (\n    Rubik,\n    Roboto,\n    system-ui,\n    sans-serif,\n  ),\n  $font-weight: 350,\n  $h1-font-weight: 500,\n  $h2-font-weight: 500,\n  $h3-font-weight: 500\n);\n\n@use \"library\" with (\n  $black: #222,\n  $border-radius: 0.1rem\n);");
}
