#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_date_js_format_1_785adfce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var d = new Date(0);\nvar x:string = d.getTime();\n\nvar y:number = d;\n\n// valid constructors\nnew Date();\nnew Date(1234567890);\nnew Date('2015/06/18');\nnew Date(2015, 6);\nnew Date(2015, 6, 18);\nnew Date(2015, 6, 18, 11);\nnew Date(2015, 6, 18, 11, 55);\nnew Date(2015, 6, 18, 11, 55, 42);\nnew Date(2015, 6, 18, 11, 55, 42, 999);\n\n// invalid constructors\nnew Date({});\nnew Date(2015, '6');\nnew Date(2015, 6, '18');\nnew Date(2015, 6, 18, '11');\nnew Date(2015, 6, 18, 11, '55');\nnew Date(2015, 6, 18, 11, 55, '42');\nnew Date(2015, 6, 18, 11, 55, 42, '999');\nnew Date(2015, 6, 18, 11, 55, 42, 999, 'hahaha');\nnew Date('2015', 6);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var d = new Date(0);\nvar x: string = d.getTime();\n\nvar y: number = d;\n\n// valid constructors\nnew Date();\nnew Date(1234567890);\nnew Date(\"2015/06/18\");\nnew Date(2015, 6);\nnew Date(2015, 6, 18);\nnew Date(2015, 6, 18, 11);\nnew Date(2015, 6, 18, 11, 55);\nnew Date(2015, 6, 18, 11, 55, 42);\nnew Date(2015, 6, 18, 11, 55, 42, 999);\n\n// invalid constructors\nnew Date({});\nnew Date(2015, \"6\");\nnew Date(2015, 6, \"18\");\nnew Date(2015, 6, 18, \"11\");\nnew Date(2015, 6, 18, 11, \"55\");\nnew Date(2015, 6, 18, 11, 55, \"42\");\nnew Date(2015, 6, 18, 11, 55, 42, \"999\");\nnew Date(2015, 6, 18, 11, 55, 42, 999, \"hahaha\");\nnew Date(\"2015\", 6);");
}
