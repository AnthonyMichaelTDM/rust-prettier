#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_var_func_css_format_1_9b8ffdb3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["css"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".foo {\n\t--bar: var(--baz,);\n  --bar: var(--baz     ,);\n  --bar: var(--baz     ,    );\n  --bar: var(--baz,);\n\t--bar: var(   --baz1, --baz2    , );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".foo {\n  --bar: var(--baz,);\n  --bar: var(--baz,);\n  --bar: var(--baz,);\n  --bar: var(--baz,);\n  --bar: var(--baz1, --baz2,);\n}");
}
