#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_classes_js_format_1_4c71f451() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass Foo {\n  type: 'foo';\n  foo: string;\n}\n\nclass Bar {\n  type: 'bar';\n  bar: string;\n}\n\ntype Foobar = Foo | Bar;\n\nfunction foobar(x: Foobar): string {\n  if (x.type === 'foo') {\n    return foo(x);\n  } else if (x.type === 'bar') {\n    return bar(x);\n  } else {\n    return 'unknown';\n  }\n}\n\nfunction foo(x: Foo): string {\n  return x.foo;\n}\n\nfunction bar(x: Bar): string {\n  return x.bar;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nclass Foo {\n  type: \"foo\";\n  foo: string;\n}\n\nclass Bar {\n  type: \"bar\";\n  bar: string;\n}\n\ntype Foobar = Foo | Bar;\n\nfunction foobar(x: Foobar): string {\n  if (x.type === \"foo\") {\n    return foo(x);\n  } else if (x.type === \"bar\") {\n    return bar(x);\n  } else {\n    return \"unknown\";\n  }\n}\n\nfunction foo(x: Foo): string {\n  return x.foo;\n}\n\nfunction bar(x: Bar): string {\n  return x.bar;\n}");
}
#[test]
fn test_interfaces_neg_js_format_1_c43ce1bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ndeclare interface IDataBase {\n  id: string,\n  name: string,\n}\n\ndeclare interface IUserData extends IDataBase {\n  kind: \"user\",\n}\n\ndeclare interface ISystemData extends IDataBase {\n  kind: \"system\",\n}\n\ndeclare type IData = IUserData | ISystemData;\n\nconst data: IData = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n}\n\nif (data.kind === \"user\") {\n  (data: ISystemData);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ndeclare interface IDataBase {\n  id: string;\n  name: string;\n}\n\ndeclare interface IUserData extends IDataBase {\n  kind: \"user\";\n}\n\ndeclare interface ISystemData extends IDataBase {\n  kind: \"system\";\n}\n\ndeclare type IData = IUserData | ISystemData;\n\nconst data: IData = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n};\n\nif (data.kind === \"user\") {\n  (data: ISystemData);\n}");
}
#[test]
fn test_interfaces_pos_js_format_1_008203a2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ndeclare interface IDataBase {\n  id: string,\n  name: string,\n}\n\ndeclare interface IUserData extends IDataBase {\n  kind: \"user\",\n}\n\ndeclare interface ISystemData extends IDataBase {\n  kind: \"system\",\n}\n\ndeclare type IData = IUserData | ISystemData;\n\nconst data: IData = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n}\n\nif (data.kind === \"system\") {\n  (data: ISystemData);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ndeclare interface IDataBase {\n  id: string;\n  name: string;\n}\n\ndeclare interface IUserData extends IDataBase {\n  kind: \"user\";\n}\n\ndeclare interface ISystemData extends IDataBase {\n  kind: \"system\";\n}\n\ndeclare type IData = IUserData | ISystemData;\n\nconst data: IData = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n};\n\nif (data.kind === \"system\") {\n  (data: ISystemData);\n}");
}
#[test]
fn test_type_decls_neg_js_format_1_be0965e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype DataBase = {\n  id: string,\n  name: string,\n};\n\ntype UserData = {\n  id: string,\n  name: string,\n  kind: \"user\",\n}\n\ntype SystemData = {\n  id: string,\n  name: string,\n  kind: \"system\",\n}\n\ndeclare type Data = UserData | SystemData;\n\nconst data: Data = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n}\n\nif (data.kind === \"user\") {\n  (data: SystemData);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ntype DataBase = {\n  id: string,\n  name: string,\n};\n\ntype UserData = {\n  id: string,\n  name: string,\n  kind: \"user\",\n};\n\ntype SystemData = {\n  id: string,\n  name: string,\n  kind: \"system\",\n};\n\ndeclare type Data = UserData | SystemData;\n\nconst data: Data = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n};\n\nif (data.kind === \"user\") {\n  (data: SystemData);\n}");
}
#[test]
fn test_type_decls_pos_js_format_1_c88669c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype DataBase = {\n  id: string,\n  name: string,\n};\n\ntype UserData = {\n  id: string,\n  name: string,\n  kind: \"user\",\n}\n\ntype SystemData = {\n  id: string,\n  name: string,\n  kind: \"system\",\n}\n\ndeclare type Data = UserData | SystemData;\n\nconst data: Data = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n}\n\nif (data.kind === \"system\") {\n  (data: SystemData);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ntype DataBase = {\n  id: string,\n  name: string,\n};\n\ntype UserData = {\n  id: string,\n  name: string,\n  kind: \"user\",\n};\n\ntype SystemData = {\n  id: string,\n  name: string,\n  kind: \"system\",\n};\n\ndeclare type Data = UserData | SystemData;\n\nconst data: Data = {\n  id: \"\",\n  name: \"\",\n  kind: \"system\",\n};\n\nif (data.kind === \"system\") {\n  (data: SystemData);\n}");
}
