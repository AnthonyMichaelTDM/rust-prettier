#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_nested_atrule_css_format_1_8b40146e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@media (min-width: 992px){@media (max-width: 1200px) {\n.container\n\n{position: absolute\n    }}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@media (min-width: 992px) {\n  @media (max-width: 1200px) {\n    .container {\n      position: absolute;\n    }\n  }\n}");
}
