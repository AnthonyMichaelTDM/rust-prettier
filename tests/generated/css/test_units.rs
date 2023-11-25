#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_values_css_format_1_4a1ec139() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@value 4XLarge 28/36px;\n\n.postCssLowerCasingValueName {\n  font: 4XLarge Helvetica;\n}\n\n.cssUnits {\n  a: 5EM;\n  a: 5REM;\n  a: 5EX;\n  a: 5REX;\n  a: 5CAP;\n  a: 5RCAP;\n  a: 5CH;\n  a: 5RCH;\n  a: 5IC;\n  a: 5RIC;\n  a: 5LH;\n  a: 5RLH;\n  a: 5VH;\n  a: 5VW;\n  a: 5VI;\n  a: 5VB;\n  a: 5VMIN;\n  a: 5VMAX;\n  a: 5CM;\n  a: 5MM;\n  a: 5q;\n  a: 5IN;\n  a: 5PT;\n  a: 5PC;\n  a: 5PX;\n  a: 5DEG;\n  a: 5GRAD;\n  a: 5RAD;\n  a: 5S;\n  a: 5MS;\n  a: 5hZ;\n  a: 5KhZ;\n  a: 5DPI;\n  a: 5DPCM;\n  a: 5DPPX;\n  a: 5X;\n\n  a: 5PROTOTYPE;\n  a: 5constructoR;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@value 4XLarge 28/36px;\n\n.postCssLowerCasingValueName {\n  font: 4XLarge Helvetica;\n}\n\n.cssUnits {\n  a: 5em;\n  a: 5rem;\n  a: 5ex;\n  a: 5rex;\n  a: 5cap;\n  a: 5rcap;\n  a: 5ch;\n  a: 5rch;\n  a: 5ic;\n  a: 5ric;\n  a: 5lh;\n  a: 5rlh;\n  a: 5vh;\n  a: 5vw;\n  a: 5vi;\n  a: 5vb;\n  a: 5vmin;\n  a: 5vmax;\n  a: 5cm;\n  a: 5mm;\n  a: 5Q;\n  a: 5in;\n  a: 5pt;\n  a: 5pc;\n  a: 5px;\n  a: 5deg;\n  a: 5grad;\n  a: 5rad;\n  a: 5s;\n  a: 5ms;\n  a: 5Hz;\n  a: 5kHz;\n  a: 5dpi;\n  a: 5dpcm;\n  a: 5dppx;\n  a: 5x;\n\n  a: 5PROTOTYPE;\n  a: 5constructoR;\n}");
}
