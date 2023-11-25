#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_generic_break_js_format_1_e004eafc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n\ta: B<> = SuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class X {\n  a: B<> =\n    SuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong;\n}");
}
#[test]
fn test_simple_types_js_format_1_1d8dce93() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const foo1: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<bigint> = a;\nconst foo2: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<1n> = a;\nconst foo3: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<1_1n> = a;\nconst foo4: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<0xDeeD_BeeFn> = a;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo1: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<bigint> =\n  a;\nconst foo2: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<1n> =\n  a;\nconst foo3: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<1_1n> =\n  a;\nconst foo4: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<0xdeed_beefn> =\n  a;");
}
