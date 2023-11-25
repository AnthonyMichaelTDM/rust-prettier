#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_optional_method_ts_format_1_23df8be0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// #6678\n\nclass Foo {\n  [bar.bar]?() {}\n}\n\n// https://github.com/prettier/prettier/issues/6569#issuecomment-542888410\nconst s = Symbol();\nclass A {\n  protected [s]?() {\n\n  }\n}") ? ;
    assert_eq ! (formatted , "// #6678\n\nclass Foo {\n  [bar.bar]?() {}\n}\n\n// https://github.com/prettier/prettier/issues/6569#issuecomment-542888410\nconst s = Symbol();\nclass A {\n  protected [s]?() {}\n}");
    Ok(())
}
