#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_export_cjs_default_class_js_format_1_14d65da4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass ClassFoo3 {\n  givesANum(): number { return 42; }\n  static givesAFoo3(): ClassFoo3 {\n    return new ClassFoo3();\n  }\n}\n\nmodule.exports = ClassFoo3;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass ClassFoo3 {\n  givesANum(): number {\n    return 42;\n  }\n  static givesAFoo3(): ClassFoo3 {\n    return new ClassFoo3();\n  }\n}\n\nmodule.exports = ClassFoo3;");
}
#[test]
fn test_export_cjs_named_class_js_format_1_562fad13() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass ClassFoo4 {\n  returnsANumber(): number { return 42; }\n}\n\nclass ClassFoo5 {}\n\nfunction givesAFoo4(): ClassFoo4 {\n  return new ClassFoo4();\n}\n\nfunction givesAFoo5(): ClassFoo5 {\n  return new ClassFoo5();\n}\n\nexports.ClassFoo4 = ClassFoo4;\nexports.ClassFoo5 = ClassFoo5\nexports.foo4Inst = new ClassFoo4();\nexports.foo5Inst = new ClassFoo5();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass ClassFoo4 {\n  returnsANumber(): number {\n    return 42;\n  }\n}\n\nclass ClassFoo5 {}\n\nfunction givesAFoo4(): ClassFoo4 {\n  return new ClassFoo4();\n}\n\nfunction givesAFoo5(): ClassFoo5 {\n  return new ClassFoo5();\n}\n\nexports.ClassFoo4 = ClassFoo4;\nexports.ClassFoo5 = ClassFoo5;\nexports.foo4Inst = new ClassFoo4();\nexports.foo5Inst = new ClassFoo5();");
}
#[test]
fn test_export_default_class_js_format_1_5d74059a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass ClassFoo1 {\n  returnsANumber(): number { return 42; }\n}\n\nexport default ClassFoo1;\nexport var foo1Inst = new ClassFoo1();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass ClassFoo1 {\n  returnsANumber(): number {\n    return 42;\n  }\n}\n\nexport default ClassFoo1;\nexport var foo1Inst = new ClassFoo1();");
}
#[test]
fn test_export_named_alias_js_format_1_f9576ed3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nexport type AliasFoo3  = {\n  givesANum(): number\n};\nexport function givesAFoo3Obj(): AliasFoo3 {\n  return {\n    givesANum(): number { return 42; }\n  };\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nexport type AliasFoo3 = {\n  givesANum(): number,\n};\nexport function givesAFoo3Obj(): AliasFoo3 {\n  return {\n    givesANum(): number {\n      return 42;\n    },\n  };\n}");
}
#[test]
fn test_export_named_class_js_format_1_b23317aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass ClassFoo2 {\n  returnsANumber(): number { return 42; }\n}\n\nexport {ClassFoo2};\nexport var foo2Inst = new ClassFoo2();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass ClassFoo2 {\n  returnsANumber(): number {\n    return 42;\n  }\n}\n\nexport { ClassFoo2 };\nexport var foo2Inst = new ClassFoo2();");
}
#[test]
fn test_exports_a_number_js_format_1_77360406() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @ flow */\n\nexport var numValue = 42;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @ flow */\n\nexport var numValue = 42;");
}
#[test]
fn test_import_type_js_format_1_e06d9d6c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\n/////////////////////////////////////////////////\n// == Importing Class Type (Default Export) == //\n/////////////////////////////////////////////////\n\nimport type ClassFoo1 from \"./ExportDefault_Class\";\nimport {foo1Inst} from \"./ExportDefault_Class\";\n\nvar a1: ClassFoo1 = foo1Inst;\nvar a2: number = foo1Inst; // Error: ClassFoo1 ~> number\nnew ClassFoo1(); // Error: ClassFoo1 is not a value-identifier\n\n///////////////////////////////////////////////\n// == Importing Class Type (Named Export) == //\n///////////////////////////////////////////////\n\nimport type {ClassFoo2} from \"./ExportNamed_Class\";\nimport {foo2Inst} from \"./ExportNamed_Class\";\n\nvar b1: ClassFoo2 = foo2Inst;\nvar b2: number = foo2Inst; // Error: ClassFoo2 ~> number\nnew ClassFoo2(); // Error: ClassFoo2 is not a value-identifier\n\n/////////////////////////////////////////////////////\n// == Importing Class Type (CJS Default Export) == //\n/////////////////////////////////////////////////////\n\nimport type ClassFoo3T from \"./ExportCJSDefault_Class\";\nimport ClassFoo3 from \"./ExportCJSDefault_Class\";\nvar c1: ClassFoo3T = new ClassFoo3();\nnew ClassFoo3T(); // Error: ClassFoo3 is not a value-identifier\n\n///////////////////////////////////////////////////\n// == Importing Class Type (CJS Named Export) == //\n///////////////////////////////////////////////////\n\nimport type {ClassFoo4, ClassFoo5} from \"./ExportCJSNamed_Class\";\nimport {foo4Inst, foo5Inst} from \"./ExportCJSNamed_Class\";\n\nvar d1: ClassFoo4 = foo4Inst;\nvar d2: number = foo4Inst; // Error: ClassFoo4 ~> number\nnew ClassFoo4(); // Error: ClassFoo4 is not a value-identifier\n// TODO: this errors correctly, but the message is just 'can't resolve name'\nvar d3: typeof ClassFoo5 = foo5Inst; // Error: Can't typeof a type alias\n\n////////////////////////////////////////////\n// == Import Type Alias (Named Export) == //\n////////////////////////////////////////////\n\nimport type {AliasFoo3} from \"./ExportNamed_Alias\";\nimport {givesAFoo3Obj} from \"./ExportNamed_Alias\";\nvar e1: AliasFoo3 = givesAFoo3Obj();\nvar e2: number = givesAFoo3Obj(); // Error: AliasFoo3 ~> number\nvar e3: typeof AliasFoo3 = givesAFoo3Obj(); // Error: Can't typeof a type alias\n\n//////////////////////////////////////////////\n// == Import Type Alias (Default Export) == //\n//////////////////////////////////////////////\n\n// TODO: No support for this right now. It's most likely possible, but it's\n//       unclear how useful it is at the moment and it entails a little\n//       more work than named type exports, so I'm punting on it for now.\n\n///////////////////////////////////////////////////////\n// == Import Type With Non-Alias Compatible Value == //\n///////////////////////////////////////////////////////\n\nimport type {numValue} from \"./ExportsANumber\"; // Error: Cannot import-type a number value\n\n////////////////////////////////////////////////////////////////////////\n// == Regression Test: https://github.com/facebook/flow/issues/359 == //\n// Ensure that type bindings stay type bindings across function body  //\n// env contexts.                                                      //\n////////////////////////////////////////////////////////////////////////\n\nimport type ClassFoo6 from \"./issue-359\";\nfunction foo() {\n  ClassFoo6; // Error: Not a value binding\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\n/////////////////////////////////////////////////\n// == Importing Class Type (Default Export) == //\n/////////////////////////////////////////////////\n\nimport type ClassFoo1 from \"./ExportDefault_Class\";\nimport { foo1Inst } from \"./ExportDefault_Class\";\n\nvar a1: ClassFoo1 = foo1Inst;\nvar a2: number = foo1Inst; // Error: ClassFoo1 ~> number\nnew ClassFoo1(); // Error: ClassFoo1 is not a value-identifier\n\n///////////////////////////////////////////////\n// == Importing Class Type (Named Export) == //\n///////////////////////////////////////////////\n\nimport type { ClassFoo2 } from \"./ExportNamed_Class\";\nimport { foo2Inst } from \"./ExportNamed_Class\";\n\nvar b1: ClassFoo2 = foo2Inst;\nvar b2: number = foo2Inst; // Error: ClassFoo2 ~> number\nnew ClassFoo2(); // Error: ClassFoo2 is not a value-identifier\n\n/////////////////////////////////////////////////////\n// == Importing Class Type (CJS Default Export) == //\n/////////////////////////////////////////////////////\n\nimport type ClassFoo3T from \"./ExportCJSDefault_Class\";\nimport ClassFoo3 from \"./ExportCJSDefault_Class\";\nvar c1: ClassFoo3T = new ClassFoo3();\nnew ClassFoo3T(); // Error: ClassFoo3 is not a value-identifier\n\n///////////////////////////////////////////////////\n// == Importing Class Type (CJS Named Export) == //\n///////////////////////////////////////////////////\n\nimport type { ClassFoo4, ClassFoo5 } from \"./ExportCJSNamed_Class\";\nimport { foo4Inst, foo5Inst } from \"./ExportCJSNamed_Class\";\n\nvar d1: ClassFoo4 = foo4Inst;\nvar d2: number = foo4Inst; // Error: ClassFoo4 ~> number\nnew ClassFoo4(); // Error: ClassFoo4 is not a value-identifier\n// TODO: this errors correctly, but the message is just 'can't resolve name'\nvar d3: typeof ClassFoo5 = foo5Inst; // Error: Can't typeof a type alias\n\n////////////////////////////////////////////\n// == Import Type Alias (Named Export) == //\n////////////////////////////////////////////\n\nimport type { AliasFoo3 } from \"./ExportNamed_Alias\";\nimport { givesAFoo3Obj } from \"./ExportNamed_Alias\";\nvar e1: AliasFoo3 = givesAFoo3Obj();\nvar e2: number = givesAFoo3Obj(); // Error: AliasFoo3 ~> number\nvar e3: typeof AliasFoo3 = givesAFoo3Obj(); // Error: Can't typeof a type alias\n\n//////////////////////////////////////////////\n// == Import Type Alias (Default Export) == //\n//////////////////////////////////////////////\n\n// TODO: No support for this right now. It's most likely possible, but it's\n//       unclear how useful it is at the moment and it entails a little\n//       more work than named type exports, so I'm punting on it for now.\n\n///////////////////////////////////////////////////////\n// == Import Type With Non-Alias Compatible Value == //\n///////////////////////////////////////////////////////\n\nimport type { numValue } from \"./ExportsANumber\"; // Error: Cannot import-type a number value\n\n////////////////////////////////////////////////////////////////////////\n// == Regression Test: https://github.com/facebook/flow/issues/359 == //\n// Ensure that type bindings stay type bindings across function body  //\n// env contexts.                                                      //\n////////////////////////////////////////////////////////////////////////\n\nimport type ClassFoo6 from \"./issue-359\";\nfunction foo() {\n  ClassFoo6; // Error: Not a value binding\n}");
}
#[test]
fn test_issue_359_js_format_1_da505398() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\n\nclass ClassFoo6 {};\n\nexport default ClassFoo6;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nclass ClassFoo6 {}\n\nexport default ClassFoo6;"
    );
}
