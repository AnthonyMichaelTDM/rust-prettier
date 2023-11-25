#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_keywords_ts_format_1_f4c3e750() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("module Y4 {\n    enum Color { Blue, Red }\n}\n\nmodule YY3 {\n    module Module {\n        class A { s: string }\n    }\n}\n\nmodule YY4 {\n    enum Color { Blue, Red }\n}\n\nmodule YYY3 {\n    module Module {\n        class A { s: string }\n    }\n}\n\nmodule YYY4 {\n    enum Color { Blue, Red }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "module Y4 {\n  enum Color {\n    Blue,\n    Red,\n  }\n}\n\nmodule YY3 {\n  module Module {\n    class A {\n      s: string;\n    }\n  }\n}\n\nmodule YY4 {\n  enum Color {\n    Blue,\n    Red,\n  }\n}\n\nmodule YYY3 {\n  module Module {\n    class A {\n      s: string;\n    }\n  }\n}\n\nmodule YYY4 {\n  enum Color {\n    Blue,\n    Red,\n  }\n}");
}
#[test]
fn test_keywords_2_ts_format_1_449dad88() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/12356\n\nclass C {\n  private *a() {}\n  public *b() {}\n  static *c() {}\n  protected *g() {}\n}\n\nclass D {\n  declare<T>() {}\n  readonly<T>() {}\n  abstract<T>() {}\n  static<T>() {}\n  private<T>() {}\n  public<T>() {}\n  protected<T>() {}\n}\n\nclass E {\n  public\n  private() {}\n}\n\nclass Foo {\n  constructor(private, public, static) {\n  }\n}\n\nclass F {\n    constructor(public []) {}\n}\nclass G {\n    constructor(public {}) {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/12356\n\nclass C {\n  private *a() {}\n  public *b() {}\n  static *c() {}\n  protected *g() {}\n}\n\nclass D {\n  declare<T>() {}\n  readonly<T>() {}\n  abstract<T>() {}\n  static<T>() {}\n  private<T>() {}\n  public<T>() {}\n  protected<T>() {}\n}\n\nclass E {\n  public;\n  private() {}\n}\n\nclass Foo {\n  constructor(private, public, static) {}\n}\n\nclass F {\n  constructor(public []) {}\n}\nclass G {\n  constructor(public {}) {}\n}");
}
#[test]
fn test_module_ts_format_1_1e05c84a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("module Y3 {\n  module Module {\n      class A { s: string }\n  }\n\n  export enum X { }\n\n  interface x {\n      readonly [x: any]: any;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "module Y3 {\n  module Module {\n    class A {\n      s: string;\n    }\n  }\n\n  export enum X {}\n\n  interface x {\n    readonly [x: any]: any;\n  }\n}");
}
