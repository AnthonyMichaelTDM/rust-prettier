#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_b_js_format_1_d525903c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nexports.numberValue = 42;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */\n\nexports.numberValue = 42;");
}
#[test]
fn test_c_js_format_1_e3121712() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */");
}
#[test]
fn test_common_js_clobbering_class_js_format_1_e58001ae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule CommonJS_Clobbering_Class\n * @flow\n */\n\nclass Base {\n  static baseProp: number;\n}\n\nclass Test extends Base {\n  static childProp: number;\n\n  static staticNumber1():number { return 1; }\n  static staticNumber2():number { return 2; }\n  static staticNumber3():number { return 3; }\n\n  instNumber1():number { return 1; }\n  instNumber2():number { return 2; }\n};\n\nmodule.exports = Test;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule CommonJS_Clobbering_Class\n * @flow\n */\n\nclass Base {\n  static baseProp: number;\n}\n\nclass Test extends Base {\n  static childProp: number;\n\n  static staticNumber1(): number {\n    return 1;\n  }\n  static staticNumber2(): number {\n    return 2;\n  }\n  static staticNumber3(): number {\n    return 3;\n  }\n\n  instNumber1(): number {\n    return 1;\n  }\n  instNumber2(): number {\n    return 2;\n  }\n}\n\nmodule.exports = Test;");
}
#[test]
fn test_common_js_clobbering_lit_js_format_1_5b46daa6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule CommonJS_Clobbering_Lit\n * @flow\n */\n\nmodule.exports = {\n  numberValue1: 1,\n  numberValue2: 2,\n  numberValue3: 3,\n  numberValue4: 4,\n  numberValue5: 5\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule CommonJS_Clobbering_Lit\n * @flow\n */\n\nmodule.exports = {\n  numberValue1: 1,\n  numberValue2: 2,\n  numberValue3: 3,\n  numberValue4: 4,\n  numberValue5: 5,\n};");
}
#[test]
fn test_common_js_named_js_format_1_b3c229d0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule CommonJS_Named\n * @flow\n */\n\nexports.numberValue1 = 1;\nexports.numberValue2 = 2;\nexports.numberValue3 = 3;\nexports.numberValue4 = 4;\nexports.numberValue5 = 5;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule CommonJS_Named\n * @flow\n */\n\nexports.numberValue1 = 1;\nexports.numberValue2 = 2;\nexports.numberValue3 = 3;\nexports.numberValue4 = 4;\nexports.numberValue5 = 5;");
}
#[test]
fn test_es_6_default_anon_function_1_js_format_1_07bc6c11() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_AnonFunction1\n * @flow\n */\n\ndeclare export default () => number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_AnonFunction1\n * @flow\n */\n\ndeclare export default () => number;");
}
#[test]
fn test_es_6_default_anon_function_2_js_format_1_c34a71f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_AnonFunction2\n * @flow\n */\n\ndeclare export default () =>number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_AnonFunction2\n * @flow\n */\n\ndeclare export default () => number;");
}
#[test]
fn test_es_6_default_named_class_1_js_format_1_ef8d826f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_NamedClass1\n * @flow\n */\n\ndeclare export default class FooImpl { givesANum(): number; };\n\n// Regression test for https://github.com/facebook/flow/issues/511\n//\n// Default-exported class should also be available in local scope\ndeclare export { FooImpl as Foo }\ndeclare export function getAFoo(): FooImpl;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_NamedClass1\n * @flow\n */\n\ndeclare export default class FooImpl {\n  givesANum(): number;\n}\n\n// Regression test for https://github.com/facebook/flow/issues/511\n//\n// Default-exported class should also be available in local scope\ndeclare export { FooImpl as Foo };\ndeclare export function getAFoo(): FooImpl;");
}
#[test]
fn test_es_6_default_named_class_2_js_format_1_eb928d58() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_NamedClass2\n * @flow\n */\n\ndeclare export default class Foo { givesANum(): number; };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_NamedClass2\n * @flow\n */\n\ndeclare export default class Foo {\n  givesANum(): number;\n}");
}
#[test]
fn test_es_6_default_named_function_1_js_format_1_c0f9b5b3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_NamedFunction1\n * @flow\n */\n\ndeclare export default function foo():number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_NamedFunction1\n * @flow\n */\n\ndeclare export default function foo(): number;");
}
#[test]
fn test_es_6_default_named_function_2_js_format_1_a49a55d7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_NamedFunction2\n * @flow\n */\n\ndeclare export default function foo():number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_NamedFunction2\n * @flow\n */\n\ndeclare export default function foo(): number;");
}
#[test]
fn test_es_6_default_and_named_js_format_1_08913289() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/* @flow */\n\ndeclare export default number;\ndeclare export var str: string;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\ndeclare export default number;\ndeclare export var str: string;"
    );
}
#[test]
fn test_es_6_export_all_from_intermediary_1_js_format_1_873217e7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportAllFrom_Intermediary1\n * @flow\n */\n\ndeclare export * from \"ES6_ExportAllFrom_Source1\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportAllFrom_Intermediary1\n * @flow\n */\n\ndeclare export * from \"ES6_ExportAllFrom_Source1\";");
}
#[test]
fn test_es_6_export_all_from_intermediary_2_js_format_1_d62c4095() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportAllFrom_Intermediary2\n * @flow\n */\n\ndeclare export * from \"ES6_ExportAllFrom_Source2\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportAllFrom_Intermediary2\n * @flow\n */\n\ndeclare export * from \"ES6_ExportAllFrom_Source2\";");
}
#[test]
fn test_es_6_export_all_from_source_1_js_format_1_3c2b925a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportAllFrom_Source1\n * @flow\n */\n\ndeclare export var numberValue1: number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportAllFrom_Source1\n * @flow\n */\n\ndeclare export var numberValue1: number;");
}
#[test]
fn test_es_6_export_all_from_source_2_js_format_1_15c4bd28() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportAllFrom_Source2\n * @flow\n */\n\ndeclare export var numberValue2: number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportAllFrom_Source2\n * @flow\n */\n\ndeclare export var numberValue2: number;");
}
#[test]
fn test_es_6_export_all_from_multi_js_format_1_15aa7a10() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare export * from \"./ES6_ExportAllFrom_Source1\";\ndeclare export * from \"./ES6_ExportAllFrom_Source2\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare export * from \"./ES6_ExportAllFrom_Source1\";\ndeclare export * from \"./ES6_ExportAllFrom_Source2\";");
}
#[test]
fn test_es_6_export_from_intermediary_1_js_format_1_1ab4cdfe() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportFrom_Intermediary1\n * @flow\n */\n\ndeclare export {\n  numberValue1,\n  numberValue2 as numberValue2_renamed\n} from \"ES6_ExportFrom_Source1\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportFrom_Intermediary1\n * @flow\n */\n\ndeclare export {\n  numberValue1,\n  numberValue2 as numberValue2_renamed,\n} from \"ES6_ExportFrom_Source1\";");
}
#[test]
fn test_es_6_export_from_intermediary_2_js_format_1_e6d9ac52() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportFrom_Intermediary2\n * @flow\n */\n\ndeclare export {\n  numberValue1,\n  numberValue2 as numberValue2_renamed2\n} from \"ES6_ExportFrom_Source2\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportFrom_Intermediary2\n * @flow\n */\n\ndeclare export {\n  numberValue1,\n  numberValue2 as numberValue2_renamed2,\n} from \"ES6_ExportFrom_Source2\";");
}
#[test]
fn test_es_6_export_from_source_1_js_format_1_126cab82() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportFrom_Source1\n * @flow\n */\n\ndeclare export var numberValue1: number;\ndeclare export var numberValue2: number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportFrom_Source1\n * @flow\n */\n\ndeclare export var numberValue1: number;\ndeclare export var numberValue2: number;");
}
#[test]
fn test_es_6_export_from_source_2_js_format_1_e86525e1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportFrom_Source2\n * @flow\n */\n\ndeclare export var numberValue1: number;\ndeclare export var numberValue2: number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportFrom_Source2\n * @flow\n */\n\ndeclare export var numberValue1: number;\ndeclare export var numberValue2: number;");
}
#[test]
fn test_es_6_named_1_js_format_1_390f4650() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Named1\n * @flow\n */\n\nvar specifierNumber1 = 1;\nvar specifierNumber2 = 2;\nvar specifierNumber3 = 3;\nvar groupedSpecifierNumber1 = 1;\nvar groupedSpecifierNumber2 = 2;\n\ndeclare export {specifierNumber1};\ndeclare export {specifierNumber2 as specifierNumber2Renamed};\ndeclare export {specifierNumber3};\ndeclare export {groupedSpecifierNumber1, groupedSpecifierNumber2};\n\ndeclare export function givesANumber(): number;\ndeclare export class NumberGenerator { givesANumber(): number; };\n\ndeclare export var varDeclNumber1: number;\ndeclare export var varDeclNumber2: number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Named1\n * @flow\n */\n\nvar specifierNumber1 = 1;\nvar specifierNumber2 = 2;\nvar specifierNumber3 = 3;\nvar groupedSpecifierNumber1 = 1;\nvar groupedSpecifierNumber2 = 2;\n\ndeclare export { specifierNumber1 };\ndeclare export { specifierNumber2 as specifierNumber2Renamed };\ndeclare export { specifierNumber3 };\ndeclare export { groupedSpecifierNumber1, groupedSpecifierNumber2 };\n\ndeclare export function givesANumber(): number;\ndeclare export class NumberGenerator {\n  givesANumber(): number;\n}\n\ndeclare export var varDeclNumber1: number;\ndeclare export var varDeclNumber2: number;");
}
#[test]
fn test_es_6_named_2_js_format_1_1b28aa44() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Named2\n * @flow\n */\n\nvar specifierNumber4 = 1;\nvar specifierNumber5 = 2;\nvar groupedSpecifierNumber3 = 1;\nvar groupedSpecifierNumber4 = 2;\n\ndeclare export {specifierNumber4};\ndeclare export {specifierNumber5 as specifierNumber5Renamed};\ndeclare export {groupedSpecifierNumber3, groupedSpecifierNumber4};\n\ndeclare export function givesANumber2(): number;\ndeclare export class NumberGenerator2 { givesANumber(): number; };\n\ndeclare export var varDeclNumber3: number;\ndeclare export var varDeclNumber4: number;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Named2\n * @flow\n */\n\nvar specifierNumber4 = 1;\nvar specifierNumber5 = 2;\nvar groupedSpecifierNumber3 = 1;\nvar groupedSpecifierNumber4 = 2;\n\ndeclare export { specifierNumber4 };\ndeclare export { specifierNumber5 as specifierNumber5Renamed };\ndeclare export { groupedSpecifierNumber3, groupedSpecifierNumber4 };\n\ndeclare export function givesANumber2(): number;\ndeclare export class NumberGenerator2 {\n  givesANumber(): number;\n}\n\ndeclare export var varDeclNumber3: number;\ndeclare export var varDeclNumber4: number;");
}
#[test]
fn test_provides_module_a_js_format_1_9401784a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule A\n * @flow\n */\n\nexports.numberValue1 = 42;\nexports.numberValue2 = 42;\nexports.numberValue3 = 42;\nexports.numberValue4 = 42;\nexports.stringValue = \"str\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule A\n * @flow\n */\n\nexports.numberValue1 = 42;\nexports.numberValue2 = 42;\nexports.numberValue3 = 42;\nexports.numberValue4 = 42;\nexports.stringValue = \"str\";");
}
#[test]
fn test_provides_module_cjs_default_js_format_1_584e99db() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule CJSDefault\n * @flow\n */\n\nmodule.exports = {\n  numberValue: 42\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule CJSDefault\n * @flow\n */\n\nmodule.exports = {\n  numberValue: 42,\n};");
}
#[test]
fn test_provides_module_d_js_format_1_acd1bf89() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/**\n * @providesModule D\n * @flow\n */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/**\n * @providesModule D\n * @flow\n */");
}
#[test]
fn test_provides_module_es_6_default_js_format_1_bf09781d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6Default\n * @flow\n */\n\n/*\nexport default {\n  numberValue: 42,\n};\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6Default\n * @flow\n */\n\n/*\nexport default {\n  numberValue: 42,\n};\n*/");
}
#[test]
fn test_side_effects_js_format_1_e3121712() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* @flow */");
}
#[test]
fn test_es_6_modules_js_format_1_b2535cdd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n// ===================== //\n// == Path Resolution == //\n// ===================== //\n\n// @providesModule\nimport * as DefaultA from \"A\";\nvar a1: number = DefaultA.numberValue1;\nvar a2: string = DefaultA.numberValue1; // Error: number ~> string\n\n// File path\nimport * as DefaultB from \"./B\";\nvar b1: number = DefaultB.numberValue;\nvar b2: string = DefaultB.numberValue; // Error: number ~> string\n\n// C.js exists, but not as a providesModule\nimport DefaultC from \"C\"; // Error: No such module\n\n// @providesModule D exists, but not as a filename\nimport DefaultD from \"./D\"; // Error: No such module\n\n// ================================================ //\n// == CommonJS Clobbering Literal Exports -> ES6 == //\n// ================================================ //\n\nimport {doesntExist1} from \"CommonJS_Clobbering_Lit\"; // Error: Not an exported binding\n\nimport {numberValue1} from \"CommonJS_Clobbering_Lit\";\nvar c1: number = numberValue1;\nvar c2: string = numberValue1; // Error: number ~> string\n\nimport {numberValue2 as numVal1} from \"CommonJS_Clobbering_Lit\";\nvar d1: number = numVal1;\nvar d2: string = numVal1; // Error: number ~> string\n\nimport CJS_Clobb_Lit from \"CommonJS_Clobbering_Lit\";\nvar e1: number = CJS_Clobb_Lit.numberValue3;\nvar e2: string = CJS_Clobb_Lit.numberValue3; // Error: number ~> string\nCJS_Clobb_Lit.doesntExist; // Error: doesntExist isn't a property\n\nimport * as CJS_Clobb_Lit_NS from \"CommonJS_Clobbering_Lit\";\nvar f1: number = CJS_Clobb_Lit_NS.numberValue4;\nvar f2: number = CJS_Clobb_Lit_NS.default.numberValue4;\nCJS_Clobb_Lit_NS.default.default; // Error: No 'default' property on the exported obj\nvar f3: string = CJS_Clobb_Lit_NS.numberValue4; // Error: number ~> string\nvar f4: string = CJS_Clobb_Lit_NS.default.numberValue5; // Error: number ~> string\n\n// ============================================== //\n// == CommonJS Clobbering Class Exports -> ES6 == //\n// ============================================== //\n\nimport {doesntExist2} from \"CommonJS_Clobbering_Class\"; // Error: Not an exported binding\n\n// The following import should error because class statics are not turned into\n// named exports for now. This avoids complexities with polymorphic static\n// members (where the polymophism is defined on the class itself rather than the\n// method).\nimport {staticNumber1, baseProp, childProp} from \"CommonJS_Clobbering_Class\"; // Error\n\nimport CJS_Clobb_Class from \"CommonJS_Clobbering_Class\";\nnew CJS_Clobb_Class();\nnew CJS_Clobb_Class().doesntExist; // Error: Class has no \\`doesntExist\\` property\nvar h1: number = CJS_Clobb_Class.staticNumber2();\nvar h2: string = CJS_Clobb_Class.staticNumber2(); // Error: number ~> string\nvar h3: number = new CJS_Clobb_Class().instNumber1();\nvar h4: string = new CJS_Clobb_Class().instNumber1(); // Error: number ~> string\n\nimport * as CJS_Clobb_Class_NS from \"CommonJS_Clobbering_Class\";\nnew CJS_Clobb_Class_NS(); // Error: Namespace object isn't constructable\nvar i1: number = CJS_Clobb_Class_NS.staticNumber3(); // Error: Class statics not copied to Namespace object\nvar i2: number = new CJS_Clobb_Class_NS.default().instNumber2();\nvar i3: string = new CJS_Clobb_Class_NS.default().instNumber2(); // Error: number ~> string\n\n// =================================== //\n// == CommonJS Named Exports -> ES6 == //\n// =================================== //\n\nimport {doesntExist3} from \"CommonJS_Named\"; // Error: Not an exported binding\n\nimport {numberValue2} from \"CommonJS_Named\";\nvar j1: number = numberValue2;\nvar j2: string = numberValue2; // Error: number ~> string\n\nimport {numberValue3 as numVal3} from \"CommonJS_Named\";\nvar k1: number = numVal3;\nvar k2: string = numVal3; // Error: number ~> string\n\nimport * as CJS_Named from \"CommonJS_Named\";\nvar l1: number = CJS_Named.numberValue1;\nvar l2: string = CJS_Named.numberValue1; // Error: number ~> string\nCJS_Named.doesntExist; // Error: doesntExist isn't a property\n\nimport * as CJS_Named_NS from \"CommonJS_Named\";\nvar m1: number = CJS_Named_NS.numberValue4;\nvar m2: string = CJS_Named_NS.default.numberValue4; // Error: CommonJS_Named has no default export\nvar m3: string = CJS_Named_NS.numberValue4; // Error: number ~> string\n\n//////////////////////////////\n// == ES6 Default -> ES6 == //\n//////////////////////////////\n\nimport {doesntExist4} from \"ES6_Default_AnonFunction1\"; // Error: Not an exported binding\n\nimport ES6_Def_AnonFunc1 from \"ES6_Default_AnonFunction1\";\nvar n1: number = ES6_Def_AnonFunc1();\nvar n2: string = ES6_Def_AnonFunc1(); // Error: number ~> string\n\nimport ES6_Def_NamedFunc1 from \"ES6_Default_NamedFunction1\";\nvar o1: number = ES6_Def_NamedFunc1();\nvar o2: string = ES6_Def_NamedFunc1(); // Error: number ~> string\n\n\n\n\n\nimport ES6_Def_NamedClass1 from \"ES6_Default_NamedClass1\";\nvar q1: number = new ES6_Def_NamedClass1().givesANum();\nvar q2: string = new ES6_Def_NamedClass1().givesANum(); // Error: number ~> string\n\n////////////////////////////\n// == ES6 Named -> ES6 == //\n////////////////////////////\n\nimport doesntExist5 from \"ES6_Named1\"; // Error: Not an exported binding\n\nimport {specifierNumber1 as specifierNumber1_1} from \"ES6_Named1\";\nvar r1: number = specifierNumber1_1;\nvar r2: string = specifierNumber1_1; // Error: number ~> string\n\nimport {specifierNumber2Renamed} from \"ES6_Named1\";\nvar s1: number = specifierNumber2Renamed;\nvar s2: string = specifierNumber2Renamed; // Error: number ~> string\n\nimport {specifierNumber3 as specifierNumber3Renamed} from \"ES6_Named1\";\nvar t1: number = specifierNumber3Renamed;\nvar t2: string = specifierNumber3Renamed; // Error: number ~> string\n\nimport {groupedSpecifierNumber1, groupedSpecifierNumber2} from \"ES6_Named1\";\nvar u1: number = groupedSpecifierNumber1;\nvar u2: number = groupedSpecifierNumber2;\nvar u3: string = groupedSpecifierNumber1; // Error: number ~> string\nvar u4: string = groupedSpecifierNumber2; // Error: number ~> string\n\nimport {givesANumber} from \"ES6_Named1\";\nvar v1: number = givesANumber();\nvar v2: string = givesANumber(); // Error: number ~> string\n\nimport {NumberGenerator} from \"ES6_Named1\";\nvar w1: number = new NumberGenerator().givesANumber();\nvar w2: string = new NumberGenerator().givesANumber(); // Error: number ~> string\n\nimport {varDeclNumber1, varDeclNumber2} from \"ES6_Named1\";\nvar x1: number = varDeclNumber1;\nvar x2: number = varDeclNumber2;\nvar x3: string = varDeclNumber1; // Error: number ~> string\nvar x4: string = varDeclNumber2; // Error: number ~> string\n\n\n\n\n\n\n\n\n\nimport {numberValue1 as numberValue4} from \"ES6_ExportFrom_Intermediary1\";\nvar aa1: number = numberValue4;\nvar aa2: string = numberValue4; // Error: number ~> string\n\nimport {numberValue2_renamed} from \"ES6_ExportFrom_Intermediary1\";\nvar ab1: number = numberValue2_renamed;\nvar ab2: string = numberValue2_renamed; // Error: number ~> string\n\nimport {numberValue1 as numberValue5} from \"ES6_ExportAllFrom_Intermediary1\";\nvar ac1: number = numberValue5;\nvar ac2: string = numberValue5; // Error: number ~> string\n\n///////////////////////////////////\n// == ES6 Default -> CommonJS == //\n///////////////////////////////////\n\nrequire('ES6_Default_AnonFunction2').doesntExist; // Error: 'doesntExist' isn't an export\n\nvar ES6_Def_AnonFunc2 = require(\"ES6_Default_AnonFunction2\").default;\nvar ad1: number = ES6_Def_AnonFunc2();\nvar ad2: string = ES6_Def_AnonFunc2(); // Error: number ~> string\n\nvar ES6_Def_NamedFunc2 = require(\"ES6_Default_NamedFunction2\").default;\nvar ae1: number = ES6_Def_NamedFunc2();\nvar ae2: string = ES6_Def_NamedFunc2(); // Error: number ~> string\n\n\n\n\n\nvar ES6_Def_NamedClass2 = require(\"ES6_Default_NamedClass2\").default;\nvar ag1: number = new ES6_Def_NamedClass2().givesANum();\nvar ag2: string = new ES6_Def_NamedClass2().givesANum(); // Error: number ~> string\n\n/////////////////////////////////\n// == ES6 Named -> CommonJS == //\n/////////////////////////////////\n\nvar specifierNumber4 = require(\"ES6_Named2\").specifierNumber4;\nvar ah1: number = specifierNumber4;\nvar ah2: string = specifierNumber4; // Error: number ~> string\n\nvar specifierNumber5Renamed = require(\"ES6_Named2\").specifierNumber5Renamed;\nvar ai1: number = specifierNumber5Renamed;\nvar ai2: string = specifierNumber5Renamed; // Error: number ~> string\n\nvar groupedSpecifierNumber3 = require(\"ES6_Named2\").groupedSpecifierNumber3;\nvar groupedSpecifierNumber4 = require(\"ES6_Named2\").groupedSpecifierNumber4;\nvar aj1: number = groupedSpecifierNumber3;\nvar aj2: number = groupedSpecifierNumber4;\nvar aj3: string = groupedSpecifierNumber3; // Error: number ~> string\nvar aj4: string = groupedSpecifierNumber4; // Error: number ~> string\n\nvar givesANumber2 = require(\"ES6_Named2\").givesANumber2;\nvar ak1: number = givesANumber2();\nvar ak2: string = givesANumber2(); // Error: number ~> string\n\nvar NumberGenerator2 = require(\"ES6_Named2\").NumberGenerator2;\nvar al1: number = new NumberGenerator2().givesANumber();\nvar al2: string = new NumberGenerator2().givesANumber(); // Error: number ~> string\n\nvar varDeclNumber3 = require(\"ES6_Named2\").varDeclNumber3;\nvar varDeclNumber4 = require(\"ES6_Named2\").varDeclNumber4;\nvar am1: number = varDeclNumber3;\nvar am2: number = varDeclNumber4;\nvar am3: string = varDeclNumber3; // Error: number ~> string\nvar am4: string = varDeclNumber4; // Error: number ~> string\n\n\n\n\n\n\n\n\n\nvar numberValue6 = require(\"ES6_ExportFrom_Intermediary2\").numberValue1;\nvar ap1: number = numberValue6;\nvar ap2: string = numberValue6; // Error: number ~> string\n\nvar numberValue2_renamed2 = require(\"ES6_ExportFrom_Intermediary2\").numberValue2_renamed2;\nvar aq1: number = numberValue2_renamed2;\nvar aq2: string = numberValue2_renamed2; // Error: number ~> string\n\nvar numberValue7 = require(\"ES6_ExportAllFrom_Intermediary2\").numberValue2;\nvar ar1: number = numberValue7;\nvar ar2: string = numberValue7; // Error: number ~> string\n\n////////////////////////////////////////////////////////\n// == ES6 Default+Named -> ES6 import Default+Named== //\n////////////////////////////////////////////////////////\n\nimport defaultNum, {str as namedStr} from \"./ES6_DefaultAndNamed\";\n\nvar as1: number = defaultNum;\nvar as2: string = defaultNum; // Error: number ~> string\n\nvar as3: string = namedStr;\nvar as4: number = namedStr; // Error: string ~> number\n\n////////////////////////////////////////\n// == Side-effect only ES6 imports == //\n////////////////////////////////////////\n\nimport \"./SideEffects\";\n\n//////////////////////////////////////////////\n// == Suggest export name on likely typo == //\n//////////////////////////////////////////////\nimport specifierNumber1 from \"ES6_Named1\"; // Error: Did you mean \\`import {specifierNumber1} from ...\\`?\nimport {specifierNumber} from \"ES6_Named1\"; // Error: Did you mean \\`specifierNumber1\\`?\n\n///////////////////////////////////////////////////\n// == Multi \\`export *\\` should combine exports == //\n///////////////////////////////////////////////////\nimport {\n  numberValue1 as numberValue8,\n  numberValue2 as numberValue9\n} from \"./ES6_ExportAllFromMulti\";\n\nvar at1: number = numberValue8;\nvar at2: string = numberValue8; // Error: number ~> string\n\nvar at3: number = numberValue9;\nvar at4: string = numberValue9; // Error: number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n// ===================== //\n// == Path Resolution == //\n// ===================== //\n\n// @providesModule\nimport * as DefaultA from \"A\";\nvar a1: number = DefaultA.numberValue1;\nvar a2: string = DefaultA.numberValue1; // Error: number ~> string\n\n// File path\nimport * as DefaultB from \"./B\";\nvar b1: number = DefaultB.numberValue;\nvar b2: string = DefaultB.numberValue; // Error: number ~> string\n\n// C.js exists, but not as a providesModule\nimport DefaultC from \"C\"; // Error: No such module\n\n// @providesModule D exists, but not as a filename\nimport DefaultD from \"./D\"; // Error: No such module\n\n// ================================================ //\n// == CommonJS Clobbering Literal Exports -> ES6 == //\n// ================================================ //\n\nimport { doesntExist1 } from \"CommonJS_Clobbering_Lit\"; // Error: Not an exported binding\n\nimport { numberValue1 } from \"CommonJS_Clobbering_Lit\";\nvar c1: number = numberValue1;\nvar c2: string = numberValue1; // Error: number ~> string\n\nimport { numberValue2 as numVal1 } from \"CommonJS_Clobbering_Lit\";\nvar d1: number = numVal1;\nvar d2: string = numVal1; // Error: number ~> string\n\nimport CJS_Clobb_Lit from \"CommonJS_Clobbering_Lit\";\nvar e1: number = CJS_Clobb_Lit.numberValue3;\nvar e2: string = CJS_Clobb_Lit.numberValue3; // Error: number ~> string\nCJS_Clobb_Lit.doesntExist; // Error: doesntExist isn't a property\n\nimport * as CJS_Clobb_Lit_NS from \"CommonJS_Clobbering_Lit\";\nvar f1: number = CJS_Clobb_Lit_NS.numberValue4;\nvar f2: number = CJS_Clobb_Lit_NS.default.numberValue4;\nCJS_Clobb_Lit_NS.default.default; // Error: No 'default' property on the exported obj\nvar f3: string = CJS_Clobb_Lit_NS.numberValue4; // Error: number ~> string\nvar f4: string = CJS_Clobb_Lit_NS.default.numberValue5; // Error: number ~> string\n\n// ============================================== //\n// == CommonJS Clobbering Class Exports -> ES6 == //\n// ============================================== //\n\nimport { doesntExist2 } from \"CommonJS_Clobbering_Class\"; // Error: Not an exported binding\n\n// The following import should error because class statics are not turned into\n// named exports for now. This avoids complexities with polymorphic static\n// members (where the polymophism is defined on the class itself rather than the\n// method).\nimport { staticNumber1, baseProp, childProp } from \"CommonJS_Clobbering_Class\"; // Error\n\nimport CJS_Clobb_Class from \"CommonJS_Clobbering_Class\";\nnew CJS_Clobb_Class();\nnew CJS_Clobb_Class().doesntExist; // Error: Class has no \\`doesntExist\\` property\nvar h1: number = CJS_Clobb_Class.staticNumber2();\nvar h2: string = CJS_Clobb_Class.staticNumber2(); // Error: number ~> string\nvar h3: number = new CJS_Clobb_Class().instNumber1();\nvar h4: string = new CJS_Clobb_Class().instNumber1(); // Error: number ~> string\n\nimport * as CJS_Clobb_Class_NS from \"CommonJS_Clobbering_Class\";\nnew CJS_Clobb_Class_NS(); // Error: Namespace object isn't constructable\nvar i1: number = CJS_Clobb_Class_NS.staticNumber3(); // Error: Class statics not copied to Namespace object\nvar i2: number = new CJS_Clobb_Class_NS.default().instNumber2();\nvar i3: string = new CJS_Clobb_Class_NS.default().instNumber2(); // Error: number ~> string\n\n// =================================== //\n// == CommonJS Named Exports -> ES6 == //\n// =================================== //\n\nimport { doesntExist3 } from \"CommonJS_Named\"; // Error: Not an exported binding\n\nimport { numberValue2 } from \"CommonJS_Named\";\nvar j1: number = numberValue2;\nvar j2: string = numberValue2; // Error: number ~> string\n\nimport { numberValue3 as numVal3 } from \"CommonJS_Named\";\nvar k1: number = numVal3;\nvar k2: string = numVal3; // Error: number ~> string\n\nimport * as CJS_Named from \"CommonJS_Named\";\nvar l1: number = CJS_Named.numberValue1;\nvar l2: string = CJS_Named.numberValue1; // Error: number ~> string\nCJS_Named.doesntExist; // Error: doesntExist isn't a property\n\nimport * as CJS_Named_NS from \"CommonJS_Named\";\nvar m1: number = CJS_Named_NS.numberValue4;\nvar m2: string = CJS_Named_NS.default.numberValue4; // Error: CommonJS_Named has no default export\nvar m3: string = CJS_Named_NS.numberValue4; // Error: number ~> string\n\n//////////////////////////////\n// == ES6 Default -> ES6 == //\n//////////////////////////////\n\nimport { doesntExist4 } from \"ES6_Default_AnonFunction1\"; // Error: Not an exported binding\n\nimport ES6_Def_AnonFunc1 from \"ES6_Default_AnonFunction1\";\nvar n1: number = ES6_Def_AnonFunc1();\nvar n2: string = ES6_Def_AnonFunc1(); // Error: number ~> string\n\nimport ES6_Def_NamedFunc1 from \"ES6_Default_NamedFunction1\";\nvar o1: number = ES6_Def_NamedFunc1();\nvar o2: string = ES6_Def_NamedFunc1(); // Error: number ~> string\n\nimport ES6_Def_NamedClass1 from \"ES6_Default_NamedClass1\";\nvar q1: number = new ES6_Def_NamedClass1().givesANum();\nvar q2: string = new ES6_Def_NamedClass1().givesANum(); // Error: number ~> string\n\n////////////////////////////\n// == ES6 Named -> ES6 == //\n////////////////////////////\n\nimport doesntExist5 from \"ES6_Named1\"; // Error: Not an exported binding\n\nimport { specifierNumber1 as specifierNumber1_1 } from \"ES6_Named1\";\nvar r1: number = specifierNumber1_1;\nvar r2: string = specifierNumber1_1; // Error: number ~> string\n\nimport { specifierNumber2Renamed } from \"ES6_Named1\";\nvar s1: number = specifierNumber2Renamed;\nvar s2: string = specifierNumber2Renamed; // Error: number ~> string\n\nimport { specifierNumber3 as specifierNumber3Renamed } from \"ES6_Named1\";\nvar t1: number = specifierNumber3Renamed;\nvar t2: string = specifierNumber3Renamed; // Error: number ~> string\n\nimport { groupedSpecifierNumber1, groupedSpecifierNumber2 } from \"ES6_Named1\";\nvar u1: number = groupedSpecifierNumber1;\nvar u2: number = groupedSpecifierNumber2;\nvar u3: string = groupedSpecifierNumber1; // Error: number ~> string\nvar u4: string = groupedSpecifierNumber2; // Error: number ~> string\n\nimport { givesANumber } from \"ES6_Named1\";\nvar v1: number = givesANumber();\nvar v2: string = givesANumber(); // Error: number ~> string\n\nimport { NumberGenerator } from \"ES6_Named1\";\nvar w1: number = new NumberGenerator().givesANumber();\nvar w2: string = new NumberGenerator().givesANumber(); // Error: number ~> string\n\nimport { varDeclNumber1, varDeclNumber2 } from \"ES6_Named1\";\nvar x1: number = varDeclNumber1;\nvar x2: number = varDeclNumber2;\nvar x3: string = varDeclNumber1; // Error: number ~> string\nvar x4: string = varDeclNumber2; // Error: number ~> string\n\nimport { numberValue1 as numberValue4 } from \"ES6_ExportFrom_Intermediary1\";\nvar aa1: number = numberValue4;\nvar aa2: string = numberValue4; // Error: number ~> string\n\nimport { numberValue2_renamed } from \"ES6_ExportFrom_Intermediary1\";\nvar ab1: number = numberValue2_renamed;\nvar ab2: string = numberValue2_renamed; // Error: number ~> string\n\nimport { numberValue1 as numberValue5 } from \"ES6_ExportAllFrom_Intermediary1\";\nvar ac1: number = numberValue5;\nvar ac2: string = numberValue5; // Error: number ~> string\n\n///////////////////////////////////\n// == ES6 Default -> CommonJS == //\n///////////////////////////////////\n\nrequire(\"ES6_Default_AnonFunction2\").doesntExist; // Error: 'doesntExist' isn't an export\n\nvar ES6_Def_AnonFunc2 = require(\"ES6_Default_AnonFunction2\").default;\nvar ad1: number = ES6_Def_AnonFunc2();\nvar ad2: string = ES6_Def_AnonFunc2(); // Error: number ~> string\n\nvar ES6_Def_NamedFunc2 = require(\"ES6_Default_NamedFunction2\").default;\nvar ae1: number = ES6_Def_NamedFunc2();\nvar ae2: string = ES6_Def_NamedFunc2(); // Error: number ~> string\n\nvar ES6_Def_NamedClass2 = require(\"ES6_Default_NamedClass2\").default;\nvar ag1: number = new ES6_Def_NamedClass2().givesANum();\nvar ag2: string = new ES6_Def_NamedClass2().givesANum(); // Error: number ~> string\n\n/////////////////////////////////\n// == ES6 Named -> CommonJS == //\n/////////////////////////////////\n\nvar specifierNumber4 = require(\"ES6_Named2\").specifierNumber4;\nvar ah1: number = specifierNumber4;\nvar ah2: string = specifierNumber4; // Error: number ~> string\n\nvar specifierNumber5Renamed = require(\"ES6_Named2\").specifierNumber5Renamed;\nvar ai1: number = specifierNumber5Renamed;\nvar ai2: string = specifierNumber5Renamed; // Error: number ~> string\n\nvar groupedSpecifierNumber3 = require(\"ES6_Named2\").groupedSpecifierNumber3;\nvar groupedSpecifierNumber4 = require(\"ES6_Named2\").groupedSpecifierNumber4;\nvar aj1: number = groupedSpecifierNumber3;\nvar aj2: number = groupedSpecifierNumber4;\nvar aj3: string = groupedSpecifierNumber3; // Error: number ~> string\nvar aj4: string = groupedSpecifierNumber4; // Error: number ~> string\n\nvar givesANumber2 = require(\"ES6_Named2\").givesANumber2;\nvar ak1: number = givesANumber2();\nvar ak2: string = givesANumber2(); // Error: number ~> string\n\nvar NumberGenerator2 = require(\"ES6_Named2\").NumberGenerator2;\nvar al1: number = new NumberGenerator2().givesANumber();\nvar al2: string = new NumberGenerator2().givesANumber(); // Error: number ~> string\n\nvar varDeclNumber3 = require(\"ES6_Named2\").varDeclNumber3;\nvar varDeclNumber4 = require(\"ES6_Named2\").varDeclNumber4;\nvar am1: number = varDeclNumber3;\nvar am2: number = varDeclNumber4;\nvar am3: string = varDeclNumber3; // Error: number ~> string\nvar am4: string = varDeclNumber4; // Error: number ~> string\n\nvar numberValue6 = require(\"ES6_ExportFrom_Intermediary2\").numberValue1;\nvar ap1: number = numberValue6;\nvar ap2: string = numberValue6; // Error: number ~> string\n\nvar numberValue2_renamed2 =\n  require(\"ES6_ExportFrom_Intermediary2\").numberValue2_renamed2;\nvar aq1: number = numberValue2_renamed2;\nvar aq2: string = numberValue2_renamed2; // Error: number ~> string\n\nvar numberValue7 = require(\"ES6_ExportAllFrom_Intermediary2\").numberValue2;\nvar ar1: number = numberValue7;\nvar ar2: string = numberValue7; // Error: number ~> string\n\n////////////////////////////////////////////////////////\n// == ES6 Default+Named -> ES6 import Default+Named== //\n////////////////////////////////////////////////////////\n\nimport defaultNum, { str as namedStr } from \"./ES6_DefaultAndNamed\";\n\nvar as1: number = defaultNum;\nvar as2: string = defaultNum; // Error: number ~> string\n\nvar as3: string = namedStr;\nvar as4: number = namedStr; // Error: string ~> number\n\n////////////////////////////////////////\n// == Side-effect only ES6 imports == //\n////////////////////////////////////////\n\nimport \"./SideEffects\";\n\n//////////////////////////////////////////////\n// == Suggest export name on likely typo == //\n//////////////////////////////////////////////\nimport specifierNumber1 from \"ES6_Named1\"; // Error: Did you mean \\`import {specifierNumber1} from ...\\`?\nimport { specifierNumber } from \"ES6_Named1\"; // Error: Did you mean \\`specifierNumber1\\`?\n\n///////////////////////////////////////////////////\n// == Multi \\`export *\\` should combine exports == //\n///////////////////////////////////////////////////\nimport {\n  numberValue1 as numberValue8,\n  numberValue2 as numberValue9,\n} from \"./ES6_ExportAllFromMulti\";\n\nvar at1: number = numberValue8;\nvar at2: string = numberValue8; // Error: number ~> string\n\nvar at3: number = numberValue9;\nvar at4: string = numberValue9; // Error: number ~> string");
}
