#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_decorators_ts_format_1_09c76336() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/typescript-eslint/typescript-eslint/pull/4382\nfunction decorator() {}\n@decorator\nclass Foo {\n  bar(baz: typeof this) {}\n}") ? ;
    assert_eq ! (formatted , "// https://github.com/typescript-eslint/typescript-eslint/pull/4382\nfunction decorator() {}\n@decorator\nclass Foo {\n  bar(baz: typeof this) {}\n}");
    Ok(())
}
#[test]
fn test_typeof_this_ts_format_1_842b9adb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/typescript-eslint/typescript-eslint/pull/4382\nlet self: typeof this;\nlet foo: typeof this.foo;") ? ;
    assert_eq ! (formatted , "// https://github.com/typescript-eslint/typescript-eslint/pull/4382\nlet self: typeof this;\nlet foo: typeof this.foo;");
    Ok(())
}
