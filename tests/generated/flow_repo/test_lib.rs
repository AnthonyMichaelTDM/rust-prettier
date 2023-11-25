use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_libtest_js_format_1_48c6d610() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x:string = NaN\nvar y:string = Number.MAX_VALUE;\nvar z:number = new TypeError().name;\nvar w:string = parseInt(\"...\");\n\nvar a = new Map();\na.delete('foobar');\n\nvar b = undefined;\nif (undefined) {\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var x: string = NaN;\nvar y: string = Number.MAX_VALUE;\nvar z: number = new TypeError().name;\nvar w: string = parseInt(\"...\");\n\nvar a = new Map();\na.delete(\"foobar\");\n\nvar b = undefined;\nif (undefined) {\n}");
}
