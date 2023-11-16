#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_array_js_bracket_spacingfalse_format_1_4993a647() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("const arr1 = [1,2,3,4];\nconst arr2 = [1, 2, 3, 4];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const arr1 = [1, 2, 3, 4];\nconst arr2 = [1, 2, 3, 4];"
    );
}
#[test]
fn test_array_js_format_1_4993a647() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("const arr1 = [1,2,3,4];\nconst arr2 = [1, 2, 3, 4];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const arr1 = [1, 2, 3, 4];\nconst arr2 = [1, 2, 3, 4];"
    );
}
#[test]
fn test_object_js_bracket_spacingfalse_format_1_b69240e7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("const obj1 = {a:1, b:2, c:3}\nconst obj2 = { a:1, b:2, c:3 };");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const obj1 = {a: 1, b: 2, c: 3};\nconst obj2 = {a: 1, b: 2, c: 3};"
    );
}
#[test]
fn test_object_js_format_1_b69240e7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("const obj1 = {a:1, b:2, c:3}\nconst obj2 = { a:1, b:2, c:3 };");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const obj1 = { a: 1, b: 2, c: 3 };\nconst obj2 = { a: 1, b: 2, c: 3 };"
    );
}
