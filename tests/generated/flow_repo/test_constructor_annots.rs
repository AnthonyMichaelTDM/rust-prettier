#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_constructors_js_format_1_79e4c3cb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Foo is a class-like function\nfunction Foo() {\n  this.x = 0; // constructs objects with property x\n}\nFoo.y = 0; // has static property y\nFoo.prototype = { m() { return 0; } };\n\n// exporting Foo directly doesn't work\n// Foo's instance and static props are not picked up\nexports.Foo = Foo;\n\n// so you want to type Foo, by declaring it as a class\ninterface IFooPrototype {\n  m: () => number;\n}\ninterface IFoo extends IFooPrototype {\n  x: boolean; // error, should have declared x: number instead\n  static (): void;\n  constructor(): void;\n}\nexports.Foo2 = (Foo: Class<IFoo>);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Foo is a class-like function\nfunction Foo() {\n  this.x = 0; // constructs objects with property x\n}\nFoo.y = 0; // has static property y\nFoo.prototype = {\n  m() {\n    return 0;\n  },\n};\n\n// exporting Foo directly doesn't work\n// Foo's instance and static props are not picked up\nexports.Foo = Foo;\n\n// so you want to type Foo, by declaring it as a class\ninterface IFooPrototype {\n  m: () => number;\n}\ninterface IFoo extends IFooPrototype {\n  x: boolean; // error, should have declared x: number instead\n  static(): void;\n  constructor(): void;\n}\nexports.Foo2 = (Foo: Class<IFoo>);");
}
#[test]
fn test_test_js_format_1_52ada683() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var Foo = require('./constructors').Foo;\nvar x: string = new Foo().x; // error, found number instead of string\nvar y: string = Foo.y; // error, found number instead of string\nvar z: string = new Foo().m();\n\nvar Foo2 = require('./constructors').Foo2;\nvar x2: string = new Foo2().x; // error, found boolean instead of string\nvar y2: string = Foo2.y; // error, found boolean instead of string\nvar z2: string = new Foo2().m();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var Foo = require(\"./constructors\").Foo;\nvar x: string = new Foo().x; // error, found number instead of string\nvar y: string = Foo.y; // error, found number instead of string\nvar z: string = new Foo().m();\n\nvar Foo2 = require(\"./constructors\").Foo2;\nvar x2: string = new Foo2().x; // error, found boolean instead of string\nvar y2: string = Foo2.y; // error, found boolean instead of string\nvar z2: string = new Foo2().m();");
}
