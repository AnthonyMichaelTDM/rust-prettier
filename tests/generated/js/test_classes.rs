#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_asi_js_format_1_42a3fea1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/facebook/flow/commit/78f657e8da014e16ff509ad34f8178b158c47af7\nclass C {\n  foo\n    () {}\n}") ? ;
    assert_eq ! (formatted , "// https://github.com/facebook/flow/commit/78f657e8da014e16ff509ad34f8178b158c47af7\nclass C {\n  foo() {}\n}");
    Ok(())
}
#[test]
fn test_assignment_js_format_1_fcf6713f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 = class extends (\n  aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg1\n) {\n  method () {\n    console.log(\"foo\");\n  }\n};\n\nfoo = class extends bar {\n  method() {\n    console.log(\"foo\");\n  }\n};\n\naaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 = class extends bar {\n  method() {\n    console.log(\"foo\");\n  }\n};\n\nfoo = class extends aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 {\n  method() {\n    console.log(\"foo\");\n  }\n};\n\nmodule.exports = class A extends B {\n  method () {\n    console.log(\"foo\");\n  }\n};") ? ;
    assert_eq ! (formatted , "aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 = class extends (\n  aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg1\n) {\n  method() {\n    console.log(\"foo\");\n  }\n};\n\nfoo = class extends bar {\n  method() {\n    console.log(\"foo\");\n  }\n};\n\naaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 = class extends (\n  bar\n) {\n  method() {\n    console.log(\"foo\");\n  }\n};\n\nfoo = class extends (\n  aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2\n) {\n  method() {\n    console.log(\"foo\");\n  }\n};\n\nmodule.exports = class A extends B {\n  method() {\n    console.log(\"foo\");\n  }\n};");
    Ok(())
}
#[test]
fn test_binary_js_format_1_654acb55() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(class {}) + 1;\n(class a {}) + 1;\n(class extends b {}) + 1;\n(class a extends b {}) + 1;") ? ;
    assert_eq ! (formatted , "(class {}) + 1;\n(class a {}) + 1;\n(class extends b {}) + 1;\n(class a extends b {}) + 1;");
    Ok(())
}
#[test]
fn test_call_js_format_1_002e596c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(class {})(class {});")?;
    assert_eq!(formatted, "(class {})(class {});");
    Ok(())
}
#[test]
fn test_class_fields_features_js_format_1_912a0d31() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  static epoch = new CustomDate(0);\n  #xValue = 0;\n  get #x() { return this.#xValue; }\n  set #x(value) {\n    this.#xValue = value;\n    window.requestAnimationFrame(this.#render.bind(this));\n  }\n  #clicked() {\n    this.#x++;\n  }\n  #render() {\n    this.textContent = this.#x.toString();\n  }\n}") ? ;
    assert_eq ! (formatted , "class Foo {\n  static epoch = new CustomDate(0);\n  #xValue = 0;\n  get #x() {\n    return this.#xValue;\n  }\n  set #x(value) {\n    this.#xValue = value;\n    window.requestAnimationFrame(this.#render.bind(this));\n  }\n  #clicked() {\n    this.#x++;\n  }\n  #render() {\n    this.textContent = this.#x.toString();\n  }\n}");
    Ok(())
}
#[test]
fn test_empty_js_format_1_3edf787d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A1 {\n  // comment\n}\n\nclass A2 { // comment\n}\n\nclass A3 {\n}\n\nclass A4 {\n  m() {}\n}") ? ;
    assert_eq ! (formatted , "class A1 {\n  // comment\n}\n\nclass A2 {\n  // comment\n}\n\nclass A3 {}\n\nclass A4 {\n  m() {}\n}");
    Ok(())
}
#[test]
fn test_member_js_format_1_9d72cf85() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(class {})[1];\n(class {}).a;")?;
    assert_eq!(formatted, "(class {})[1];\n(class {}).a;");
    Ok(())
}
#[test]
fn test_method_js_format_1_bcc5386a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "\nclass C {\n  name/*comment*/() {\n  }\n};\n\n\n({\n  name/*comment*/() {\n  }\n});",
    )?;
    assert_eq!(
        formatted,
        "class C {\n  name /*comment*/() {}\n}\n\n({\n  name /*comment*/() {},\n});"
    );
    Ok(())
}
#[test]
fn test_new_js_format_1_9be69c30() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("new class {};\nnew Ctor(class {});")?;
    assert_eq!(formatted, "new (class {})();\nnew Ctor(class {});");
    Ok(())
}
#[test]
fn test_property_js_format_1_d30facc6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  foobar =\n    // comment to break\n    1 +\n    // comment to break again\n    2;\n}\n\nclass B {\n  someInstanceProperty = this.props.foofoofoofoofoofoo &&\n    this.props.barbarbarbar;\n  \n  someInstanceProperty2 = { foo: this.props.foofoofoofoofoofoo &&\n    this.props.barbarbarbar };\n  \n    someInstanceProperty3 =\n  \"foo\";\n}") ? ;
    assert_eq ! (formatted , "class A {\n  foobar =\n    // comment to break\n    1 +\n    // comment to break again\n    2;\n}\n\nclass B {\n  someInstanceProperty =\n    this.props.foofoofoofoofoofoo && this.props.barbarbarbar;\n\n  someInstanceProperty2 = {\n    foo: this.props.foofoofoofoofoofoo && this.props.barbarbarbar,\n  };\n\n  someInstanceProperty3 = \"foo\";\n}");
    Ok(())
}
#[test]
fn test_super_js_format_1_d194035c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class A extends B {\n    #a() {\n        super.x();\n    }\n}")?;
    assert_eq!(
        formatted,
        "class A extends B {\n  #a() {\n    super.x();\n  }\n}"
    );
    Ok(())
}
#[test]
fn test_ternary_js_format_1_be6e16e0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("if (1) (class {}) ? 1 : 2;")?;
    assert_eq!(formatted, "if (1) (class {}) ? 1 : 2;");
    Ok(())
}
