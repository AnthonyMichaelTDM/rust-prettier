#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_trailing_commas_js_trailing_commaall_format_1_b2017e29() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("class C {\n  f(\n    superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n    ...args\n  ) {}\n}\n\nfunction f(\n  superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n  ...args\n) {}\n\nclass D { f(...superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong) {}; }\n\n[superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,,];\n\n[veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong, ...a] = [];\nvar {veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong, ...a} = {};\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  f(\n    superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n    ...args\n  ) {}\n}\n\nfunction f(\n  superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n  ...args\n) {}\n\nclass D {\n  f(\n    ...superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong\n  ) {}\n}\n\n[\n  superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,\n  ,\n];\n\n[\n  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong,\n  ...a\n] = [];\nvar {\n  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong,\n  ...a\n} = {};");
}
