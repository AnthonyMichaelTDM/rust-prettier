#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_boolean_js_format_1_b8f5f320() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction veryOptimistic(isThisAwesome: true): boolean {\n  return isThisAwesome;\n}\n\nvar x : boolean = veryOptimistic(true);\nvar y : boolean = veryOptimistic(false); // error\n\nfunction veryPessimistic(isThisAwesome: true): boolean {\n  return !isThisAwesome; // test bool conversion\n}\n\nvar x : boolean = veryPessimistic(true);\nvar y : boolean = veryPessimistic(false); // error\n\ntype MyOwnBooleanLOL = true | false\n\nfunction bar(x: MyOwnBooleanLOL): false {\n  if (x) {\n    return x;\n  } else {\n    return !x;\n  }\n}\n\nbar(true);\nbar(false);\nbar(1); // error\n\nfunction alwaysFalsy(x: boolean): false {\n  if (x) {\n    return !x;\n  } else {\n    return x;\n  }\n}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction veryOptimistic(isThisAwesome: true): boolean {\n  return isThisAwesome;\n}\n\nvar x: boolean = veryOptimistic(true);\nvar y: boolean = veryOptimistic(false); // error\n\nfunction veryPessimistic(isThisAwesome: true): boolean {\n  return !isThisAwesome; // test bool conversion\n}\n\nvar x: boolean = veryPessimistic(true);\nvar y: boolean = veryPessimistic(false); // error\n\ntype MyOwnBooleanLOL = true | false;\n\nfunction bar(x: MyOwnBooleanLOL): false {\n  if (x) {\n    return x;\n  } else {\n    return !x;\n  }\n}\n\nbar(true);\nbar(false);\nbar(1); // error\n\nfunction alwaysFalsy(x: boolean): false {\n  if (x) {\n    return !x;\n  } else {\n    return x;\n  }\n}");
    Ok(())
}
#[test]
fn test_number_js_format_1_156abf8e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction highlander(howMany: 1): number {\n  return howMany; // there can be only one!\n}\n\nhighlander(1);\nhighlander(2); // error\n\n\ntype Foo = 1 | 2\n\nfunction bar(num: Foo): number {\n  return num + 1;\n}\n\nbar(1);\nbar(2);\nbar(3); // error\n\ntype ComparatorResult = -1 | 0 | 1\nfunction sort(fn: (x: any, y: any) => ComparatorResult) {}\nsort((x, y) => -1);") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction highlander(howMany: 1): number {\n  return howMany; // there can be only one!\n}\n\nhighlander(1);\nhighlander(2); // error\n\ntype Foo = 1 | 2;\n\nfunction bar(num: Foo): number {\n  return num + 1;\n}\n\nbar(1);\nbar(2);\nbar(3); // error\n\ntype ComparatorResult = -1 | 0 | 1;\nfunction sort(fn: (x: any, y: any) => ComparatorResult) {}\nsort((x, y) => -1);");
    Ok(())
}
#[test]
fn test_string_js_format_1_219d678b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype NoSpaces = \"foobar\"\n(\"foobar\": NoSpaces);\n\ntype HasSpaces = \"foo bar\"\n(\"foo bar\": HasSpaces);") ? ;
    assert_eq ! (formatted , "/* @flow */\n\ntype NoSpaces = \"foobar\";\n(\"foobar\": NoSpaces);\n\ntype HasSpaces = \"foo bar\";\n(\"foo bar\": HasSpaces);");
    Ok(())
}
