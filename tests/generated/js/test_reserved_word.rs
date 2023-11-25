use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_interfaces_js_format_1_febe38f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo.interface;\ninterface.foo;\nnew interface();\n({ interface: \"foo\" });\n(interface, \"foo\");\nvoid interface;\nconst interface = \"foo\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo.interface;\ninterface.foo;\nnew interface();\n({ interface: \"foo\" });\ninterface, \"foo\";\nvoid interface;\nconst interface = \"foo\";");
}
