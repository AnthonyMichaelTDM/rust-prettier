#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_import_meta_js_format_1_b805bc8c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const x = import.meta;\nconst url = import.meta.url;\nimport.meta;\nimport.meta.url;\nimport.meta.couldBeMutable = true;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const x = import.meta;\nconst url = import.meta.url;\nimport.meta;\nimport.meta.url;\nimport.meta.couldBeMutable = true;");
}
