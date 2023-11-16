#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_scope_js_format_1_d19a1503() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function foo<X, Y:X>(x:X, y:Y):void { }\nfoo(0, \"\");\n\nfunction bar<X:number, Y:X>(x:X, y:Y): number { return y*0; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo<X, Y: X>(x: X, y: Y): void {}\nfoo(0, \"\");\n\nfunction bar<X: number, Y: X>(x: X, y: Y): number {\n  return y * 0;\n}");
}
#[test]
fn test_test_js_format_1_ca63dcff() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function foo<T: number>(x: T): T {\n    var _ = x * 1; // OK\n    var y: string = x; // error\n    return x; // OK\n}\n\nclass C<T: number> {\n    bar<U: number>(x: U): T {\n        return x; // error, since T: number and U: number does not imply U: T\n    }\n    qux<U: T>(x: U): T {\n        var _ = x * 1; // OK, since T: number and U: T implies U: number\n        var y: string = x; // error\n        return x; // OK, since U: T\n    }\n}\n\nfunction example<T: {x: number}>(o: T): T { o.x = 0; return o; }\nvar obj1: {x: number; y: string} = example({x: 0, y: \"\"});\nvar obj2: {x: number} = example({x: 0});\n\nvar c: C<string> = new C; // error, since T = string is incompatible with number\nvar q: number = c.qux(0);\n/* 2 more errors, since argument U = number is incompatible with T = string, and\n * result T = string is incompatible with number */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo<T: number>(x: T): T {\n  var _ = x * 1; // OK\n  var y: string = x; // error\n  return x; // OK\n}\n\nclass C<T: number> {\n  bar<U: number>(x: U): T {\n    return x; // error, since T: number and U: number does not imply U: T\n  }\n  qux<U: T>(x: U): T {\n    var _ = x * 1; // OK, since T: number and U: T implies U: number\n    var y: string = x; // error\n    return x; // OK, since U: T\n  }\n}\n\nfunction example<T: { x: number }>(o: T): T {\n  o.x = 0;\n  return o;\n}\nvar obj1: { x: number, y: string } = example({ x: 0, y: \"\" });\nvar obj2: { x: number } = example({ x: 0 });\n\nvar c: C<string> = new C(); // error, since T = string is incompatible with number\nvar q: number = c.qux(0);\n/* 2 more errors, since argument U = number is incompatible with T = string, and\n * result T = string is incompatible with number */");
}
