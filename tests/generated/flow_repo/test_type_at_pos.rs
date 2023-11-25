#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_destructuring_js_format_1_06f6ae34() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet [x, y] = [1, 2];\n\n/**\n * Test what happens when the destructuring is unevaluated. In this case,\n * `this` in a function is unbound, so we never actually find out the type of\n * `this.returnsATuple()` is; thus, we never evaluate `b` and so type-at-pos\n * returns EmptyT.\n */\nexport const X = {\n  returnsATuple: function(): [number, number] {\n    return [1, 2];\n  },\n\n  test: function() {\n    let [a, b] = this.returnsATuple();\n  }\n};") ? ;
    assert_eq ! (formatted , "// @flow\n\nlet [x, y] = [1, 2];\n\n/**\n * Test what happens when the destructuring is unevaluated. In this case,\n * `this` in a function is unbound, so we never actually find out the type of\n * `this.returnsATuple()` is; thus, we never evaluate `b` and so type-at-pos\n * returns EmptyT.\n */\nexport const X = {\n  returnsATuple: function (): [number, number] {\n    return [1, 2];\n  },\n\n  test: function () {\n    let [a, b] = this.returnsATuple();\n  },\n};");
    Ok(())
}
#[test]
fn test_function_expressions_js_format_1_83a0f200() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// TODO: type-at-pos between the ()'s should be () => void\n// class X {\n//   foo(): void {}\n// }\n\nconst a = {\n  bar(): void {}\n};\n\nconst b = {\n  bar: function (): void {}\n};\n\nconst c = {\n  m<T>(x: T): T { return x; }\n};\n\nconst d = {\n  m: function<T>(x: T): T { return x; }\n};") ? ;
    assert_eq ! (formatted , "// @flow\n\n// TODO: type-at-pos between the ()'s should be () => void\n// class X {\n//   foo(): void {}\n// }\n\nconst a = {\n  bar(): void {},\n};\n\nconst b = {\n  bar: function (): void {},\n};\n\nconst c = {\n  m<T>(x: T): T {\n    return x;\n  },\n};\n\nconst d = {\n  m: function <T>(x: T): T {\n    return x;\n  },\n};");
    Ok(())
}
#[test]
fn test_generics_js_format_1_c78f9899() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass C<X> { }\nvar cn: C<number> = new C;\ncn;\n\nfunction foo() { return C; }\nvar D = foo();\nvar dn: D<number> = new C;\ndn;\n\ntype E<X> = C<X>;\nvar en: E<number> = new C;\nen;\n\ntype F<X> = C<void>;\nvar fn: F<number> = new C;\nfn;\n\ntype O<X> = { x: X };\nvar on: O<number> = { x: 0 };\non;\n\ntype Mono = C<void>;\nvar mn: Mono<number> = new C; // error: application of non-poly type\nmn;") ? ;
    assert_eq ! (formatted , "// @flow\n\nclass C<X> {}\nvar cn: C<number> = new C();\ncn;\n\nfunction foo() {\n  return C;\n}\nvar D = foo();\nvar dn: D<number> = new C();\ndn;\n\ntype E<X> = C<X>;\nvar en: E<number> = new C();\nen;\n\ntype F<X> = C<void>;\nvar fn: F<number> = new C();\nfn;\n\ntype O<X> = { x: X };\nvar on: O<number> = { x: 0 };\non;\n\ntype Mono = C<void>;\nvar mn: Mono<number> = new C(); // error: application of non-poly type\nmn;");
    Ok(())
}
#[test]
fn test_import_js_format_1_bb0bf1a8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\nvar num = 42;\nfunction bar() { }\nbar();\nmodule.exports = num;")?;
    assert_eq!(
        formatted,
        "// @flow\nvar num = 42;\nfunction bar() {}\nbar();\nmodule.exports = num;"
    );
    Ok(())
}
#[test]
fn test_object_special_cases_js_format_1_153689cb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nlet tests = [\n  function() {\n    let x = {};\n    Object.defineProperty(x, 'foo', { value: '' });\n  },\n];") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nlet tests = [\n  function () {\n    let x = {};\n    Object.defineProperty(x, \"foo\", { value: \"\" });\n  },\n];");
    Ok(())
}
#[test]
fn test_optional_js_format_1_789b7ea1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction foo(x?: string) {\n  return x;\n}\n\nfoo();\n\nfunction bar(obj: { x?: string }) {\n  return obj.x;\n}\n\nfunction qux(x?) {\n  return x;\n}") ? ;
    assert_eq ! (formatted , "// @flow\n\nfunction foo(x?: string) {\n  return x;\n}\n\nfoo();\n\nfunction bar(obj: { x?: string }) {\n  return obj.x;\n}\n\nfunction qux(x?) {\n  return x;\n}");
    Ok(())
}
#[test]
fn test_predicates_js_format_1_2d77edff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nlet x = 0;\nif (x == null) {}\nif (x == undefined) {}\nif (Array.isArray(x)) {}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nlet x = 0;\nif (x == null) {\n}\nif (x == undefined) {\n}\nif (Array.isArray(x)) {\n}");
    Ok(())
}
#[test]
fn test_react_js_format_1_e9cbb0b0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import React from \"react\";\nReact.createElement;")?;
    assert_eq!(
        formatted,
        "import React from \"react\";\nReact.createElement;"
    );
    Ok(())
}
#[test]
fn test_templates_js_format_1_52cf3705() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n`foo bar`;")?;
    assert_eq!(formatted, "/* @flow */\n`foo bar`;");
    Ok(())
}
#[test]
fn test_test_js_format_1_912bb85a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\nvar str = require('./import');\nfunction foo() { }\nfoo();\nstr\n\ntype Point = [number, string];\nconst x:Point = [1, \"foo\"];\ntype MyStr = \"cool\";\nconst y:MyStr = \"cool\";\ntype MyBool = true;\nconst z:MyBool = true;\ntype MyNum = 42;\nconst w:MyNum = 42;") ? ;
    assert_eq ! (formatted , "// @flow\nvar str = require(\"./import\");\nfunction foo() {}\nfoo();\nstr;\n\ntype Point = [number, string];\nconst x: Point = [1, \"foo\"];\ntype MyStr = \"cool\";\nconst y: MyStr = \"cool\";\ntype MyBool = true;\nconst z: MyBool = true;\ntype MyNum = 42;\nconst w: MyNum = 42;");
    Ok(())
}
#[test]
fn test_trycatch_js_format_1_9729dca5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\ntry {\n  throw \"foo\";\n} catch (e) {}")?;
    assert_eq!(
        formatted,
        "// @flow\n\ntry {\n  throw \"foo\";\n} catch (e) {}"
    );
    Ok(())
}
