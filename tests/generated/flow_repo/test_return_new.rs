#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_3df01622() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function Foo() { return {}; }\nvar foo: number = new Foo(); // error (returns object literal above)\n\nfunction Bar() { return 0; }\nvar bar: number = new Bar(); // error (returns new object)\n\nfunction Qux() { }\nvar qux: number = new Qux(); // error (returns new object)\n\nclass A { }\nfunction B() { return new A(); }\nvar a: A = new B(); // OK (returns new A)\n\n// type applications should be applied before deciding if object-like\ntype C<T> = { x: T };\nfunction makeC<T>(x: T): C<T> { return {x}; }\n(new makeC('x'): C<string>); // normally you wouldn't use \\`new\\`, but you can\n\n// unions should be split before deciding if object-like\nfunction makeUnion(): number | {x: string} {\n  return {x: 'x'};\n}\n(new makeUnion(): {x: string}); // error: \\`number\\` returns {}, missing prop x") ? ;
    assert_eq ! (formatted , "function Foo() {\n  return {};\n}\nvar foo: number = new Foo(); // error (returns object literal above)\n\nfunction Bar() {\n  return 0;\n}\nvar bar: number = new Bar(); // error (returns new object)\n\nfunction Qux() {}\nvar qux: number = new Qux(); // error (returns new object)\n\nclass A {}\nfunction B() {\n  return new A();\n}\nvar a: A = new B(); // OK (returns new A)\n\n// type applications should be applied before deciding if object-like\ntype C<T> = { x: T };\nfunction makeC<T>(x: T): C<T> {\n  return { x };\n}\n(new makeC(\"x\"): C<string>); // normally you wouldn't use \\`new\\`, but you can\n\n// unions should be split before deciding if object-like\nfunction makeUnion(): number | { x: string } {\n  return { x: \"x\" };\n}\n(new makeUnion(): { x: string }); // error: \\`number\\` returns {}, missing prop x");
    Ok(())
}
#[test]
fn test_test_2_js_format_1_68af8680() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class D {\n  constructor(): { x: number }; // OK\n  y: any;\n}\n\nvar d = new D();\nd.x = \"\"; // error, string ~/~ number (but property x is found)\n\n(new D: D); // error, new D is an object, D not in proto chain\n\nmodule.exports = D;") ? ;
    assert_eq ! (formatted , "declare class D {\n  constructor(): { x: number }; // OK\n  y: any;\n}\n\nvar d = new D();\nd.x = \"\"; // error, string ~/~ number (but property x is found)\n\n(new D(): D); // error, new D is an object, D not in proto chain\n\nmodule.exports = D;");
    Ok(())
}
