#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_array_js_format_1_55200b14() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var a = [\"...\"];\nvar b = a.map (function (x) { return 0; });\nvar c: string = b[0]; // error: number !~> string\n\nvar array = [];\nfunction f() {\n    array = array.map (function () { return \"...\"; });\n    var x:number = array[0]; // error: string !~> number\n}\n\nvar Foo = require('./genericfoo');\nvar foo = new Foo();\nfunction g() {\n    var foo1 = foo.map (function() { return \"...\"; });\n    var x:number = foo1.get(); // error: string !~> number\n    foo = foo1;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var a = [\"...\"];\nvar b = a.map(function (x) {\n  return 0;\n});\nvar c: string = b[0]; // error: number !~> string\n\nvar array = [];\nfunction f() {\n  array = array.map(function () {\n    return \"...\";\n  });\n  var x: number = array[0]; // error: string !~> number\n}\n\nvar Foo = require(\"./genericfoo\");\nvar foo = new Foo();\nfunction g() {\n  var foo1 = foo.map(function () {\n    return \"...\";\n  });\n  var x: number = foo1.get(); // error: string !~> number\n  foo = foo1;\n}");
}
#[test]
fn test_genericfoo_js_format_1_7dcfd301() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("class Foo<T> {\n    x:T;\n    self():Foo<T> { return this; }\n    map<U>(callbackfn: () => U): Foo<U> { return new Foo(); }\n    set(x:T): void { }\n    get(): T { return this.x; }\n}\n\nmodule.exports = Foo;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo<T> {\n  x: T;\n  self(): Foo<T> {\n    return this;\n  }\n  map<U>(callbackfn: () => U): Foo<U> {\n    return new Foo();\n  }\n  set(x: T): void {}\n  get(): T {\n    return this.x;\n  }\n}\n\nmodule.exports = Foo;");
}
