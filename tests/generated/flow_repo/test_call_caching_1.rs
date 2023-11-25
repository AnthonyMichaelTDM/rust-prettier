#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_8b756246() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nconst Immutable = require('immutable');\n\nconst tasksPerStatusMap = new Map(\n  [].map(taskStatus => [taskStatus, new Map()]),\n);\nfor (let [taskStatus, tasksMap] of tasksPerStatusMap) {\n  tasksPerStatusMap.set(taskStatus, Immutable.Map(tasksMap));\n}") ? ;
    assert_eq ! (formatted , "// @flow\n\nconst Immutable = require(\"immutable\");\n\nconst tasksPerStatusMap = new Map(\n  [].map((taskStatus) => [taskStatus, new Map()]),\n);\nfor (let [taskStatus, tasksMap] of tasksPerStatusMap) {\n  tasksPerStatusMap.set(taskStatus, Immutable.Map(tasksMap));\n}");
    Ok(())
}
#[test]
fn test_test_2_js_format_1_9687c2cf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ndeclare class Bar<K> {\n  update<K_>(updater: (value: this) => Bar<K_>): Bar<K_>;\n}\n\ndeclare function foo<U>(\n  initialValue: U,\n  callbackfn: (previousValue: U) => U\n): U;\n\ndeclare var items: Bar<string>;\ndeclare var updater: (value: Bar<string>) => Bar<string>;\n\nfoo(\n  items,\n  (acc) => acc.update(updater)\n);") ? ;
    assert_eq ! (formatted , "/* @flow */\n\ndeclare class Bar<K> {\n  update<K_>(updater: (value: this) => Bar<K_>): Bar<K_>;\n}\n\ndeclare function foo<U>(\n  initialValue: U,\n  callbackfn: (previousValue: U) => U,\n): U;\n\ndeclare var items: Bar<string>;\ndeclare var updater: (value: Bar<string>) => Bar<string>;\n\nfoo(items, (acc) => acc.update(updater));");
    Ok(())
}
#[test]
fn test_test_3_js_format_1_3ca44e41() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare class ImmBox<T> {\n  static <U>(x: any): ImmBox<U>;\n  static (x: any): any;\n}\n\ndeclare class Box<T> {\n  constructor(x: T): void;\n  set(value: T): void;\n  get(): T;\n}\n\nconst outer = new Box();\nconst inner = outer.get();\nouter.set(ImmBox(inner));") ? ;
    assert_eq ! (formatted , "// @flow\n\ndeclare class ImmBox<T> {\n  static <U>(x: any): ImmBox<U>;\n  static (x: any): any;\n}\n\ndeclare class Box<T> {\n  constructor(x: T): void;\n  set(value: T): void;\n  get(): T;\n}\n\nconst outer = new Box();\nconst inner = outer.get();\nouter.set(ImmBox(inner));");
    Ok(())
}
