#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_numbers_css_format_1_445e4573() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@supports (margin: .5px \".30px\" 1E+2px) {\n  a {\n    a: 0;\n    a: 1;\n\n    a: 0.1;\n    a: 1.1;\n\n    a: .1;\n    a: 1.;\n    a: +.1;\n    a: -.1;\n\n    a: 1e1;\n    a: 1e+1;\n    a: 1e-1;\n    a: 1.e1;\n    a: .1e1;\n    a: 1.1e1;\n    a: 1.1e0010;\n    a: +1.1e0010;\n    a: -1.1e0010;\n    a: .1e+0010;\n    a: .1e-0010;\n\n    a: 1E1;\n    a: 1E+1;\n    a: 1E-1;\n    a: 1.E1;\n    a: .1E1;\n    a: 1.1E1;\n    a: 1.1E0010;\n    a: .1E+0010;\n    a: .1E-0010;\n\n    a: 0.5e0;\n    a: 0.5e00;\n    a: 0.5e+0;\n    a: 0.5e+00;\n    a: 0.5e-0;\n    a: 0.5e-00;\n    a: +0.5e0;\n    a: -0.5e0;\n\n    a: 1;\n    a: 1.00500;\n    a: 1.0;\n    a: 1.5;\n    a: 1.50;\n\n    a: 0.00500;\n    a: 0.0;\n    a: 0.0000;\n\n    a: 500600.001230045000;\n    a: 1.00500e60;\n    a: 1.0e60;\n    a: 0.00500e60;\n    a: 0.0e60;\n    a: 0.0000e60;\n    a: .0e60;\n    a: 0.e60;\n    a: 0e60;\n    a: 500600.001230045000e60;\n    a: 10;\n    a: 9700;\n    a: 10e100;\n    height: attr(data-size em, .01);\n  }\n}\n\n@media only screen and (-webkit-min-device-pixel-ratio : .5), only screen and (min-device-pixel-ratio : .5) { }\n@include single-transition(transform, .5s ease);") ? ;
    assert_eq ! (formatted , "@supports (margin: 0.5px \".30px\" 1e2px) {\n  a {\n    a: 0;\n    a: 1;\n\n    a: 0.1;\n    a: 1.1;\n\n    a: 0.1;\n    a: 1;\n    a: +0.1;\n    a: -0.1;\n\n    a: 1e1;\n    a: 1e1;\n    a: 1e-1;\n    a: 1e1;\n    a: 0.1e1;\n    a: 1.1e1;\n    a: 1.1e10;\n    a: +1.1e10;\n    a: -1.1e10;\n    a: 0.1e10;\n    a: 0.1e-10;\n\n    a: 1e1;\n    a: 1e1;\n    a: 1e-1;\n    a: 1e1;\n    a: 0.1e1;\n    a: 1.1e1;\n    a: 1.1e10;\n    a: 0.1e10;\n    a: 0.1e-10;\n\n    a: 0.5;\n    a: 0.5;\n    a: 0.5;\n    a: 0.5;\n    a: 0.5;\n    a: 0.5;\n    a: +0.5;\n    a: -0.5;\n\n    a: 1;\n    a: 1.005;\n    a: 1;\n    a: 1.5;\n    a: 1.5;\n\n    a: 0.005;\n    a: 0;\n    a: 0;\n\n    a: 500600.001230045;\n    a: 1.005e60;\n    a: 1e60;\n    a: 0.005e60;\n    a: 0e60;\n    a: 0e60;\n    a: 0e60;\n    a: 0e60;\n    a: 0e60;\n    a: 500600.001230045e60;\n    a: 10;\n    a: 9700;\n    a: 10e100;\n    height: attr(data-size em, 0.01);\n  }\n}\n\n@media only screen and (-webkit-min-device-pixel-ratio: 0.5),\n  only screen and (min-device-pixel-ratio: 0.5) {\n}\n@include single-transition(transform, 0.5s ease);");
    Ok(())
}
