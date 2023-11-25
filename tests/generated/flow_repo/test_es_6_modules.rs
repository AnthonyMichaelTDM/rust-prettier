use pretty_assertions::assert_eq;
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
fn test_common_js_clobbering_frozen_js_format_1_eb50de11() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule CommonJS_Clobbering_Frozen\n * @flow\n */\n\nmodule.exports = Object.freeze({\n  numberValue1: 1,\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule CommonJS_Clobbering_Frozen\n * @flow\n */\n\nmodule.exports = Object.freeze({\n  numberValue1: 1,\n});");
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
fn test_es_6_default_anon_class_1_js_format_1_44f5b8c6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_AnonClass1\n * @flow\n */\n\nexport default class { givesANum(): number { return 42; }};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_AnonClass1\n * @flow\n */\n\nexport default class {\n  givesANum(): number {\n    return 42;\n  }\n}");
}
#[test]
fn test_es_6_default_anon_class_2_js_format_1_7f824419() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_AnonClass2\n * @flow\n */\n\nexport default class { givesANum(): number { return 42; }};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_AnonClass2\n * @flow\n */\n\nexport default class {\n  givesANum(): number {\n    return 42;\n  }\n}");
}
#[test]
fn test_es_6_default_anon_function_1_js_format_1_dd350a8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_AnonFunction1\n * @flow\n */\n\nexport default function():number { return 42; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_AnonFunction1\n * @flow\n */\n\nexport default function (): number {\n  return 42;\n}");
}
#[test]
fn test_es_6_default_anon_function_2_js_format_1_51798f70() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_AnonFunction2\n * @flow\n */\n\nexport default function():number { return 42; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_AnonFunction2\n * @flow\n */\n\nexport default function (): number {\n  return 42;\n}");
}
#[test]
fn test_es_6_default_named_class_1_js_format_1_769f4bae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_NamedClass1\n * @flow\n */\n\nexport default class Foo { givesANum(): number { return 42; }};\n\n// Regression test for https://github.com/facebook/flow/issues/511\n//\n// Default-exported class should also be available in local scope\nnew Foo();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_NamedClass1\n * @flow\n */\n\nexport default class Foo {\n  givesANum(): number {\n    return 42;\n  }\n}\n\n// Regression test for https://github.com/facebook/flow/issues/511\n//\n// Default-exported class should also be available in local scope\nnew Foo();");
}
#[test]
fn test_es_6_default_named_class_2_js_format_1_daeba755() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_NamedClass2\n * @flow\n */\n\nexport default class Foo { givesANum(): number { return 42; }};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_NamedClass2\n * @flow\n */\n\nexport default class Foo {\n  givesANum(): number {\n    return 42;\n  }\n}");
}
#[test]
fn test_es_6_default_named_function_1_js_format_1_31e75fe2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_NamedFunction1\n * @flow\n */\n\nexport default function foo():number { return 42; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_NamedFunction1\n * @flow\n */\n\nexport default function foo(): number {\n  return 42;\n}");
}
#[test]
fn test_es_6_default_named_function_2_js_format_1_c9ca5331() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Default_NamedFunction2\n * @flow\n */\n\nexport default function foo():number { return 42; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Default_NamedFunction2\n * @flow\n */\n\nexport default function foo(): number {\n  return 42;\n}");
}
#[test]
fn test_es_6_default_and_named_js_format_1_a2e95cce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\n\nexport default 42;\nexport var str = 'asdf';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nexport default 42;\nexport var str = \"asdf\";"
    );
}
#[test]
fn test_es_6_export_all_from_intermediary_1_js_format_1_dba05430() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportAllFrom_Intermediary1\n * @flow\n */\n\nexport * from \"ES6_ExportAllFrom_Source1\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportAllFrom_Intermediary1\n * @flow\n */\n\nexport * from \"ES6_ExportAllFrom_Source1\";");
}
#[test]
fn test_es_6_export_all_from_intermediary_2_js_format_1_add86200() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportAllFrom_Intermediary2\n * @flow\n */\n\nexport * from \"ES6_ExportAllFrom_Source2\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportAllFrom_Intermediary2\n * @flow\n */\n\nexport * from \"ES6_ExportAllFrom_Source2\";");
}
#[test]
fn test_es_6_export_all_from_source_1_js_format_1_062e006d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportAllFrom_Source1\n * @flow\n */\n\nexport var numberValue1 = 1;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportAllFrom_Source1\n * @flow\n */\n\nexport var numberValue1 = 1;");
}
#[test]
fn test_es_6_export_all_from_source_2_js_format_1_84fc2d29() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportAllFrom_Source2\n * @flow\n */\n\nexport var numberValue2 = 1;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportAllFrom_Source2\n * @flow\n */\n\nexport var numberValue2 = 1;");
}
#[test]
fn test_es_6_export_all_from_multi_js_format_1_44755923() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nexport * from \"./ES6_ExportAllFrom_Source1\";\nexport * from \"./ES6_ExportAllFrom_Source2\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nexport * from \"./ES6_ExportAllFrom_Source1\";\nexport * from \"./ES6_ExportAllFrom_Source2\";");
}
#[test]
fn test_es_6_export_from_intermediary_1_js_format_1_b770ec28() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportFrom_Intermediary1\n * @flow\n */\n\nexport {\n  numberValue1,\n  numberValue2 as numberValue2_renamed\n} from \"ES6_ExportFrom_Source1\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportFrom_Intermediary1\n * @flow\n */\n\nexport {\n  numberValue1,\n  numberValue2 as numberValue2_renamed,\n} from \"ES6_ExportFrom_Source1\";");
}
#[test]
fn test_es_6_export_from_intermediary_2_js_format_1_d441645a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportFrom_Intermediary2\n * @flow\n */\n\nexport {\n  numberValue1,\n  numberValue2 as numberValue2_renamed2\n} from \"ES6_ExportFrom_Source2\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportFrom_Intermediary2\n * @flow\n */\n\nexport {\n  numberValue1,\n  numberValue2 as numberValue2_renamed2,\n} from \"ES6_ExportFrom_Source2\";");
}
#[test]
fn test_es_6_export_from_source_1_js_format_1_15fa7569() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportFrom_Source1\n * @flow\n */\n\nexport var numberValue1 = 1, numberValue2 = 2;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportFrom_Source1\n * @flow\n */\n\nexport var numberValue1 = 1,\n  numberValue2 = 2;");
}
#[test]
fn test_es_6_export_from_source_2_js_format_1_7bc767d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_ExportFrom_Source2\n * @flow\n */\n\nexport var numberValue1 = 1, numberValue2 = 2;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_ExportFrom_Source2\n * @flow\n */\n\nexport var numberValue1 = 1,\n  numberValue2 = 2;");
}
#[test]
fn test_es_6_named_1_js_format_1_58c9ed31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Named1\n * @flow\n */\n\nvar specifierNumber1 = 1;\nvar specifierNumber2 = 2;\nvar specifierNumber3 = 3;\nvar groupedSpecifierNumber1 = 1;\nvar groupedSpecifierNumber2 = 2;\n\nexport {specifierNumber1};\nexport {specifierNumber2 as specifierNumber2Renamed};\nexport {specifierNumber3};\nexport {groupedSpecifierNumber1, groupedSpecifierNumber2};\n\nexport function givesANumber(): number { return 42; };\nexport class NumberGenerator { givesANumber(): number { return 42; }};\n\nexport var varDeclNumber1 = 1, varDeclNumber2 = 2;\nexport var {destructuredObjNumber} = {destructuredObjNumber: 1};\nexport var [destructuredArrNumber] = [1]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Named1\n * @flow\n */\n\nvar specifierNumber1 = 1;\nvar specifierNumber2 = 2;\nvar specifierNumber3 = 3;\nvar groupedSpecifierNumber1 = 1;\nvar groupedSpecifierNumber2 = 2;\n\nexport { specifierNumber1 };\nexport { specifierNumber2 as specifierNumber2Renamed };\nexport { specifierNumber3 };\nexport { groupedSpecifierNumber1, groupedSpecifierNumber2 };\n\nexport function givesANumber(): number {\n  return 42;\n}\nexport class NumberGenerator {\n  givesANumber(): number {\n    return 42;\n  }\n}\n\nexport var varDeclNumber1 = 1,\n  varDeclNumber2 = 2;\nexport var { destructuredObjNumber } = { destructuredObjNumber: 1 };\nexport var [destructuredArrNumber] = [1];");
}
#[test]
fn test_es_6_named_2_js_format_1_a096b481() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ES6_Named2\n * @flow\n */\n\nvar specifierNumber4 = 1;\nvar specifierNumber5 = 2;\nvar groupedSpecifierNumber3 = 1;\nvar groupedSpecifierNumber4 = 2;\n\nexport {specifierNumber4};\nexport {specifierNumber5 as specifierNumber5Renamed};\nexport {groupedSpecifierNumber3, groupedSpecifierNumber4};\n\nexport function givesANumber2(): number { return 42; };\nexport class NumberGenerator2 { givesANumber(): number { return 42; }};\n\nexport var varDeclNumber3 = 1, varDeclNumber4 = 2;\nexport var {destructuredObjNumber2} = {destructuredObjNumber2: 1};\nexport var [destructuredArrNumber2] = [1]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ES6_Named2\n * @flow\n */\n\nvar specifierNumber4 = 1;\nvar specifierNumber5 = 2;\nvar groupedSpecifierNumber3 = 1;\nvar groupedSpecifierNumber4 = 2;\n\nexport { specifierNumber4 };\nexport { specifierNumber5 as specifierNumber5Renamed };\nexport { groupedSpecifierNumber3, groupedSpecifierNumber4 };\n\nexport function givesANumber2(): number {\n  return 42;\n}\nexport class NumberGenerator2 {\n  givesANumber(): number {\n    return 42;\n  }\n}\n\nexport var varDeclNumber3 = 1,\n  varDeclNumber4 = 2;\nexport var { destructuredObjNumber2 } = { destructuredObjNumber2: 1 };\nexport var [destructuredArrNumber2] = [1];");
}
#[test]
fn test_export_type_js_format_1_15e061d5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nexport type typeAlias = number;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @flow\n\nexport type typeAlias = number;");
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
fn test_es_6_modules_js_format_1_528f991e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n// ===================== //\n// == Path Resolution == //\n// ===================== //\n\n// @providesModule\nimport * as DefaultA from \"A\";\nvar a1: number = DefaultA.numberValue1;\nvar a2: string = DefaultA.numberValue1; // Error: number ~> string\n\n// File path\nimport * as DefaultB from \"./B\";\nvar b1: number = DefaultB.numberValue;\nvar b2: string = DefaultB.numberValue; // Error: number ~> string\n\n// C.js exists, but not as a providesModule\nimport DefaultC from \"C\"; // Error: No such module\n\n// @providesModule D exists, but not as a filename\nimport DefaultD from \"./D\"; // Error: No such module\n\n// ================================================ //\n// == CommonJS Clobbering Literal Exports -> ES6 == //\n// ================================================ //\n\nimport {doesntExist1} from \"CommonJS_Clobbering_Lit\"; // Error: Not an exported binding\n\nimport {numberValue1} from \"CommonJS_Clobbering_Lit\";\nvar c1: number = numberValue1;\nvar c2: string = numberValue1; // Error: number ~> string\n\nimport {numberValue2 as numVal1} from \"CommonJS_Clobbering_Lit\";\nvar d1: number = numVal1;\nvar d2: string = numVal1; // Error: number ~> string\n\nimport CJS_Clobb_Lit from \"CommonJS_Clobbering_Lit\";\nvar e1: number = CJS_Clobb_Lit.numberValue3;\nvar e2: string = CJS_Clobb_Lit.numberValue3; // Error: number ~> string\nCJS_Clobb_Lit.doesntExist; // Error: doesntExist isn't a property\n\nimport * as CJS_Clobb_Lit_NS from \"CommonJS_Clobbering_Lit\";\nvar f1: number = CJS_Clobb_Lit_NS.numberValue4;\nvar f2: number = CJS_Clobb_Lit_NS.default.numberValue4;\nCJS_Clobb_Lit_NS.default.default; // Error: No 'default' property on the exported obj\nvar f3: string = CJS_Clobb_Lit_NS.numberValue4; // Error: number ~> string\nvar f4: string = CJS_Clobb_Lit_NS.default.numberValue5; // Error: number ~> string\n\n// ============================================== //\n// == CommonJS Clobbering Class Exports -> ES6 == //\n// ============================================== //\n\nimport {doesntExist2} from \"CommonJS_Clobbering_Class\"; // Error: Not an exported binding\n\n// The following import should error because class statics are not turned into\n// named exports for now. This avoids complexities with polymorphic static\n// members (where the polymophism is defined on the class itself rather than the\n// method).\nimport {staticNumber1, baseProp, childProp} from \"CommonJS_Clobbering_Class\"; // Error\n\nimport CJS_Clobb_Class from \"CommonJS_Clobbering_Class\";\nnew CJS_Clobb_Class();\nnew CJS_Clobb_Class().doesntExist; // Error: Class has no \\`doesntExist\\` property\nvar h1: number = CJS_Clobb_Class.staticNumber2();\nvar h2: string = CJS_Clobb_Class.staticNumber2(); // Error: number ~> string\nvar h3: number = new CJS_Clobb_Class().instNumber1();\nvar h4: string = new CJS_Clobb_Class().instNumber1(); // Error: number ~> string\n\nimport * as CJS_Clobb_Class_NS from \"CommonJS_Clobbering_Class\";\nnew CJS_Clobb_Class_NS(); // Error: Namespace object isn't constructable\nvar i1: number = CJS_Clobb_Class_NS.staticNumber3(); // Error: Class statics not copied to Namespace object\nvar i2: number = new CJS_Clobb_Class_NS.default().instNumber2();\nvar i3: string = new CJS_Clobb_Class_NS.default().instNumber2(); // Error: number ~> string\n\n// =================================== //\n// == CommonJS Named Exports -> ES6 == //\n// =================================== //\n\nimport {doesntExist3} from \"CommonJS_Named\"; // Error: Not an exported binding\n\nimport {numberValue2} from \"CommonJS_Named\";\nvar j1: number = numberValue2;\nvar j2: string = numberValue2; // Error: number ~> string\n\nimport {numberValue3 as numVal3} from \"CommonJS_Named\";\nvar k1: number = numVal3;\nvar k2: string = numVal3; // Error: number ~> string\n\nimport * as CJS_Named from \"CommonJS_Named\";\nvar l1: number = CJS_Named.numberValue1;\nvar l2: string = CJS_Named.numberValue1; // Error: number ~> string\nCJS_Named.doesntExist; // Error: doesntExist isn't a property\n\nimport * as CJS_Named_NS from \"CommonJS_Named\";\nvar m1: number = CJS_Named_NS.numberValue4;\nvar m2: string = CJS_Named_NS.default.numberValue4; // Error: CommonJS_Named has no default export\nvar m3: string = CJS_Named_NS.numberValue4; // Error: number ~> string\n\n//////////////////////////////\n// == ES6 Default -> ES6 == //\n//////////////////////////////\n\nimport {doesntExist4} from \"ES6_Default_AnonFunction1\"; // Error: Not an exported binding\n\nimport ES6_Def_AnonFunc1 from \"ES6_Default_AnonFunction1\";\nvar n1: number = ES6_Def_AnonFunc1();\nvar n2: string = ES6_Def_AnonFunc1(); // Error: number ~> string\n\nimport ES6_Def_NamedFunc1 from \"ES6_Default_NamedFunction1\";\nvar o1: number = ES6_Def_NamedFunc1();\nvar o2: string = ES6_Def_NamedFunc1(); // Error: number ~> string\n\nimport ES6_Def_AnonClass1 from \"ES6_Default_AnonClass1\";\nvar p1: number = new ES6_Def_AnonClass1().givesANum();\nvar p2: string = new ES6_Def_AnonClass1().givesANum(); // Error: number ~> string\n\nimport ES6_Def_NamedClass1 from \"ES6_Default_NamedClass1\";\nvar q1: number = new ES6_Def_NamedClass1().givesANum();\nvar q2: string = new ES6_Def_NamedClass1().givesANum(); // Error: number ~> string\n\n////////////////////////////\n// == ES6 Named -> ES6 == //\n////////////////////////////\n\nimport doesntExist5 from \"ES6_Named1\"; // Error: Not an exported binding\n\nimport {specifierNumber1 as specifierNumber1_1} from \"ES6_Named1\";\nvar r1: number = specifierNumber1_1;\nvar r2: string = specifierNumber1_1; // Error: number ~> string\n\nimport {specifierNumber2Renamed} from \"ES6_Named1\";\nvar s1: number = specifierNumber2Renamed;\nvar s2: string = specifierNumber2Renamed; // Error: number ~> string\n\nimport {specifierNumber3 as specifierNumber3Renamed} from \"ES6_Named1\";\nvar t1: number = specifierNumber3Renamed;\nvar t2: string = specifierNumber3Renamed; // Error: number ~> string\n\nimport {groupedSpecifierNumber1, groupedSpecifierNumber2} from \"ES6_Named1\";\nvar u1: number = groupedSpecifierNumber1;\nvar u2: number = groupedSpecifierNumber2;\nvar u3: string = groupedSpecifierNumber1; // Error: number ~> string\nvar u4: string = groupedSpecifierNumber2; // Error: number ~> string\n\nimport {givesANumber} from \"ES6_Named1\";\nvar v1: number = givesANumber();\nvar v2: string = givesANumber(); // Error: number ~> string\n\nimport {NumberGenerator} from \"ES6_Named1\";\nvar w1: number = new NumberGenerator().givesANumber();\nvar w2: string = new NumberGenerator().givesANumber(); // Error: number ~> string\n\nimport {varDeclNumber1, varDeclNumber2} from \"ES6_Named1\";\nvar x1: number = varDeclNumber1;\nvar x2: number = varDeclNumber2;\nvar x3: string = varDeclNumber1; // Error: number ~> string\nvar x4: string = varDeclNumber2; // Error: number ~> string\n\nimport {destructuredObjNumber} from \"ES6_Named1\";\nvar y1: number = destructuredObjNumber;\nvar y2: string = destructuredObjNumber; // Error: number ~> string\n\nimport {destructuredArrNumber} from \"ES6_Named1\";\nvar z1: number = destructuredArrNumber;\nvar z2: string = destructuredArrNumber; // Error: number ~> string\n\nimport {numberValue1 as numberValue4} from \"ES6_ExportFrom_Intermediary1\";\nvar aa1: number = numberValue4;\nvar aa2: string = numberValue4; // Error: number ~> string\n\nimport {numberValue2_renamed} from \"ES6_ExportFrom_Intermediary1\";\nvar ab1: number = numberValue2_renamed;\nvar ab2: string = numberValue2_renamed; // Error: number ~> string\n\nimport {numberValue1 as numberValue5} from \"ES6_ExportAllFrom_Intermediary1\";\nvar ac1: number = numberValue5;\nvar ac2: string = numberValue5; // Error: number ~> string\n\n///////////////////////////////////\n// == ES6 Default -> CommonJS == //\n///////////////////////////////////\n\nrequire('ES6_Default_AnonFunction2').doesntExist; // Error: 'doesntExist' isn't an export\n\nvar ES6_Def_AnonFunc2 = require(\"ES6_Default_AnonFunction2\").default;\nvar ad1: number = ES6_Def_AnonFunc2();\nvar ad2: string = ES6_Def_AnonFunc2(); // Error: number ~> string\n\nvar ES6_Def_NamedFunc2 = require(\"ES6_Default_NamedFunction2\").default;\nvar ae1: number = ES6_Def_NamedFunc2();\nvar ae2: string = ES6_Def_NamedFunc2(); // Error: number ~> string\n\nvar ES6_Def_AnonClass2 = require(\"ES6_Default_AnonClass2\").default;\nvar af1: number = new ES6_Def_AnonClass2().givesANum();\nvar af2: string = new ES6_Def_AnonClass2().givesANum(); // Error: number ~> string\n\nvar ES6_Def_NamedClass2 = require(\"ES6_Default_NamedClass2\").default;\nvar ag1: number = new ES6_Def_NamedClass2().givesANum();\nvar ag2: string = new ES6_Def_NamedClass2().givesANum(); // Error: number ~> string\n\n/////////////////////////////////\n// == ES6 Named -> CommonJS == //\n/////////////////////////////////\n\nvar specifierNumber4 = require(\"ES6_Named2\").specifierNumber4;\nvar ah1: number = specifierNumber4;\nvar ah2: string = specifierNumber4; // Error: number ~> string\n\nvar specifierNumber5Renamed = require(\"ES6_Named2\").specifierNumber5Renamed;\nvar ai1: number = specifierNumber5Renamed;\nvar ai2: string = specifierNumber5Renamed; // Error: number ~> string\n\nvar groupedSpecifierNumber3 = require(\"ES6_Named2\").groupedSpecifierNumber3;\nvar groupedSpecifierNumber4 = require(\"ES6_Named2\").groupedSpecifierNumber4;\nvar aj1: number = groupedSpecifierNumber3;\nvar aj2: number = groupedSpecifierNumber4;\nvar aj3: string = groupedSpecifierNumber3; // Error: number ~> string\nvar aj4: string = groupedSpecifierNumber4; // Error: number ~> string\n\nvar givesANumber2 = require(\"ES6_Named2\").givesANumber2;\nvar ak1: number = givesANumber2();\nvar ak2: string = givesANumber2(); // Error: number ~> string\n\nvar NumberGenerator2 = require(\"ES6_Named2\").NumberGenerator2;\nvar al1: number = new NumberGenerator2().givesANumber();\nvar al2: string = new NumberGenerator2().givesANumber(); // Error: number ~> string\n\nvar varDeclNumber3 = require(\"ES6_Named2\").varDeclNumber3;\nvar varDeclNumber4 = require(\"ES6_Named2\").varDeclNumber4;\nvar am1: number = varDeclNumber3;\nvar am2: number = varDeclNumber4;\nvar am3: string = varDeclNumber3; // Error: number ~> string\nvar am4: string = varDeclNumber4; // Error: number ~> string\n\nvar destructuredObjNumber2 = require(\"ES6_Named2\").destructuredObjNumber2;\nvar an1: number = destructuredObjNumber2;\nvar an2: string = destructuredObjNumber2; // Error: number ~> string\n\nvar destructuredArrNumber2 = require(\"ES6_Named2\").destructuredArrNumber2;\nvar ao1: number = destructuredArrNumber2;\nvar ao2: string = destructuredArrNumber2; // Error: number ~> string\n\nvar numberValue6 = require(\"ES6_ExportFrom_Intermediary2\").numberValue1;\nvar ap1: number = numberValue6;\nvar ap2: string = numberValue6; // Error: number ~> string\n\nvar numberValue2_renamed2 = require(\"ES6_ExportFrom_Intermediary2\").numberValue2_renamed2;\nvar aq1: number = numberValue2_renamed2;\nvar aq2: string = numberValue2_renamed2; // Error: number ~> string\n\nvar numberValue7 = require(\"ES6_ExportAllFrom_Intermediary2\").numberValue2;\nvar ar1: number = numberValue7;\nvar ar2: string = numberValue7; // Error: number ~> string\n\n////////////////////////////////////////////////////////\n// == ES6 Default+Named -> ES6 import Default+Named== //\n////////////////////////////////////////////////////////\n\nimport defaultNum, {str as namedStr} from \"./ES6_DefaultAndNamed\";\n\nvar as1: number = defaultNum;\nvar as2: string = defaultNum; // Error: number ~> string\n\nvar as3: string = namedStr;\nvar as4: number = namedStr; // Error: string ~> number\n\n////////////////////////////////////////\n// == Side-effect only ES6 imports == //\n////////////////////////////////////////\n\nimport \"./SideEffects\";\n\n//////////////////////////////////////////////\n// == Suggest export name on likely typo == //\n//////////////////////////////////////////////\nimport specifierNumber1 from \"ES6_Named1\"; // Error: Did you mean \\`import {specifierNumber1} from ...\\`?\nimport {specifierNumber} from \"ES6_Named1\"; // Error: Did you mean \\`specifierNumber1\\`?\n\n///////////////////////////////////////////////////\n// == Multi \\`export *\\` should combine exports == //\n///////////////////////////////////////////////////\nimport {\n  numberValue1 as numberValue8,\n  numberValue2 as numberValue9\n} from \"./ES6_ExportAllFromMulti\";\n\nvar at1: number = numberValue8;\nvar at2: string = numberValue8; // Error: number ~> string\n\nvar at3: number = numberValue9;\nvar at4: string = numberValue9; // Error: number ~> string\n\n/////////////////////////////////////////////////////////////\n// == Vanilla \\`import\\` cannot import a type-only export == //\n/////////////////////////////////////////////////////////////\nimport {typeAlias} from \"./ExportType\"; // Error: Cannot vanilla-import a type alias!") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n// ===================== //\n// == Path Resolution == //\n// ===================== //\n\n// @providesModule\nimport * as DefaultA from \"A\";\nvar a1: number = DefaultA.numberValue1;\nvar a2: string = DefaultA.numberValue1; // Error: number ~> string\n\n// File path\nimport * as DefaultB from \"./B\";\nvar b1: number = DefaultB.numberValue;\nvar b2: string = DefaultB.numberValue; // Error: number ~> string\n\n// C.js exists, but not as a providesModule\nimport DefaultC from \"C\"; // Error: No such module\n\n// @providesModule D exists, but not as a filename\nimport DefaultD from \"./D\"; // Error: No such module\n\n// ================================================ //\n// == CommonJS Clobbering Literal Exports -> ES6 == //\n// ================================================ //\n\nimport { doesntExist1 } from \"CommonJS_Clobbering_Lit\"; // Error: Not an exported binding\n\nimport { numberValue1 } from \"CommonJS_Clobbering_Lit\";\nvar c1: number = numberValue1;\nvar c2: string = numberValue1; // Error: number ~> string\n\nimport { numberValue2 as numVal1 } from \"CommonJS_Clobbering_Lit\";\nvar d1: number = numVal1;\nvar d2: string = numVal1; // Error: number ~> string\n\nimport CJS_Clobb_Lit from \"CommonJS_Clobbering_Lit\";\nvar e1: number = CJS_Clobb_Lit.numberValue3;\nvar e2: string = CJS_Clobb_Lit.numberValue3; // Error: number ~> string\nCJS_Clobb_Lit.doesntExist; // Error: doesntExist isn't a property\n\nimport * as CJS_Clobb_Lit_NS from \"CommonJS_Clobbering_Lit\";\nvar f1: number = CJS_Clobb_Lit_NS.numberValue4;\nvar f2: number = CJS_Clobb_Lit_NS.default.numberValue4;\nCJS_Clobb_Lit_NS.default.default; // Error: No 'default' property on the exported obj\nvar f3: string = CJS_Clobb_Lit_NS.numberValue4; // Error: number ~> string\nvar f4: string = CJS_Clobb_Lit_NS.default.numberValue5; // Error: number ~> string\n\n// ============================================== //\n// == CommonJS Clobbering Class Exports -> ES6 == //\n// ============================================== //\n\nimport { doesntExist2 } from \"CommonJS_Clobbering_Class\"; // Error: Not an exported binding\n\n// The following import should error because class statics are not turned into\n// named exports for now. This avoids complexities with polymorphic static\n// members (where the polymophism is defined on the class itself rather than the\n// method).\nimport { staticNumber1, baseProp, childProp } from \"CommonJS_Clobbering_Class\"; // Error\n\nimport CJS_Clobb_Class from \"CommonJS_Clobbering_Class\";\nnew CJS_Clobb_Class();\nnew CJS_Clobb_Class().doesntExist; // Error: Class has no \\`doesntExist\\` property\nvar h1: number = CJS_Clobb_Class.staticNumber2();\nvar h2: string = CJS_Clobb_Class.staticNumber2(); // Error: number ~> string\nvar h3: number = new CJS_Clobb_Class().instNumber1();\nvar h4: string = new CJS_Clobb_Class().instNumber1(); // Error: number ~> string\n\nimport * as CJS_Clobb_Class_NS from \"CommonJS_Clobbering_Class\";\nnew CJS_Clobb_Class_NS(); // Error: Namespace object isn't constructable\nvar i1: number = CJS_Clobb_Class_NS.staticNumber3(); // Error: Class statics not copied to Namespace object\nvar i2: number = new CJS_Clobb_Class_NS.default().instNumber2();\nvar i3: string = new CJS_Clobb_Class_NS.default().instNumber2(); // Error: number ~> string\n\n// =================================== //\n// == CommonJS Named Exports -> ES6 == //\n// =================================== //\n\nimport { doesntExist3 } from \"CommonJS_Named\"; // Error: Not an exported binding\n\nimport { numberValue2 } from \"CommonJS_Named\";\nvar j1: number = numberValue2;\nvar j2: string = numberValue2; // Error: number ~> string\n\nimport { numberValue3 as numVal3 } from \"CommonJS_Named\";\nvar k1: number = numVal3;\nvar k2: string = numVal3; // Error: number ~> string\n\nimport * as CJS_Named from \"CommonJS_Named\";\nvar l1: number = CJS_Named.numberValue1;\nvar l2: string = CJS_Named.numberValue1; // Error: number ~> string\nCJS_Named.doesntExist; // Error: doesntExist isn't a property\n\nimport * as CJS_Named_NS from \"CommonJS_Named\";\nvar m1: number = CJS_Named_NS.numberValue4;\nvar m2: string = CJS_Named_NS.default.numberValue4; // Error: CommonJS_Named has no default export\nvar m3: string = CJS_Named_NS.numberValue4; // Error: number ~> string\n\n//////////////////////////////\n// == ES6 Default -> ES6 == //\n//////////////////////////////\n\nimport { doesntExist4 } from \"ES6_Default_AnonFunction1\"; // Error: Not an exported binding\n\nimport ES6_Def_AnonFunc1 from \"ES6_Default_AnonFunction1\";\nvar n1: number = ES6_Def_AnonFunc1();\nvar n2: string = ES6_Def_AnonFunc1(); // Error: number ~> string\n\nimport ES6_Def_NamedFunc1 from \"ES6_Default_NamedFunction1\";\nvar o1: number = ES6_Def_NamedFunc1();\nvar o2: string = ES6_Def_NamedFunc1(); // Error: number ~> string\n\nimport ES6_Def_AnonClass1 from \"ES6_Default_AnonClass1\";\nvar p1: number = new ES6_Def_AnonClass1().givesANum();\nvar p2: string = new ES6_Def_AnonClass1().givesANum(); // Error: number ~> string\n\nimport ES6_Def_NamedClass1 from \"ES6_Default_NamedClass1\";\nvar q1: number = new ES6_Def_NamedClass1().givesANum();\nvar q2: string = new ES6_Def_NamedClass1().givesANum(); // Error: number ~> string\n\n////////////////////////////\n// == ES6 Named -> ES6 == //\n////////////////////////////\n\nimport doesntExist5 from \"ES6_Named1\"; // Error: Not an exported binding\n\nimport { specifierNumber1 as specifierNumber1_1 } from \"ES6_Named1\";\nvar r1: number = specifierNumber1_1;\nvar r2: string = specifierNumber1_1; // Error: number ~> string\n\nimport { specifierNumber2Renamed } from \"ES6_Named1\";\nvar s1: number = specifierNumber2Renamed;\nvar s2: string = specifierNumber2Renamed; // Error: number ~> string\n\nimport { specifierNumber3 as specifierNumber3Renamed } from \"ES6_Named1\";\nvar t1: number = specifierNumber3Renamed;\nvar t2: string = specifierNumber3Renamed; // Error: number ~> string\n\nimport { groupedSpecifierNumber1, groupedSpecifierNumber2 } from \"ES6_Named1\";\nvar u1: number = groupedSpecifierNumber1;\nvar u2: number = groupedSpecifierNumber2;\nvar u3: string = groupedSpecifierNumber1; // Error: number ~> string\nvar u4: string = groupedSpecifierNumber2; // Error: number ~> string\n\nimport { givesANumber } from \"ES6_Named1\";\nvar v1: number = givesANumber();\nvar v2: string = givesANumber(); // Error: number ~> string\n\nimport { NumberGenerator } from \"ES6_Named1\";\nvar w1: number = new NumberGenerator().givesANumber();\nvar w2: string = new NumberGenerator().givesANumber(); // Error: number ~> string\n\nimport { varDeclNumber1, varDeclNumber2 } from \"ES6_Named1\";\nvar x1: number = varDeclNumber1;\nvar x2: number = varDeclNumber2;\nvar x3: string = varDeclNumber1; // Error: number ~> string\nvar x4: string = varDeclNumber2; // Error: number ~> string\n\nimport { destructuredObjNumber } from \"ES6_Named1\";\nvar y1: number = destructuredObjNumber;\nvar y2: string = destructuredObjNumber; // Error: number ~> string\n\nimport { destructuredArrNumber } from \"ES6_Named1\";\nvar z1: number = destructuredArrNumber;\nvar z2: string = destructuredArrNumber; // Error: number ~> string\n\nimport { numberValue1 as numberValue4 } from \"ES6_ExportFrom_Intermediary1\";\nvar aa1: number = numberValue4;\nvar aa2: string = numberValue4; // Error: number ~> string\n\nimport { numberValue2_renamed } from \"ES6_ExportFrom_Intermediary1\";\nvar ab1: number = numberValue2_renamed;\nvar ab2: string = numberValue2_renamed; // Error: number ~> string\n\nimport { numberValue1 as numberValue5 } from \"ES6_ExportAllFrom_Intermediary1\";\nvar ac1: number = numberValue5;\nvar ac2: string = numberValue5; // Error: number ~> string\n\n///////////////////////////////////\n// == ES6 Default -> CommonJS == //\n///////////////////////////////////\n\nrequire(\"ES6_Default_AnonFunction2\").doesntExist; // Error: 'doesntExist' isn't an export\n\nvar ES6_Def_AnonFunc2 = require(\"ES6_Default_AnonFunction2\").default;\nvar ad1: number = ES6_Def_AnonFunc2();\nvar ad2: string = ES6_Def_AnonFunc2(); // Error: number ~> string\n\nvar ES6_Def_NamedFunc2 = require(\"ES6_Default_NamedFunction2\").default;\nvar ae1: number = ES6_Def_NamedFunc2();\nvar ae2: string = ES6_Def_NamedFunc2(); // Error: number ~> string\n\nvar ES6_Def_AnonClass2 = require(\"ES6_Default_AnonClass2\").default;\nvar af1: number = new ES6_Def_AnonClass2().givesANum();\nvar af2: string = new ES6_Def_AnonClass2().givesANum(); // Error: number ~> string\n\nvar ES6_Def_NamedClass2 = require(\"ES6_Default_NamedClass2\").default;\nvar ag1: number = new ES6_Def_NamedClass2().givesANum();\nvar ag2: string = new ES6_Def_NamedClass2().givesANum(); // Error: number ~> string\n\n/////////////////////////////////\n// == ES6 Named -> CommonJS == //\n/////////////////////////////////\n\nvar specifierNumber4 = require(\"ES6_Named2\").specifierNumber4;\nvar ah1: number = specifierNumber4;\nvar ah2: string = specifierNumber4; // Error: number ~> string\n\nvar specifierNumber5Renamed = require(\"ES6_Named2\").specifierNumber5Renamed;\nvar ai1: number = specifierNumber5Renamed;\nvar ai2: string = specifierNumber5Renamed; // Error: number ~> string\n\nvar groupedSpecifierNumber3 = require(\"ES6_Named2\").groupedSpecifierNumber3;\nvar groupedSpecifierNumber4 = require(\"ES6_Named2\").groupedSpecifierNumber4;\nvar aj1: number = groupedSpecifierNumber3;\nvar aj2: number = groupedSpecifierNumber4;\nvar aj3: string = groupedSpecifierNumber3; // Error: number ~> string\nvar aj4: string = groupedSpecifierNumber4; // Error: number ~> string\n\nvar givesANumber2 = require(\"ES6_Named2\").givesANumber2;\nvar ak1: number = givesANumber2();\nvar ak2: string = givesANumber2(); // Error: number ~> string\n\nvar NumberGenerator2 = require(\"ES6_Named2\").NumberGenerator2;\nvar al1: number = new NumberGenerator2().givesANumber();\nvar al2: string = new NumberGenerator2().givesANumber(); // Error: number ~> string\n\nvar varDeclNumber3 = require(\"ES6_Named2\").varDeclNumber3;\nvar varDeclNumber4 = require(\"ES6_Named2\").varDeclNumber4;\nvar am1: number = varDeclNumber3;\nvar am2: number = varDeclNumber4;\nvar am3: string = varDeclNumber3; // Error: number ~> string\nvar am4: string = varDeclNumber4; // Error: number ~> string\n\nvar destructuredObjNumber2 = require(\"ES6_Named2\").destructuredObjNumber2;\nvar an1: number = destructuredObjNumber2;\nvar an2: string = destructuredObjNumber2; // Error: number ~> string\n\nvar destructuredArrNumber2 = require(\"ES6_Named2\").destructuredArrNumber2;\nvar ao1: number = destructuredArrNumber2;\nvar ao2: string = destructuredArrNumber2; // Error: number ~> string\n\nvar numberValue6 = require(\"ES6_ExportFrom_Intermediary2\").numberValue1;\nvar ap1: number = numberValue6;\nvar ap2: string = numberValue6; // Error: number ~> string\n\nvar numberValue2_renamed2 =\n  require(\"ES6_ExportFrom_Intermediary2\").numberValue2_renamed2;\nvar aq1: number = numberValue2_renamed2;\nvar aq2: string = numberValue2_renamed2; // Error: number ~> string\n\nvar numberValue7 = require(\"ES6_ExportAllFrom_Intermediary2\").numberValue2;\nvar ar1: number = numberValue7;\nvar ar2: string = numberValue7; // Error: number ~> string\n\n////////////////////////////////////////////////////////\n// == ES6 Default+Named -> ES6 import Default+Named== //\n////////////////////////////////////////////////////////\n\nimport defaultNum, { str as namedStr } from \"./ES6_DefaultAndNamed\";\n\nvar as1: number = defaultNum;\nvar as2: string = defaultNum; // Error: number ~> string\n\nvar as3: string = namedStr;\nvar as4: number = namedStr; // Error: string ~> number\n\n////////////////////////////////////////\n// == Side-effect only ES6 imports == //\n////////////////////////////////////////\n\nimport \"./SideEffects\";\n\n//////////////////////////////////////////////\n// == Suggest export name on likely typo == //\n//////////////////////////////////////////////\nimport specifierNumber1 from \"ES6_Named1\"; // Error: Did you mean \\`import {specifierNumber1} from ...\\`?\nimport { specifierNumber } from \"ES6_Named1\"; // Error: Did you mean \\`specifierNumber1\\`?\n\n///////////////////////////////////////////////////\n// == Multi \\`export *\\` should combine exports == //\n///////////////////////////////////////////////////\nimport {\n  numberValue1 as numberValue8,\n  numberValue2 as numberValue9,\n} from \"./ES6_ExportAllFromMulti\";\n\nvar at1: number = numberValue8;\nvar at2: string = numberValue8; // Error: number ~> string\n\nvar at3: number = numberValue9;\nvar at4: string = numberValue9; // Error: number ~> string\n\n/////////////////////////////////////////////////////////////\n// == Vanilla \\`import\\` cannot import a type-only export == //\n/////////////////////////////////////////////////////////////\nimport { typeAlias } from \"./ExportType\"; // Error: Cannot vanilla-import a type alias!");
}
#[test]
fn test_test_imports_are_frozen_js_format_1_16a48a45() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n//\n// Imports\n//\n\n// CommonJS module\nimport * as DefaultA from \"A\";\nDefaultA.numberValue1 = 123; // Error: DefaultA is frozen\n\n// ES6 module\nimport * as ES6_Named1 from \"ES6_Named1\";\nES6_Named1.varDeclNumber1 = 123; // Error: ES6_Named1 is frozen\n\n// CommonJS module that clobbers module.exports\nimport * as CommonJS_Star from \"CommonJS_Clobbering_Lit\";\nCommonJS_Star.numberValue1 = 123; // Error: frozen\nCommonJS_Star.default.numberValue1 = 123; // ok\n\nimport CommonJS_Clobbering_Lit from \"CommonJS_Clobbering_Lit\";\nCommonJS_Clobbering_Lit.numberValue1 = 123; // ok\n\n// CommonJS module that clobbers module.exports with a frozen object\nimport * as CommonJS_Frozen_Star from \"CommonJS_Clobbering_Frozen\";\nCommonJS_Frozen_Star.numberValue1 = 123; // Error: frozen\nCommonJS_Frozen_Star.default.numberValue1 = 123; // Error: frozen\n\nimport CommonJS_Clobbering_Frozen from \"CommonJS_Clobbering_Frozen\";\nCommonJS_Clobbering_Frozen.numberValue1 = 123; // Error: exports are frozen\n\n\n//\n// Requires\n//\n\nfunction testRequires() {\n  // CommonJS module\n  var DefaultA = require(\"A\");\n  DefaultA.numberValue1 = 123; // ok, not frozen by default\n\n  // ES6 module\n  var ES6_Named1 = require(\"ES6_Named1\");\n  ES6_Named1.numberValue = 123; // error, es6 exports are frozen\n\n  // CommonJS module that clobbers module.exports\n  var CommonJS_Star = require(\"CommonJS_Clobbering_Lit\");\n  CommonJS_Star.numberValue1 = 123; // ok, not frozen by default\n\n  // CommonJS module that clobbers module.exports with a frozen object\n  var CommonJS_Frozen_Star = require(\"CommonJS_Clobbering_Frozen\");\n  CommonJS_Frozen_Star.numberValue1 = 123; // Error: frozen\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n//\n// Imports\n//\n\n// CommonJS module\nimport * as DefaultA from \"A\";\nDefaultA.numberValue1 = 123; // Error: DefaultA is frozen\n\n// ES6 module\nimport * as ES6_Named1 from \"ES6_Named1\";\nES6_Named1.varDeclNumber1 = 123; // Error: ES6_Named1 is frozen\n\n// CommonJS module that clobbers module.exports\nimport * as CommonJS_Star from \"CommonJS_Clobbering_Lit\";\nCommonJS_Star.numberValue1 = 123; // Error: frozen\nCommonJS_Star.default.numberValue1 = 123; // ok\n\nimport CommonJS_Clobbering_Lit from \"CommonJS_Clobbering_Lit\";\nCommonJS_Clobbering_Lit.numberValue1 = 123; // ok\n\n// CommonJS module that clobbers module.exports with a frozen object\nimport * as CommonJS_Frozen_Star from \"CommonJS_Clobbering_Frozen\";\nCommonJS_Frozen_Star.numberValue1 = 123; // Error: frozen\nCommonJS_Frozen_Star.default.numberValue1 = 123; // Error: frozen\n\nimport CommonJS_Clobbering_Frozen from \"CommonJS_Clobbering_Frozen\";\nCommonJS_Clobbering_Frozen.numberValue1 = 123; // Error: exports are frozen\n\n//\n// Requires\n//\n\nfunction testRequires() {\n  // CommonJS module\n  var DefaultA = require(\"A\");\n  DefaultA.numberValue1 = 123; // ok, not frozen by default\n\n  // ES6 module\n  var ES6_Named1 = require(\"ES6_Named1\");\n  ES6_Named1.numberValue = 123; // error, es6 exports are frozen\n\n  // CommonJS module that clobbers module.exports\n  var CommonJS_Star = require(\"CommonJS_Clobbering_Lit\");\n  CommonJS_Star.numberValue1 = 123; // ok, not frozen by default\n\n  // CommonJS module that clobbers module.exports with a frozen object\n  var CommonJS_Frozen_Star = require(\"CommonJS_Clobbering_Frozen\");\n  CommonJS_Frozen_Star.numberValue1 = 123; // Error: frozen\n}");
}
