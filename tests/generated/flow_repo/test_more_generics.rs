use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_poly_js_format_1_0b3b200b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var foo1 = function<T>(x:T):T { return x; }\n\nfunction foo2<T,S>(x:T):S { return x; }\n\nvar foo3 = function <T>(x:T):T { return foo3(x); }\n\nfunction foo4<T,S>(x:T):S { return foo4(x); }\n\nvar x = [];\nfunction foo5<T>():Array<T> { return x; }\n/*\n var a = foo5();\n a[0] = 0;\n var b = foo5();\n var y: string = b[0];\n*/\n\nvar foo6 = function<R>(x:R):R { return foo1(x); }\n\nfunction foo7<R>(x:R):R { return foo5(); }\n\nfunction foo8<U>(x:U,y):U {\n  var z = foo8(x,x);\n  y();\n  return x;\n}\n/*\n foo8(0,void 0);\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var foo1 = function <T>(x: T): T {\n  return x;\n};\n\nfunction foo2<T, S>(x: T): S {\n  return x;\n}\n\nvar foo3 = function <T>(x: T): T {\n  return foo3(x);\n};\n\nfunction foo4<T, S>(x: T): S {\n  return foo4(x);\n}\n\nvar x = [];\nfunction foo5<T>(): Array<T> {\n  return x;\n}\n/*\n var a = foo5();\n a[0] = 0;\n var b = foo5();\n var y: string = b[0];\n*/\n\nvar foo6 = function <R>(x: R): R {\n  return foo1(x);\n};\n\nfunction foo7<R>(x: R): R {\n  return foo5();\n}\n\nfunction foo8<U>(x: U, y): U {\n  var z = foo8(x, x);\n  y();\n  return x;\n}\n/*\n foo8(0,void 0);\n*/");
}
