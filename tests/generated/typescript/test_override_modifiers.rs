#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_override_modifier_ts_format_1_22def4bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class MyClass extends BaseClass {\n  override show() {}\n  public override show() {}\n  override size = 5;\n  override readonly size = 5;\n  abstract override foo: string;\n  static override foo: string;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class MyClass extends BaseClass {\n  override show() {}\n  public override show() {}\n  override size = 5;\n  override readonly size = 5;\n  abstract override foo: string;\n  static override foo: string;\n}");
}
#[test]
fn test_parameter_property_ts_format_1_3ce559e5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("class D extends B {\n  constructor(override foo: string) {\n    super();\n  }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class D extends B {\n  constructor(override foo: string) {\n    super();\n  }\n}"
    );
}
