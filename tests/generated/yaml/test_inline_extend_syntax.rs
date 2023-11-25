#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_inline_extend_syntax_yml_format_1_0b22cbc0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# 8876\n\nfoo:\n <<: &anchor\n   K1: \"One\"\n K2: \"Two\"\n\nbar:\n <<: *anchor\n K3: \"Three\"") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# 8876\n\nfoo:\n  <<: &anchor\n    K1: \"One\"\n  K2: \"Two\"\n\nbar:\n  <<: *anchor\n  K3: \"Three\"");
}
