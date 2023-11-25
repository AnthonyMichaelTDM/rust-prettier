#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_trailing_commas_js_trailing_commaall_format_1_b2017e29() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n  f(\n    superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n    ...args\n  ) {}\n}\n\nfunction f(\n  superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n  ...args\n) {}\n\nclass D { f(...superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong) {}; }\n\n[superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,,];\n\n[veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong, ...a] = [];\nvar {veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong, ...a} = {};\n") ? ;
    assert_eq ! (formatted , "class C {\n  f(\n    superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n    ...args\n  ) {}\n}\n\nfunction f(\n  superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n  ...args\n) {}\n\nclass D {\n  f(\n    ...superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong\n  ) {}\n}\n\n[\n  superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n  ,\n];\n\n[\n  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong,\n  ...a\n] = [];\nvar {\n  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong,\n  ...a\n} = {};");
    Ok(())
}
