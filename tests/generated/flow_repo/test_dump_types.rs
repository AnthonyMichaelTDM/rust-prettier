#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_import_js_format_1_bb0bf1a8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("// @flow\nvar num = 42;\nfunction bar() { }\nbar();\nmodule.exports = num;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\nvar num = 42;\nfunction bar() {}\nbar();\nmodule.exports = num;"
    );
}
#[test]
fn test_test_js_format_1_67af1c9f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\nvar num = require('./import');\nfunction foo(x) { }\nfoo(0);\nvar a:string = num;\n\nfunction unannotated(x) {\n  return x;\n}\n\n// test deduping of inferred types\nconst nullToUndefined = val => val === null ? undefined : val;\n\nfunction f0(x: ?Object) { return nullToUndefined(x); }\nfunction f1(x: ?Object) { return nullToUndefined(x); }\nfunction f2(x: ?string) { return nullToUndefined(x); }\nfunction f3(x: ?string) { return nullToUndefined(x); }\n\ndeclare var idx: $Facebookism$Idx;\ndeclare var obj: {a?: {b: ?{c: null | {d: number}}}};\nconst idxResult = idx(obj, obj => obj.a.b.c.d);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\nvar num = require(\"./import\");\nfunction foo(x) {}\nfoo(0);\nvar a: string = num;\n\nfunction unannotated(x) {\n  return x;\n}\n\n// test deduping of inferred types\nconst nullToUndefined = (val) => (val === null ? undefined : val);\n\nfunction f0(x: ?Object) {\n  return nullToUndefined(x);\n}\nfunction f1(x: ?Object) {\n  return nullToUndefined(x);\n}\nfunction f2(x: ?string) {\n  return nullToUndefined(x);\n}\nfunction f3(x: ?string) {\n  return nullToUndefined(x);\n}\n\ndeclare var idx: $Facebookism$Idx;\ndeclare var obj: { a?: { b: ?{ c: null | { d: number } } } };\nconst idxResult = idx(obj, (obj) => obj.a.b.c.d);");
}
