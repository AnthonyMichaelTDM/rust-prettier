#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_f1332958() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// You should be able to call objects with call properties\nfunction a(f: { (): string }, g: { (x: number): string } ): string {\n  return f() + g(123);\n}\n\n// ...and get an error if the return type is wrong\nfunction b(f: { (): string }): number {\n  return f();\n}\n\n// ...or if the param type is wrong\nfunction c(f: { (x: number): number }): number {\n  return f(\"hello\");\n}\n\n// ...or if the arity is wrong\nfunction d(f: { (x: number): number }): number {\n  return f();\n}\n\n// ...or if there is no call property\nfunction e(f: {}): number {\n  return f();\n}\n\n// Make sure we complain even if the object literal is unsealed.\nfunction f(): number {\n  var x = {};\n  return x();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// You should be able to call objects with call properties\nfunction a(f: { (): string }, g: { (x: number): string }): string {\n  return f() + g(123);\n}\n\n// ...and get an error if the return type is wrong\nfunction b(f: { (): string }): number {\n  return f();\n}\n\n// ...or if the param type is wrong\nfunction c(f: { (x: number): number }): number {\n  return f(\"hello\");\n}\n\n// ...or if the arity is wrong\nfunction d(f: { (x: number): number }): number {\n  return f();\n}\n\n// ...or if there is no call property\nfunction e(f: {}): number {\n  return f();\n}\n\n// Make sure we complain even if the object literal is unsealed.\nfunction f(): number {\n  var x = {};\n  return x();\n}");
}
#[test]
fn test_b_js_format_1_3e139176() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// You should be able to use a function as an object with a call property\nvar a: { (x: number): string } = function (x: number): string { return \"hi\"; };\n\n// ...and it should notice when the return type is wrong\nvar b: { (x: number): number } = function (x: number): string { return \"hi\"; };\n\n// ...or if the param type is wrong\nvar c: { (x: string): string } = function (x: number): string { return \"hi\"; };\n\n// ...or if the arity is wrong\nvar d: { (): string } = function (x: number): string { return \"hi\"; };\n\n// ...but subtyping rules still apply\nvar e: { (x: any): void } = function() { } // arity\nvar f: { (): mixed } = function(): string { return \"hi\" } // return type\nvar g: { (x: string): void } = function(x: mixed) { } // param type\n\n// A function can be an object\nvar y : {} = function (x: number): string { return \"hi\"; };\nvar z : Object = function (x: number): string { return \"hi\"; };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// You should be able to use a function as an object with a call property\nvar a: { (x: number): string } = function (x: number): string {\n  return \"hi\";\n};\n\n// ...and it should notice when the return type is wrong\nvar b: { (x: number): number } = function (x: number): string {\n  return \"hi\";\n};\n\n// ...or if the param type is wrong\nvar c: { (x: string): string } = function (x: number): string {\n  return \"hi\";\n};\n\n// ...or if the arity is wrong\nvar d: { (): string } = function (x: number): string {\n  return \"hi\";\n};\n\n// ...but subtyping rules still apply\nvar e: { (x: any): void } = function () {}; // arity\nvar f: { (): mixed } = function (): string {\n  return \"hi\";\n}; // return type\nvar g: { (x: string): void } = function (x: mixed) {}; // param type\n\n// A function can be an object\nvar y: {} = function (x: number): string {\n  return \"hi\";\n};\nvar z: Object = function (x: number): string {\n  return \"hi\";\n};");
}
#[test]
fn test_c_js_format_1_4907fd18() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// You should be able to use an object as a function\nfunction a(x: { (z: number): string }): (z: number) => string {\n  return x;\n}\n\n// ...and it should notice when the return type is wrong\nfunction b(x: { (z: number): string }): (z: number) => number {\n  return x;\n}\n\n// ...or if the param type is wrong\nfunction c(x: { (z: number): string }): (z: string) => string {\n  return x;\n}\n\n// ...or if the arity is wrong\nfunction d(x: { (z: number): string }): () => string {\n  return x;\n}\n\n// ...or if it doesn't have a call property\nfunction e(x: {}): () => string {\n  return x;\n}\n\n// AnyFunT should also be allowed\nfunction f(x: { (z: number): string }): Function {\n  return x;\n}\n\n// ... but only if the object is callable\nfunction g(x: {}): Function {\n  return x; // error\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// You should be able to use an object as a function\nfunction a(x: { (z: number): string }): (z: number) => string {\n  return x;\n}\n\n// ...and it should notice when the return type is wrong\nfunction b(x: { (z: number): string }): (z: number) => number {\n  return x;\n}\n\n// ...or if the param type is wrong\nfunction c(x: { (z: number): string }): (z: string) => string {\n  return x;\n}\n\n// ...or if the arity is wrong\nfunction d(x: { (z: number): string }): () => string {\n  return x;\n}\n\n// ...or if it doesn't have a call property\nfunction e(x: {}): () => string {\n  return x;\n}\n\n// AnyFunT should also be allowed\nfunction f(x: { (z: number): string }): Function {\n  return x;\n}\n\n// ... but only if the object is callable\nfunction g(x: {}): Function {\n  return x; // error\n}");
}
#[test]
fn test_d_js_format_1_a880dc61() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Multiple call properties should also be supported\nfunction a(f: { (): string; (x: number): string }): string {\n  return f() + f(123);\n}\n\n// It should be fine when a function satisfies them all\nvar b: { (): string; (x: number): string } =\n  function (x?: number): string { return \"hi\"; };\n\n// ...but should notice when a function doesn't satisfy them all\nvar c: { (): string; (x: number): string } =\n  function (x: number): string { return \"hi\"; };\n\n// Only one call property needs to match the function\nfunction d(x: { (): string; (x: number): string }): () => string {\n  return x;\n}\n\n// ...but you need at least one\nfunction e(x: { (): string; (x: number): string }): () => number {\n  return x;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Multiple call properties should also be supported\nfunction a(f: { (): string, (x: number): string }): string {\n  return f() + f(123);\n}\n\n// It should be fine when a function satisfies them all\nvar b: { (): string, (x: number): string } = function (x?: number): string {\n  return \"hi\";\n};\n\n// ...but should notice when a function doesn't satisfy them all\nvar c: { (): string, (x: number): string } = function (x: number): string {\n  return \"hi\";\n};\n\n// Only one call property needs to match the function\nfunction d(x: { (): string, (x: number): string }): () => string {\n  return x;\n}\n\n// ...but you need at least one\nfunction e(x: { (): string, (x: number): string }): () => number {\n  return x;\n}");
}
#[test]
fn test_e_js_format_1_ead80323() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Expecting properties that don't exist should be an error\nvar a : { someProp: number } = function () {};\n\n// Expecting properties that do exist should be fine\nvar b : { apply: Function } = function () {};\n\n// Expecting properties in the functions statics should be fine\nvar f = function () {};\nf.myProp = 123;\nvar c : { myProp: number } = f;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Expecting properties that don't exist should be an error\nvar a: { someProp: number } = function () {};\n\n// Expecting properties that do exist should be fine\nvar b: { apply: Function } = function () {};\n\n// Expecting properties in the functions statics should be fine\nvar f = function () {};\nf.myProp = 123;\nvar c: { myProp: number } = f;");
}
#[test]
fn test_f_js_format_1_46a6c506() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// You should be able to use an arrow function as an object with a call property\n\nvar a: { (x: number): string } = (x) => x.toString()\n\n// ...and it should notice when the return type is wrong\nvar b: { (x: number): number } = (x) => \"hi\"\n\n// ...or if the param type is wrong\nvar c: { (x: string): string } = (x) => x.toFixed()\n\n// ...or if the arity is wrong\nvar d: { (): string } = (x) => \"hi\"\n\n// ...but subtyping rules still apply\nvar e: { (x: any): void } = () => { } // arity\nvar f: { (): mixed } = () => \"hi\" // return type\nvar g: { (x: Date): void } = (x) => { x * 2 } // param type (date < number)\n\n// A function can be an object\nvar y : {} = (x) => \"hi\"\nvar z : Object = (x) => \"hi\"") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// You should be able to use an arrow function as an object with a call property\n\nvar a: { (x: number): string } = (x) => x.toString();\n\n// ...and it should notice when the return type is wrong\nvar b: { (x: number): number } = (x) => \"hi\";\n\n// ...or if the param type is wrong\nvar c: { (x: string): string } = (x) => x.toFixed();\n\n// ...or if the arity is wrong\nvar d: { (): string } = (x) => \"hi\";\n\n// ...but subtyping rules still apply\nvar e: { (x: any): void } = () => {}; // arity\nvar f: { (): mixed } = () => \"hi\"; // return type\nvar g: { (x: Date): void } = (x) => {\n  x * 2;\n}; // param type (date < number)\n\n// A function can be an object\nvar y: {} = (x) => \"hi\";\nvar z: Object = (x) => \"hi\";");
}
