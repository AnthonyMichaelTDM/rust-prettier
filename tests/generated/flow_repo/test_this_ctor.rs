#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_98eb64e7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  n: number;\n  constructor(n: number) {\n    this.n = n;\n  }\n  clone(): A {\n    return new this.constructor(this.n);\n  }\n  badClone(): number {\n    return new this.constructor(this.n); // Error A ~> number\n  }\n}\n\nvar a1 = new A(1);\nvar a2: A = new a1.constructor(2);\nvar a3: A = a2.clone();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  n: number;\n  constructor(n: number) {\n    this.n = n;\n  }\n  clone(): A {\n    return new this.constructor(this.n);\n  }\n  badClone(): number {\n    return new this.constructor(this.n); // Error A ~> number\n  }\n}\n\nvar a1 = new A(1);\nvar a2: A = new a1.constructor(2);\nvar a3: A = a2.clone();");
}
