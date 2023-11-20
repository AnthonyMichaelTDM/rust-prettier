#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_single_js_arrow_parensavoid_format_1_3c8889a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const selectorByPath:\n  Path\n => SomethingSelector<\n  SomethingUEditorContextType,\n  SomethingUEditorContextType,\n  SomethingBulkValue<string>\n> = memoizeWithArgs(/* ... */)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const selectorByPath: Path => SomethingSelector<\n  SomethingUEditorContextType,\n  SomethingUEditorContextType,\n  SomethingBulkValue<string>,\n> = memoizeWithArgs(/* ... */);");
}
#[test]
fn test_single_js_trailing_commaall_format_1_3c8889a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const selectorByPath:\n  Path\n => SomethingSelector<\n  SomethingUEditorContextType,\n  SomethingUEditorContextType,\n  SomethingBulkValue<string>\n> = memoizeWithArgs(/* ... */)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const selectorByPath: (Path) => SomethingSelector<\n  SomethingUEditorContextType,\n  SomethingUEditorContextType,\n  SomethingBulkValue<string>,\n> = memoizeWithArgs(/* ... */);");
}
#[test]
fn test_single_js_trailing_commaes_5_format_1_3c8889a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const selectorByPath:\n  Path\n => SomethingSelector<\n  SomethingUEditorContextType,\n  SomethingUEditorContextType,\n  SomethingBulkValue<string>\n> = memoizeWithArgs(/* ... */)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const selectorByPath: (Path) => SomethingSelector<\n  SomethingUEditorContextType,\n  SomethingUEditorContextType,\n  SomethingBulkValue<string>,\n> = memoizeWithArgs(/* ... */);");
}
#[test]
fn test_test_js_arrow_parensavoid_format_1_4b85d60a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("avoid")
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Banana = {\n  eat: string => boolean,\n};\n\ntype Hex = {n: 0x01};\n\ntype T1 = { method: (a) => void };\n\ntype T2 = { method(a): void };\n\ndeclare class X { method(a): void }\n\ndeclare function f(a): void;\n\nvar f: (a) => void;\n\ninterface F1 { m(string): number }\n\ninterface F2 { m: (string) => number }\n\nfunction f1(o: { f: (string) => void }) {}\n\nfunction f2(o: { f(string): void }) {}\n\ntype f3 = (...arg) => void;\n\ntype f4 = (/* comment */ arg) => void;\n\ntype f5 = (arg /* comment */) => void;\n\ntype f6 = (?arg) => void;\n\nclass Y {\n  constructor(\n    ideConnectionFactory: child_process$ChildProcess => FlowIDEConnection =\n        defaultIDEConnectionFactory,\n  ) {\n  }\n}\n\ninterface F {\n  ideConnectionFactoryLongLongLong: (child_process$ChildProcess) => FlowIDEConnection\n}\n\ntype ExtractType = <A>(B<C>) => D\n\ntype T3 = ?(() => A);\n\ntype T4 = ?(() => A) | B;\n\ntype T5 = ?() => A | B;\n\ntype T6 = (?() => A) | B;\n\n// https://github.com/babel/babel/issues/7924\n//type T = ??() => A;\n\ntype T7 = ?(?(() => A));\n\ntype T8 = ?(?() => A) | B;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Banana = {\n  eat: string => boolean,\n};\n\ntype Hex = { n: 0x01 };\n\ntype T1 = { method: a => void };\n\ntype T2 = { method(a): void };\n\ndeclare class X {\n  method(a): void;\n}\n\ndeclare function f(a): void;\n\nvar f: a => void;\n\ninterface F1 {\n  m(string): number;\n}\n\ninterface F2 {\n  m: string => number;\n}\n\nfunction f1(o: { f: string => void }) {}\n\nfunction f2(o: { f(string): void }) {}\n\ntype f3 = (...arg) => void;\n\ntype f4 = /* comment */ arg => void;\n\ntype f5 = arg /* comment */ => void;\n\ntype f6 = (?arg) => void;\n\nclass Y {\n  constructor(\n    ideConnectionFactory: child_process$ChildProcess => FlowIDEConnection = defaultIDEConnectionFactory,\n  ) {}\n}\n\ninterface F {\n  ideConnectionFactoryLongLongLong: child_process$ChildProcess => FlowIDEConnection;\n}\n\ntype ExtractType = <A>(B<C>) => D;\n\ntype T3 = ?() => A;\n\ntype T4 = ?(() => A) | B;\n\ntype T5 = ?() => A | B;\n\ntype T6 = ?(() => A) | B;\n\n// https://github.com/babel/babel/issues/7924\n//type T = ??() => A;\n\ntype T7 = ??(() => A);\n\ntype T8 = ??(() => A) | B;");
}
#[test]
fn test_test_js_trailing_commaall_format_1_4b85d60a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Banana = {\n  eat: string => boolean,\n};\n\ntype Hex = {n: 0x01};\n\ntype T1 = { method: (a) => void };\n\ntype T2 = { method(a): void };\n\ndeclare class X { method(a): void }\n\ndeclare function f(a): void;\n\nvar f: (a) => void;\n\ninterface F1 { m(string): number }\n\ninterface F2 { m: (string) => number }\n\nfunction f1(o: { f: (string) => void }) {}\n\nfunction f2(o: { f(string): void }) {}\n\ntype f3 = (...arg) => void;\n\ntype f4 = (/* comment */ arg) => void;\n\ntype f5 = (arg /* comment */) => void;\n\ntype f6 = (?arg) => void;\n\nclass Y {\n  constructor(\n    ideConnectionFactory: child_process$ChildProcess => FlowIDEConnection =\n        defaultIDEConnectionFactory,\n  ) {\n  }\n}\n\ninterface F {\n  ideConnectionFactoryLongLongLong: (child_process$ChildProcess) => FlowIDEConnection\n}\n\ntype ExtractType = <A>(B<C>) => D\n\ntype T3 = ?(() => A);\n\ntype T4 = ?(() => A) | B;\n\ntype T5 = ?() => A | B;\n\ntype T6 = (?() => A) | B;\n\n// https://github.com/babel/babel/issues/7924\n//type T = ??() => A;\n\ntype T7 = ?(?(() => A));\n\ntype T8 = ?(?() => A) | B;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Banana = {\n  eat: (string) => boolean,\n};\n\ntype Hex = { n: 0x01 };\n\ntype T1 = { method: (a) => void };\n\ntype T2 = { method(a): void };\n\ndeclare class X {\n  method(a): void;\n}\n\ndeclare function f(a): void;\n\nvar f: (a) => void;\n\ninterface F1 {\n  m(string): number;\n}\n\ninterface F2 {\n  m: (string) => number;\n}\n\nfunction f1(o: { f: (string) => void }) {}\n\nfunction f2(o: { f(string): void }) {}\n\ntype f3 = (...arg) => void;\n\ntype f4 = (/* comment */ arg) => void;\n\ntype f5 = (arg /* comment */) => void;\n\ntype f6 = (?arg) => void;\n\nclass Y {\n  constructor(\n    ideConnectionFactory: (child_process$ChildProcess) => FlowIDEConnection = defaultIDEConnectionFactory,\n  ) {}\n}\n\ninterface F {\n  ideConnectionFactoryLongLongLong: (child_process$ChildProcess) => FlowIDEConnection;\n}\n\ntype ExtractType = <A>(B<C>) => D;\n\ntype T3 = ?() => A;\n\ntype T4 = ?(() => A) | B;\n\ntype T5 = ?() => A | B;\n\ntype T6 = ?(() => A) | B;\n\n// https://github.com/babel/babel/issues/7924\n//type T = ??() => A;\n\ntype T7 = ??(() => A);\n\ntype T8 = ??(() => A) | B;");
}
#[test]
fn test_test_js_trailing_commaes_5_format_1_4b85d60a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Banana = {\n  eat: string => boolean,\n};\n\ntype Hex = {n: 0x01};\n\ntype T1 = { method: (a) => void };\n\ntype T2 = { method(a): void };\n\ndeclare class X { method(a): void }\n\ndeclare function f(a): void;\n\nvar f: (a) => void;\n\ninterface F1 { m(string): number }\n\ninterface F2 { m: (string) => number }\n\nfunction f1(o: { f: (string) => void }) {}\n\nfunction f2(o: { f(string): void }) {}\n\ntype f3 = (...arg) => void;\n\ntype f4 = (/* comment */ arg) => void;\n\ntype f5 = (arg /* comment */) => void;\n\ntype f6 = (?arg) => void;\n\nclass Y {\n  constructor(\n    ideConnectionFactory: child_process$ChildProcess => FlowIDEConnection =\n        defaultIDEConnectionFactory,\n  ) {\n  }\n}\n\ninterface F {\n  ideConnectionFactoryLongLongLong: (child_process$ChildProcess) => FlowIDEConnection\n}\n\ntype ExtractType = <A>(B<C>) => D\n\ntype T3 = ?(() => A);\n\ntype T4 = ?(() => A) | B;\n\ntype T5 = ?() => A | B;\n\ntype T6 = (?() => A) | B;\n\n// https://github.com/babel/babel/issues/7924\n//type T = ??() => A;\n\ntype T7 = ?(?(() => A));\n\ntype T8 = ?(?() => A) | B;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Banana = {\n  eat: (string) => boolean,\n};\n\ntype Hex = { n: 0x01 };\n\ntype T1 = { method: (a) => void };\n\ntype T2 = { method(a): void };\n\ndeclare class X {\n  method(a): void;\n}\n\ndeclare function f(a): void;\n\nvar f: (a) => void;\n\ninterface F1 {\n  m(string): number;\n}\n\ninterface F2 {\n  m: (string) => number;\n}\n\nfunction f1(o: { f: (string) => void }) {}\n\nfunction f2(o: { f(string): void }) {}\n\ntype f3 = (...arg) => void;\n\ntype f4 = (/* comment */ arg) => void;\n\ntype f5 = (arg /* comment */) => void;\n\ntype f6 = (?arg) => void;\n\nclass Y {\n  constructor(\n    ideConnectionFactory: (child_process$ChildProcess) => FlowIDEConnection = defaultIDEConnectionFactory\n  ) {}\n}\n\ninterface F {\n  ideConnectionFactoryLongLongLong: (child_process$ChildProcess) => FlowIDEConnection;\n}\n\ntype ExtractType = <A>(B<C>) => D;\n\ntype T3 = ?() => A;\n\ntype T4 = ?(() => A) | B;\n\ntype T5 = ?() => A | B;\n\ntype T6 = ?(() => A) | B;\n\n// https://github.com/babel/babel/issues/7924\n//type T = ??() => A;\n\ntype T7 = ??(() => A);\n\ntype T8 = ??(() => A) | B;");
}
