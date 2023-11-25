use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_values_css_format_1_e869138c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".class {\n  background:linear-gradient(to bottom right,white,hsla(0,0%,100%,.8));\n  border: 1px      solid rgba(0,0,0,.3);\n  font-family: Arial  ,  sans-serif;\n  color: rgba(0, 0, 0, 1);\n  margin: 0   20px  0 -24px;\n  background-position:20% -26px;\n  transform-style: preserve-3d;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".class {\n  background: linear-gradient(to bottom right, white, hsla(0, 0%, 100%, 0.8));\n  border: 1px solid rgba(0, 0, 0, 0.3);\n  font-family: Arial, sans-serif;\n  color: rgba(0, 0, 0, 1);\n  margin: 0 20px 0 -24px;\n  background-position: 20% -26px;\n  transform-style: preserve-3d;\n}");
}
