#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_encaps_js_format_1_67c4e1bc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A { }\nvar a = new A();\nvar s1 = \\`l\\${a.x}r\\`; // error: no prop x in A\n\nfunction tag(strings,...values) {\n    var x:number = strings[0]; // error: string ~> number\n    return x;\n}\nvar s2 = tag \\`l\\${42}r\\`;\n\nfunction tag2(strings,...values) {\n  return { foo: \"\" }; // ok: tagged templates can return whatever\n}\n\nvar s3 = tag2 \\`la la la\\`;\n(s3.foo: number); // error: string ~> number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {}\nvar a = new A();\nvar s1 = \\`l\\${a.x}r\\`; // error: no prop x in A\n\nfunction tag(strings, ...values) {\n  var x: number = strings[0]; // error: string ~> number\n  return x;\n}\nvar s2 = tag\\`l\\${42}r\\`;\n\nfunction tag2(strings, ...values) {\n  return { foo: \"\" }; // ok: tagged templates can return whatever\n}\n\nvar s3 = tag2\\`la la la\\`;\n(s3.foo: number); // error: string ~> number");
}
