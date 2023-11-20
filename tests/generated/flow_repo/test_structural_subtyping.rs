#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_builtin_js_format_1_e6ab584e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\ninterface IHasLength {\n  length: number;\n}\n\nvar lengthTest1: IHasLength = [];\nvar lengthTest2: IHasLength = 'hello';\nvar lengthTest3: IHasLength = 123; // number doesn't have length\nvar lengthTest4: IHasLength = true; // bool doesn't have length") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\ninterface IHasLength {\n  length: number;\n}\n\nvar lengthTest1: IHasLength = [];\nvar lengthTest2: IHasLength = \"hello\";\nvar lengthTest3: IHasLength = 123; // number doesn't have length\nvar lengthTest4: IHasLength = true; // bool doesn't have length");
}
#[test]
fn test_class_js_format_1_abe9d9f4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass ClassWithXString {\n  x: string;\n}\n\ninterface IHasXString {\n  x: string;\n}\n\ninterface IHasXNumber {\n  x: number;\n}\n\ninterface IHasYString {\n  y: string;\n}\n\nvar testInstance1: IHasXString = new ClassWithXString();\nvar testInstance2: IHasXNumber = new ClassWithXString(); // Error wrong type\nvar testInstance3: IHasYString = new ClassWithXString(); // Error missing prop") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass ClassWithXString {\n  x: string;\n}\n\ninterface IHasXString {\n  x: string;\n}\n\ninterface IHasXNumber {\n  x: number;\n}\n\ninterface IHasYString {\n  y: string;\n}\n\nvar testInstance1: IHasXString = new ClassWithXString();\nvar testInstance2: IHasXNumber = new ClassWithXString(); // Error wrong type\nvar testInstance3: IHasYString = new ClassWithXString(); // Error missing prop");
}
#[test]
fn test_obj_js_format_1_94096ecb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\ninterface IHasXString {\n  x: string;\n}\n\nvar propTest1: IHasXString = { x: 'hello' };\nvar propTest2: IHasXString = { x: 123 }; // Error string != number\nvar propTest3: IHasXString = {}; // Property not found\nvar propTest4: IHasXString = ({}: Object);\n\nfunction propTest5(y: {[key: string]: string}) {\n  (y: IHasXString); // OK\n}\n\nfunction propTest6(y: {[key: string]: number}) {\n  (y: IHasXString); // error: string != number\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\ninterface IHasXString {\n  x: string;\n}\n\nvar propTest1: IHasXString = { x: \"hello\" };\nvar propTest2: IHasXString = { x: 123 }; // Error string != number\nvar propTest3: IHasXString = {}; // Property not found\nvar propTest4: IHasXString = ({}: Object);\n\nfunction propTest5(y: { [key: string]: string }) {\n  (y: IHasXString); // OK\n}\n\nfunction propTest6(y: { [key: string]: number }) {\n  (y: IHasXString); // error: string != number\n}");
}
#[test]
fn test_optional_js_format_1_45780b70() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ninterface HasOptional {\n  a: string,\n  b?: number,\n};\n\nvar test1: HasOptional = { a: \"hello\" }\n\nvar test2: HasOptional = {}; // Error: missing property a\n\nvar test3: HasOptional = { a: \"hello\", b: true }; // Error: boolean ~> number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ninterface HasOptional {\n  a: string;\n  b?: number;\n}\n\nvar test1: HasOptional = { a: \"hello\" };\n\nvar test2: HasOptional = {}; // Error: missing property a\n\nvar test3: HasOptional = { a: \"hello\", b: true }; // Error: boolean ~> number");
}
