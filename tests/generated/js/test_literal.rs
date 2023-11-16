#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_number_js_format_1_22c54130() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// parentheses around numeric literal should be preserved\nfunction test5() {\n  return (100).toString();\n}\n\n0\n1\n\n0.1\n1.1\n\n.1\n1.\n\n0b1\n0B1\n0o1\n0O1\n0x1\n0X1\n\n0x123abcdef456ABCDEF\n0X123abcdef456ABCDEF\n0xdeadbeef;\n\n0b111000\n0b000111\n0B111000\n0B000111\n0o111000\n0o000111\n0O111000\n0O000111\n0x111000\n0x000111\n0X111000\n0X000111\n\n1e1\n1e+1\n1e-1\n1.e1\n.1e1\n1.1e1\n1.1e0010\n.1e+0010\n.1e-0010\n\n1E1\n1E+1\n1E-1\n1.E1\n.1E1\n1.1E1\n1.1E0010\n.1E+0010\n.1E-0010\n\n0.5e0\n0.5e00\n0.5e+0\n0.5e+00\n0.5e-0\n0.5e-00\n\n1\n1.00500\n1.0\n1.5\n1.50\n0\n0.00500\n0.0\n0.0000\n.0\n500600.001230045000\n1.00500e60\n1.0e60\n0.00500e60\n0.0e60\n0.0000e60\n.0e60\n0.e60\n0e60\n500600.001230045000e60\n10\n9700\n10e100") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// parentheses around numeric literal should be preserved\nfunction test5() {\n  return (100).toString();\n}\n\n0;\n1;\n\n0.1;\n1.1;\n\n0.1;\n1;\n\n0b1;\n0b1;\n0o1;\n0o1;\n0x1;\n0x1;\n\n0x123abcdef456abcdef;\n0x123abcdef456abcdef;\n0xdeadbeef;\n\n0b111000;\n0b000111;\n0b111000;\n0b000111;\n0o111000;\n0o000111;\n0o111000;\n0o000111;\n0x111000;\n0x000111;\n0x111000;\n0x000111;\n\n1e1;\n1e1;\n1e-1;\n1e1;\n0.1e1;\n1.1e1;\n1.1e10;\n0.1e10;\n0.1e-10;\n\n1e1;\n1e1;\n1e-1;\n1e1;\n0.1e1;\n1.1e1;\n1.1e10;\n0.1e10;\n0.1e-10;\n\n0.5;\n0.5;\n0.5;\n0.5;\n0.5;\n0.5;\n\n1;\n1.005;\n1.0;\n1.5;\n1.5;\n0;\n0.005;\n0.0;\n0.0;\n0.0;\n500600.001230045;\n1.005e60;\n1.0e60;\n0.005e60;\n0.0e60;\n0.0e60;\n0.0e60;\n0e60;\n0e60;\n500600.001230045e60;\n10;\n9700;\n10e100;");
}
