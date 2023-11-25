#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_export_cjs_default_class_js_format_1_14d65da4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass ClassFoo3 {\n  givesANum(): number { return 42; }\n  static givesAFoo3(): ClassFoo3 {\n    return new ClassFoo3();\n  }\n}\n\nmodule.exports = ClassFoo3;") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass ClassFoo3 {\n  givesANum(): number {\n    return 42;\n  }\n  static givesAFoo3(): ClassFoo3 {\n    return new ClassFoo3();\n  }\n}\n\nmodule.exports = ClassFoo3;");
    Ok(())
}
#[test]
fn test_export_cjs_default_number_js_format_1_7bbea462() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nmodule.exports = 42;")?;
    assert_eq!(formatted, "/* @flow */\n\nmodule.exports = 42;");
    Ok(())
}
#[test]
fn test_export_cjs_named_class_js_format_1_aa1acc1d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @flow\n */\n\nclass ClassFoo4 {}\n\nexports.ClassFoo4 = ClassFoo4;")?;
    assert_eq!(
        formatted,
        "/**\n * @flow\n */\n\nclass ClassFoo4 {}\n\nexports.ClassFoo4 = ClassFoo4;"
    );
    Ok(())
}
#[test]
fn test_export_cjs_named_number_js_format_1_0f8f6f24() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nexports.num = 42;")?;
    assert_eq!(formatted, "/* @flow */\n\nexports.num = 42;");
    Ok(())
}
#[test]
fn test_export_default_class_js_format_1_f0a98631() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass ClassFoo1 {\n  returnsANumber(): number { return 42; }\n}\n\nexport default ClassFoo1;") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass ClassFoo1 {\n  returnsANumber(): number {\n    return 42;\n  }\n}\n\nexport default ClassFoo1;");
    Ok(())
}
#[test]
fn test_export_default_number_js_format_1_46e2a4c7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nexport default 42;")?;
    assert_eq!(formatted, "/* @flow */\n\nexport default 42;");
    Ok(())
}
#[test]
fn test_export_named_alias_js_format_1_f9576ed3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nexport type AliasFoo3  = {\n  givesANum(): number\n};\nexport function givesAFoo3Obj(): AliasFoo3 {\n  return {\n    givesANum(): number { return 42; }\n  };\n};") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nexport type AliasFoo3 = {\n  givesANum(): number,\n};\nexport function givesAFoo3Obj(): AliasFoo3 {\n  return {\n    givesANum(): number {\n      return 42;\n    },\n  };\n}");
    Ok(())
}
#[test]
fn test_export_named_class_js_format_1_8692ba97() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass ClassFoo2 {\n  returnsANumber(): number { return 42; }\n}\n\nexport {ClassFoo2};") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass ClassFoo2 {\n  returnsANumber(): number {\n    return 42;\n  }\n}\n\nexport { ClassFoo2 };");
    Ok(())
}
#[test]
fn test_export_named_multi_js_format_1_d2b1e689() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nexport var num = 42;\nexport var str = 'asdf';")?;
    assert_eq!(
        formatted,
        "// @flow\n\nexport var num = 42;\nexport var str = \"asdf\";"
    );
    Ok(())
}
#[test]
fn test_export_named_number_js_format_1_134e26d3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nexport var num = 42;")?;
    assert_eq!(formatted, "/* @flow */\n\nexport var num = 42;");
    Ok(())
}
#[test]
fn test_import_typeof_js_format_1_06a0be55() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\n///////////////////////////////////////////////////\n// == Importing Class Typeof (Default Export) == //\n///////////////////////////////////////////////////\n\nimport typeof ClassFoo1T from \"./ExportDefault_Class\";\nimport ClassFoo1 from \"./ExportDefault_Class\";\n\nvar a1: ClassFoo1T = ClassFoo1;\nvar a2: ClassFoo1T = new ClassFoo1(); // Error: ClassFoo1 (inst) ~> ClassFoo1 (class)\nnew ClassFoo1T(); // Error: ClassFoo1T is not bound to a value\n\n/////////////////////////////////////////////////\n// == Importing Class Typeof (Named Export) == //\n/////////////////////////////////////////////////\n\nimport typeof {ClassFoo2 as ClassFoo2T} from \"./ExportNamed_Class\";\nimport {ClassFoo2} from \"./ExportNamed_Class\";\n\nvar b1: ClassFoo2T = ClassFoo2;\nvar b2: ClassFoo2T = new ClassFoo2(); // Error: ClassFoo2 (inst) ~> ClassFoo2 (class)\nnew ClassFoo2T(); // Error: ClassFoo2T is not bound to a value\n\n///////////////////////////////////////////////////////\n// == Importing Class Typeof (CJS Default Export) == //\n///////////////////////////////////////////////////////\n\nimport typeof ClassFoo3T from \"./ExportCJSDefault_Class\";\nimport ClassFoo3 from \"./ExportCJSDefault_Class\";\n\nvar c1: ClassFoo3T = ClassFoo3;\nvar c2: ClassFoo3T = new ClassFoo3(); // Error: ClassFoo3 (inst) ~> ClassFoo3 (class)\n\n/////////////////////////////////////////////////////\n// == Importing Class Typeof (CJS Named Export) == //\n/////////////////////////////////////////////////////\n\nimport typeof {ClassFoo4 as ClassFoo4T} from \"./ExportCJSNamed_Class\";\nimport {ClassFoo4} from \"./ExportCJSNamed_Class\";\n\nvar d1: ClassFoo4T = ClassFoo4;\nvar d2: ClassFoo4T = new ClassFoo4(); // Error: ClassFoo4 (inst) ~> ClassFoo4 (class)\n\n//////////////////////////////////////////////\n// == Import Typeof Alias (Named Export) == //\n//////////////////////////////////////////////\n\nimport typeof {AliasFoo3} from \"./ExportNamed_Alias\"; // Error: Can't `import typeof` type aliases!\n\n////////////////////////////////////////////////\n// == Import Typeof Alias (Default Export) == //\n////////////////////////////////////////////////\n\n// TODO: No support for this right now. It's most likely possible, but it's\n//       unclear how useful it is at the moment and it entails a little\n//       more work than named type exports, so I'm punting on it for now.\n\n///////////////////////////////////////////////////////////////\n// == Import Typeof With Non-Class Value (Default Export) == //\n///////////////////////////////////////////////////////////////\n\nimport typeof num_default from \"./ExportDefault_Number\";\n\nvar f1: num_default = 42;\nvar f2: num_default = 'asdf'; // Error: string ~> number\n\n/////////////////////////////////////////////////////////////\n// == Import Typeof With Non-Class Value (Named Export) == //\n/////////////////////////////////////////////////////////////\n\nimport typeof {num as num_named} from \"./ExportNamed_Number\";\n\nvar g1: num_named = 42;\nvar g2: num_named = 'asdf'; // Error: string ~> number\n\n///////////////////////////////////////////////////////////////////\n// == Import Typeof With Non-Class Value (CJS Default Export) == //\n///////////////////////////////////////////////////////////////////\n\nimport typeof num_cjs_default from \"./ExportCJSDefault_Number\";\n\nvar h1: num_cjs_default = 42;\nvar h2: num_cjs_default = 'asdf'; // Error: string ~> number\n\n/////////////////////////////////////////////////////////////////\n// == Import Typeof With Non-Class Value (CJS Named Export) == //\n/////////////////////////////////////////////////////////////////\n\nimport typeof {num as num_cjs_named} from \"./ExportCJSNamed_Number\";\n\nvar i1: num_cjs_named = 42;\nvar i2: num_cjs_named = 'asdf'; // Error: string ~> number\n\n///////////////////////////////////////////////\n// == Import Typeof ModuleNamespaceObject == //\n///////////////////////////////////////////////\n\nimport typeof * as ModuleNSObjT from \"./ExportNamed_Multi\";\nvar j1: ModuleNSObjT = {num: 42, str: 'asdf'};\nvar j2: ModuleNSObjT = {num: 42, str: 42}; // Error: number ~> string") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\n///////////////////////////////////////////////////\n// == Importing Class Typeof (Default Export) == //\n///////////////////////////////////////////////////\n\nimport typeof ClassFoo1T from \"./ExportDefault_Class\";\nimport ClassFoo1 from \"./ExportDefault_Class\";\n\nvar a1: ClassFoo1T = ClassFoo1;\nvar a2: ClassFoo1T = new ClassFoo1(); // Error: ClassFoo1 (inst) ~> ClassFoo1 (class)\nnew ClassFoo1T(); // Error: ClassFoo1T is not bound to a value\n\n/////////////////////////////////////////////////\n// == Importing Class Typeof (Named Export) == //\n/////////////////////////////////////////////////\n\nimport typeof { ClassFoo2 as ClassFoo2T } from \"./ExportNamed_Class\";\nimport { ClassFoo2 } from \"./ExportNamed_Class\";\n\nvar b1: ClassFoo2T = ClassFoo2;\nvar b2: ClassFoo2T = new ClassFoo2(); // Error: ClassFoo2 (inst) ~> ClassFoo2 (class)\nnew ClassFoo2T(); // Error: ClassFoo2T is not bound to a value\n\n///////////////////////////////////////////////////////\n// == Importing Class Typeof (CJS Default Export) == //\n///////////////////////////////////////////////////////\n\nimport typeof ClassFoo3T from \"./ExportCJSDefault_Class\";\nimport ClassFoo3 from \"./ExportCJSDefault_Class\";\n\nvar c1: ClassFoo3T = ClassFoo3;\nvar c2: ClassFoo3T = new ClassFoo3(); // Error: ClassFoo3 (inst) ~> ClassFoo3 (class)\n\n/////////////////////////////////////////////////////\n// == Importing Class Typeof (CJS Named Export) == //\n/////////////////////////////////////////////////////\n\nimport typeof { ClassFoo4 as ClassFoo4T } from \"./ExportCJSNamed_Class\";\nimport { ClassFoo4 } from \"./ExportCJSNamed_Class\";\n\nvar d1: ClassFoo4T = ClassFoo4;\nvar d2: ClassFoo4T = new ClassFoo4(); // Error: ClassFoo4 (inst) ~> ClassFoo4 (class)\n\n//////////////////////////////////////////////\n// == Import Typeof Alias (Named Export) == //\n//////////////////////////////////////////////\n\nimport typeof { AliasFoo3 } from \"./ExportNamed_Alias\"; // Error: Can't `import typeof` type aliases!\n\n////////////////////////////////////////////////\n// == Import Typeof Alias (Default Export) == //\n////////////////////////////////////////////////\n\n// TODO: No support for this right now. It's most likely possible, but it's\n//       unclear how useful it is at the moment and it entails a little\n//       more work than named type exports, so I'm punting on it for now.\n\n///////////////////////////////////////////////////////////////\n// == Import Typeof With Non-Class Value (Default Export) == //\n///////////////////////////////////////////////////////////////\n\nimport typeof num_default from \"./ExportDefault_Number\";\n\nvar f1: num_default = 42;\nvar f2: num_default = \"asdf\"; // Error: string ~> number\n\n/////////////////////////////////////////////////////////////\n// == Import Typeof With Non-Class Value (Named Export) == //\n/////////////////////////////////////////////////////////////\n\nimport typeof { num as num_named } from \"./ExportNamed_Number\";\n\nvar g1: num_named = 42;\nvar g2: num_named = \"asdf\"; // Error: string ~> number\n\n///////////////////////////////////////////////////////////////////\n// == Import Typeof With Non-Class Value (CJS Default Export) == //\n///////////////////////////////////////////////////////////////////\n\nimport typeof num_cjs_default from \"./ExportCJSDefault_Number\";\n\nvar h1: num_cjs_default = 42;\nvar h2: num_cjs_default = \"asdf\"; // Error: string ~> number\n\n/////////////////////////////////////////////////////////////////\n// == Import Typeof With Non-Class Value (CJS Named Export) == //\n/////////////////////////////////////////////////////////////////\n\nimport typeof { num as num_cjs_named } from \"./ExportCJSNamed_Number\";\n\nvar i1: num_cjs_named = 42;\nvar i2: num_cjs_named = \"asdf\"; // Error: string ~> number\n\n///////////////////////////////////////////////\n// == Import Typeof ModuleNamespaceObject == //\n///////////////////////////////////////////////\n\nimport typeof * as ModuleNSObjT from \"./ExportNamed_Multi\";\nvar j1: ModuleNSObjT = { num: 42, str: \"asdf\" };\nvar j2: ModuleNSObjT = { num: 42, str: 42 }; // Error: number ~> string");
    Ok(())
}
