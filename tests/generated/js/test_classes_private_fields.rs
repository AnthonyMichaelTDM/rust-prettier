use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_optional_chaining_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_optional_chaining_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_optional_chaining_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_optional_chaining_js_semifalse_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_optional_chaining_js_semifalse_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_optional_chaining_js_semifalse_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_optional_chaining_js_semifalse_format_1_85dd90aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// https://github.com/babel/babel/pull/11669\n\ndelete obj?.#x.a");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// https://github.com/babel/babel/pull/11669\n\ndelete obj?.#x.a"
    );
}
#[test]
fn test_optional_chaining_js_format_1_85dd90aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// https://github.com/babel/babel/pull/11669\n\ndelete obj?.#x.a");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// https://github.com/babel/babel/pull/11669\n\ndelete obj?.#x.a;"
    );
}
#[test]
fn test_private_fields_js_semifalse_format_1_6ef63b38() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A { #x; #y; }\nclass B { #x = 0; #y = 1; }\n\nclass C {\n  static #x;\n  static #y = 1;\n}\n\nclass D {\n  #x\n  #y\n}\n\nclass Point {\n  #x = 1;\n  #y = 2;\n\n  constructor(x = 0, y = 0) {\n    this.#x = +x;\n    this.#y = +y;\n  }\n\n  get x() { return this.#x }\n  set x(value) { this.#x = +value }\n\n  get y() { return this.#y }\n  set y(value) { this.#y = +value }\n\n  equals(p) { return this.#x === p.#x && this.#y === p.#y }\n\n  toString() { return \\`Point<\\${ this.#x },\\${ this.#y }>\\` }\n}\n\nclass E {\n  async #a() {}\n  #b() {}\n  get #c() {}\n  set #c(bar) {}\n  *#d() {}\n  async *#e() {}\n  get #f() {}\n  set #f(taz) {}\n}\n\nclass F {\n  #func(id, { blog: { title } }) {\n    return id + title;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  #x\n  #y\n}\nclass B {\n  #x = 0\n  #y = 1\n}\n\nclass C {\n  static #x\n  static #y = 1\n}\n\nclass D {\n  #x\n  #y\n}\n\nclass Point {\n  #x = 1\n  #y = 2\n\n  constructor(x = 0, y = 0) {\n    this.#x = +x\n    this.#y = +y\n  }\n\n  get x() {\n    return this.#x\n  }\n  set x(value) {\n    this.#x = +value\n  }\n\n  get y() {\n    return this.#y\n  }\n  set y(value) {\n    this.#y = +value\n  }\n\n  equals(p) {\n    return this.#x === p.#x && this.#y === p.#y\n  }\n\n  toString() {\n    return \\`Point<\\${this.#x},\\${this.#y}>\\`\n  }\n}\n\nclass E {\n  async #a() {}\n  #b() {}\n  get #c() {}\n  set #c(bar) {}\n  *#d() {}\n  async *#e() {}\n  get #f() {}\n  set #f(taz) {}\n}\n\nclass F {\n  #func(id, { blog: { title } }) {\n    return id + title\n  }\n}");
}
#[test]
fn test_private_fields_js_format_1_6ef63b38() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A { #x; #y; }\nclass B { #x = 0; #y = 1; }\n\nclass C {\n  static #x;\n  static #y = 1;\n}\n\nclass D {\n  #x\n  #y\n}\n\nclass Point {\n  #x = 1;\n  #y = 2;\n\n  constructor(x = 0, y = 0) {\n    this.#x = +x;\n    this.#y = +y;\n  }\n\n  get x() { return this.#x }\n  set x(value) { this.#x = +value }\n\n  get y() { return this.#y }\n  set y(value) { this.#y = +value }\n\n  equals(p) { return this.#x === p.#x && this.#y === p.#y }\n\n  toString() { return \\`Point<\\${ this.#x },\\${ this.#y }>\\` }\n}\n\nclass E {\n  async #a() {}\n  #b() {}\n  get #c() {}\n  set #c(bar) {}\n  *#d() {}\n  async *#e() {}\n  get #f() {}\n  set #f(taz) {}\n}\n\nclass F {\n  #func(id, { blog: { title } }) {\n    return id + title;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  #x;\n  #y;\n}\nclass B {\n  #x = 0;\n  #y = 1;\n}\n\nclass C {\n  static #x;\n  static #y = 1;\n}\n\nclass D {\n  #x;\n  #y;\n}\n\nclass Point {\n  #x = 1;\n  #y = 2;\n\n  constructor(x = 0, y = 0) {\n    this.#x = +x;\n    this.#y = +y;\n  }\n\n  get x() {\n    return this.#x;\n  }\n  set x(value) {\n    this.#x = +value;\n  }\n\n  get y() {\n    return this.#y;\n  }\n  set y(value) {\n    this.#y = +value;\n  }\n\n  equals(p) {\n    return this.#x === p.#x && this.#y === p.#y;\n  }\n\n  toString() {\n    return \\`Point<\\${this.#x},\\${this.#y}>\\`;\n  }\n}\n\nclass E {\n  async #a() {}\n  #b() {}\n  get #c() {}\n  set #c(bar) {}\n  *#d() {}\n  async *#e() {}\n  get #f() {}\n  set #f(taz) {}\n}\n\nclass F {\n  #func(id, { blog: { title } }) {\n    return id + title;\n  }\n}");
}
#[test]
fn test_with_comments_js_semifalse_format_1_7c025ab6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  #foobar =\n    // comment to break\n    1 +\n    // comment to break again\n    2;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  #foobar =\n    // comment to break\n    1 +\n    // comment to break again\n    2\n}");
}
#[test]
fn test_with_comments_js_format_1_7c025ab6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  #foobar =\n    // comment to break\n    1 +\n    // comment to break again\n    2;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  #foobar =\n    // comment to break\n    1 +\n    // comment to break again\n    2;\n}");
}
