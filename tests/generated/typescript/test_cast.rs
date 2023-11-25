#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_as_const_ts_format_1_36f880a5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let x = '123' as const;\n\n// https://github.com/babel/babel/pull/11912\nx as boolean <= y; // (x as boolean) <= y;\nx as boolean ?? y; // (x as boolean) ?? y;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let x = \"123\" as const;\n\n// https://github.com/babel/babel/pull/11912\n(x as boolean) <= y; // (x as boolean) <= y;\n(x as boolean) ?? y; // (x as boolean) ?? y;");
}
#[test]
fn test_assert_and_assign_ts_format_1_1ce5cfa4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(a as number) = 42;\n({ a: (b as any) = 2000 } = x)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(a as number) = 42;\n({ a: (b as any) = 2000 } = x);"
    );
}
#[test]
fn test_generic_cast_ts_format_1_e9729711() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/prettier/prettier/issues/4171\nfunction y() {\n\n  const fits = <Immutable.Map<string, any>>fits();\n  const fitsObjLiteral = <Immutable.Map<string, any>>{ a: \"test\" };\n  const fitsArrayLiteral = <Immutable.Map<string, any>>[\"test\", \"test2\"]\n\n  const breakAfterCast = <Immutable.Map<string, any>>someExistingConfigMap.mergeDeep(fallbackOpts);\n\n  const stillTooLong = <Immutable.Map<string, boolean, number, object, null, undefined, any, void, never>>someExistingConfigMap.mergeDeep(fallbackOptions);\n\n  const stillTooLong2 = <Immutable.Map<string, boolean, number, object, null, undefined, any, void, never> | undefined>someExistingConfigMap.mergeDeep(fallbackOptions);\n\n  const stillTooLong3 = <Immutable.Map<string>>someExistingConfigMap.mergeDeep(fallbackOptions.someMethodWithLongName(param1, param2));\n\n  const stillTooLong4 = <Immutable.Map<string, boolean, number, object, null, undefined, any, void, never> | undefined>someExistingConfigMap.mergeDeep(fallbackOptions.someMethodWithLongName(param1, param2));\n  \n  const testObjLiteral = <Immutable.Map<string, any>>{ property1: \"myPropertyVal\" };\n\n  const testObjLiteral2 = <Immutable.Map<string, any, number, boolean, object, null, undefined, never, \"extra long\">>{ property1: \"myPropertyVal\" };\n\n  const testArrayLiteral = <Immutable.Map<string, any>>[\"first\", \"second\", \"third\"];\n\n  const testArrayLiteral2 = <Immutable.Map<string, any, number, boolean, object, null, undefined, never, \"extra long\">>[\"first\", \"second\", \"third\"];\n\n  const insideFuncCall = myFunc(param1, <Immutable.Map<string, any>>param2, param3)\n}\n\n// https://github.com/prettier/prettier/issues/4168\nfunction x() {\n  const fits = <PermissionsChecker<any> | undefined>(<any>permissions)[type];\n  const fitsObjLiteral = <PermissionsChecker<any> | undefined>{ a: \"test\" };\n  const fitsArrayLiteral = <PermissionsChecker<any> | undefined>[\"t1\", \"t2\"];\n\n  const breakAfterCast = <PermissionsChecker<any> | undefined>(<any>permissions)[receiverType];\n\n  const stillTooLong = <PermissionsChecker<object> | undefined | number | string | boolean>(<any>permissions)[receiverType];\n\n  const stillTooLong2 = <PermissionsChecker<object> | undefined | number | string | boolean | null | never>(<any>permissions)[receiverType];\n\n  const stillTooLong3 = <PermissionsChecker<object> | undefined>(<any>permissions.someMethodWithLongName(parameter1, parameter2))[receiverTypeLongName];\n\n  const stillTooLong4 = <PermissionsChecker<object> | undefined | number | string | boolean | null | never>(<any>permissions.someMethodWithLongName(parameter1, parameter2))[receiverTypeLongName];\n\n  const testObjLiteral =  <PermissionsChecker<any> | undefined>{ prop1: \"myPropVal\" };\n\n  const testObjLiteral2 = <PermissionsChecker<object> | undefined | number | string | boolean | null | never | object>{ prop1: \"myPropVal\" };\n\n  const testArrayLiteral = <PermissionsChecker<any> | undefined>[\"first\", \"second\", \"third\"];\n\n  const testArrayLiteral2 = <PermissionsChecker<object> | undefined | number | string | boolean | null | never | object>[\"first\", \"second\", \"third\"];\n\n  const insideFuncCall = myFunc(param1, <PermissionsChecker<any> | undefined>param2, param3)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/prettier/prettier/issues/4171\nfunction y() {\n  const fits = <Immutable.Map<string, any>>fits();\n  const fitsObjLiteral = <Immutable.Map<string, any>>{ a: \"test\" };\n  const fitsArrayLiteral = <Immutable.Map<string, any>>[\"test\", \"test2\"];\n\n  const breakAfterCast = <Immutable.Map<string, any>>(\n    someExistingConfigMap.mergeDeep(fallbackOpts)\n  );\n\n  const stillTooLong = <\n    Immutable.Map<\n      string,\n      boolean,\n      number,\n      object,\n      null,\n      undefined,\n      any,\n      void,\n      never\n    >\n  >someExistingConfigMap.mergeDeep(fallbackOptions);\n\n  const stillTooLong2 = <\n    | Immutable.Map<\n        string,\n        boolean,\n        number,\n        object,\n        null,\n        undefined,\n        any,\n        void,\n        never\n      >\n    | undefined\n  >someExistingConfigMap.mergeDeep(fallbackOptions);\n\n  const stillTooLong3 = <Immutable.Map<string>>(\n    someExistingConfigMap.mergeDeep(\n      fallbackOptions.someMethodWithLongName(param1, param2),\n    )\n  );\n\n  const stillTooLong4 = <\n    | Immutable.Map<\n        string,\n        boolean,\n        number,\n        object,\n        null,\n        undefined,\n        any,\n        void,\n        never\n      >\n    | undefined\n  >someExistingConfigMap.mergeDeep(\n    fallbackOptions.someMethodWithLongName(param1, param2),\n  );\n\n  const testObjLiteral = <Immutable.Map<string, any>>{\n    property1: \"myPropertyVal\",\n  };\n\n  const testObjLiteral2 = <\n    Immutable.Map<\n      string,\n      any,\n      number,\n      boolean,\n      object,\n      null,\n      undefined,\n      never,\n      \"extra long\"\n    >\n  >{ property1: \"myPropertyVal\" };\n\n  const testArrayLiteral = <Immutable.Map<string, any>>[\n    \"first\",\n    \"second\",\n    \"third\",\n  ];\n\n  const testArrayLiteral2 = <\n    Immutable.Map<\n      string,\n      any,\n      number,\n      boolean,\n      object,\n      null,\n      undefined,\n      never,\n      \"extra long\"\n    >\n  >[\"first\", \"second\", \"third\"];\n\n  const insideFuncCall = myFunc(\n    param1,\n    <Immutable.Map<string, any>>param2,\n    param3,\n  );\n}\n\n// https://github.com/prettier/prettier/issues/4168\nfunction x() {\n  const fits = <PermissionsChecker<any> | undefined>(<any>permissions)[type];\n  const fitsObjLiteral = <PermissionsChecker<any> | undefined>{ a: \"test\" };\n  const fitsArrayLiteral = <PermissionsChecker<any> | undefined>[\"t1\", \"t2\"];\n\n  const breakAfterCast = <PermissionsChecker<any> | undefined>(\n    (<any>permissions)[receiverType]\n  );\n\n  const stillTooLong = <\n    PermissionsChecker<object> | undefined | number | string | boolean\n  >(<any>permissions)[receiverType];\n\n  const stillTooLong2 = <\n    | PermissionsChecker<object>\n    | undefined\n    | number\n    | string\n    | boolean\n    | null\n    | never\n  >(<any>permissions)[receiverType];\n\n  const stillTooLong3 = <PermissionsChecker<object> | undefined>(\n    (<any>permissions.someMethodWithLongName(parameter1, parameter2))[\n      receiverTypeLongName\n    ]\n  );\n\n  const stillTooLong4 = <\n    | PermissionsChecker<object>\n    | undefined\n    | number\n    | string\n    | boolean\n    | null\n    | never\n  >(<any>permissions.someMethodWithLongName(parameter1, parameter2))[\n    receiverTypeLongName\n  ];\n\n  const testObjLiteral = <PermissionsChecker<any> | undefined>{\n    prop1: \"myPropVal\",\n  };\n\n  const testObjLiteral2 = <\n    | PermissionsChecker<object>\n    | undefined\n    | number\n    | string\n    | boolean\n    | null\n    | never\n    | object\n  >{ prop1: \"myPropVal\" };\n\n  const testArrayLiteral = <PermissionsChecker<any> | undefined>[\n    \"first\",\n    \"second\",\n    \"third\",\n  ];\n\n  const testArrayLiteral2 = <\n    | PermissionsChecker<object>\n    | undefined\n    | number\n    | string\n    | boolean\n    | null\n    | never\n    | object\n  >[\"first\", \"second\", \"third\"];\n\n  const insideFuncCall = myFunc(\n    param1,\n    <PermissionsChecker<any> | undefined>param2,\n    param3,\n  );\n}");
}
#[test]
fn test_hug_args_ts_format_1_a57fc7e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("postMessage(\n  <IActionMessage>{\n    context: item.context,\n    topic: item.topic\n  }\n);\n\nwindow.postMessage(\n  {\n    context: item.context,\n    topic: item.topic\n  } as IActionMessage\n);\n\npostMessages(\n  <IActionMessage[]>[\n    {\n      context: item.context,\n      topic: item.topic\n    }\n  ]\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "postMessage(<IActionMessage>{\n  context: item.context,\n  topic: item.topic,\n});\n\nwindow.postMessage({\n  context: item.context,\n  topic: item.topic,\n} as IActionMessage);\n\npostMessages(<IActionMessage[]>[\n  {\n    context: item.context,\n    topic: item.topic,\n  },\n]);");
}
#[test]
fn test_parenthesis_ts_format_1_c72e352e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<DocumentHighlightKind>(a ? b : c);\n<any>(() => {});\n\n<x>a || {};\n<x>a && [];\ntrue || <x>a;\n<x>a + <x>b;\n(<x>a) = 1;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<DocumentHighlightKind>(a ? b : c);\n<any>(() => {});\n\n<x>a || {};\n<x>a && [];\ntrue || <x>a;\n<x>a + <x>b;\n(<x>a) = 1;");
}
#[test]
fn test_tuple_and_record_ts_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_ts_format_1_c5c206f4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("breakAfterCast = <PermissionsChecker<any> | undefined>(<any>permissions)[receiverType];\nbreakAfterCast = <PermissionsChecker<any> | undefined>(<any>permissions)(#[receiverType]);\n\ntestObjLiteral =  <PermissionsChecker<any> | undefined>{ prop1: \"myPropVal\" };\ntestObjLiteral =  <PermissionsChecker<any> | undefined>#{ prop1: \"myPropVal\" };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "breakAfterCast = <PermissionsChecker<any> | undefined>(\n  (<any>permissions)[receiverType]\n);\nbreakAfterCast = <PermissionsChecker<any> | undefined>(\n  (<any>permissions)(#[receiverType])\n);\n\ntestObjLiteral = <PermissionsChecker<any> | undefined>{ prop1: \"myPropVal\" };\ntestObjLiteral = <PermissionsChecker<any> | undefined>#{ prop1: \"myPropVal\" };");
}
