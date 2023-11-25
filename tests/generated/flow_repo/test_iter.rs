use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_iter_js_format_1_6e8cffa9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var a = [true,false];\nfunction foo(x) { }\n\nfor (var i=0;i<3;i++) {\n    foo(a[i]);\n}\nfor (var k in a) {\n    foo(a[k]); // k is a string, which shouldn't be used for array access\n}\n\nvar b = (null : ?{[key: string]: string});\nfor (var j in b) {\n    foo(b[j]);\n}\n\nvar c;\nfor (var m in (c = b)) {\n    foo(c[m]);\n}\n\nvar d;\nfor (var n in (d = a)) {\n    foo(d[n]); // d is a string, which shouldn't be used for array access\n}\n\nfor (var x in undefined) {\n    foo(x); // unreachable\n}\n\nfor (var x in null) {\n    foo(x); // unreachable\n}\n\nfor (var y in this) {\n    // regression test to make sure \\`in this\\` doesn't fatal. it's currently\n    // allowed, even though we can't actually enumerate all the keys on \\`this\\`.\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var a = [true, false];\nfunction foo(x) {}\n\nfor (var i = 0; i < 3; i++) {\n  foo(a[i]);\n}\nfor (var k in a) {\n  foo(a[k]); // k is a string, which shouldn't be used for array access\n}\n\nvar b = (null: ?{ [key: string]: string });\nfor (var j in b) {\n  foo(b[j]);\n}\n\nvar c;\nfor (var m in (c = b)) {\n  foo(c[m]);\n}\n\nvar d;\nfor (var n in (d = a)) {\n  foo(d[n]); // d is a string, which shouldn't be used for array access\n}\n\nfor (var x in undefined) {\n  foo(x); // unreachable\n}\n\nfor (var x in null) {\n  foo(x); // unreachable\n}\n\nfor (var y in this) {\n  // regression test to make sure \\`in this\\` doesn't fatal. it's currently\n  // allowed, even though we can't actually enumerate all the keys on \\`this\\`.\n}");
}
