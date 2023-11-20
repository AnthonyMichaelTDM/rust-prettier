#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_es_declare_module_js_format_1_93fa5d7b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport {num1, str1} from \"CJS_Named\";\nimport CJS_Named from \"CJS_Named\";\n(num1: number);\n(num1: string); // Error: number ~> string\n(str1: string);\n(str1: number); // Error: string ~> number\n(CJS_Named: {num1: number, str1: string});\n(CJS_Named: number); // Error: Module ~> number\n\nimport {num2} from \"CJS_Clobbered\"; // Error: No such export!\nimport {numExport} from \"CJS_Clobbered\";\n(numExport: number);\n(numExport: string); // Error: number ~> string\nimport type {numType} from \"CJS_Clobbered\";\n(42: numType);\n('asdf': numType); // Error: string ~> number\n\nimport {strHidden} from \"ES\"; // Error: No such export!\nimport {str3} from \"ES\";\n(str3: string);\n(str3: number); // Error: string ~> number\n\nimport {num3} from \"ES\";\n(num3: number);\n(num3: string); // Error: number ~> string\n\nimport {C} from \"ES\";\nimport type {C as CType} from \"ES\";\n(new C(): C);\n(42: C); // Error: number ~> C\n(new C(): CType);\n(42: CType); // Error: number ~> CType\n\nimport {T} from \"ES\"; // Error: T is a type import, not a value\nimport type {T as T2} from \"ES\";\n(42: T2);\n('asdf': T2); // Error: string ~> number\n\nimport {exports as nope} from \"ES\"; // Error: Not an export") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport { num1, str1 } from \"CJS_Named\";\nimport CJS_Named from \"CJS_Named\";\n(num1: number);\n(num1: string); // Error: number ~> string\n(str1: string);\n(str1: number); // Error: string ~> number\n(CJS_Named: { num1: number, str1: string });\n(CJS_Named: number); // Error: Module ~> number\n\nimport { num2 } from \"CJS_Clobbered\"; // Error: No such export!\nimport { numExport } from \"CJS_Clobbered\";\n(numExport: number);\n(numExport: string); // Error: number ~> string\nimport type { numType } from \"CJS_Clobbered\";\n(42: numType);\n(\"asdf\": numType); // Error: string ~> number\n\nimport { strHidden } from \"ES\"; // Error: No such export!\nimport { str3 } from \"ES\";\n(str3: string);\n(str3: number); // Error: string ~> number\n\nimport { num3 } from \"ES\";\n(num3: number);\n(num3: string); // Error: number ~> string\n\nimport { C } from \"ES\";\nimport type { C as CType } from \"ES\";\n(new C(): C);\n(42: C); // Error: number ~> C\n(new C(): CType);\n(42: CType); // Error: number ~> CType\n\nimport { T } from \"ES\"; // Error: T is a type import, not a value\nimport type { T as T2 } from \"ES\";\n(42: T2);\n(\"asdf\": T2); // Error: string ~> number\n\nimport { exports as nope } from \"ES\"; // Error: Not an export");
}
