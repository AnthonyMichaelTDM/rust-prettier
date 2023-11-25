#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_10352_consistency_ts_format_1_d511cd13() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface Store {\n  getRecord(collectionName: string, documentPath: string): TaskEither<Error, Option<GenericRecord>>;\n}\n\nexport default class StoreImpl extends Service implements Store {\n  getRecord(collectionName: string, documentPath: string): TaskEither<Error, Option<GenericRecord>> {\n    // Do some stuff.\n  }\n}\n\nexport function loadPlugin(\n  name: string,\n  dirname: string,\n): { filepath: string, value: mixed } {\n  // ...\n}") ? ;
    assert_eq ! (formatted , "export interface Store {\n  getRecord(\n    collectionName: string,\n    documentPath: string,\n  ): TaskEither<Error, Option<GenericRecord>>;\n}\n\nexport default class StoreImpl extends Service implements Store {\n  getRecord(\n    collectionName: string,\n    documentPath: string,\n  ): TaskEither<Error, Option<GenericRecord>> {\n    // Do some stuff.\n  }\n}\n\nexport function loadPlugin(\n  name: string,\n  dirname: string,\n): { filepath: string; value: mixed } {\n  // ...\n}");
    Ok(())
}
#[test]
fn test_method_signature_ts_format_1_2dfda62a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = {\n  get(key: \"foo\"): \\`\n  \\`;\n};\ntype Foo = {\n  get(key: \"foo\"): \\`\\`;\n};\n\ntype Bar = {\n  get(key: \"bar\"): {\n    bar: \"bar\"\n  };\n}\ntype Bar = {\n  get(key: \"bar\"): { bar: \"bar\" };\n}") ? ;
    assert_eq ! (formatted , "type Foo = {\n  get(key: \"foo\"): \\`\n  \\`;\n};\ntype Foo = {\n  get(key: \"foo\"): \\`\\`;\n};\n\ntype Bar = {\n  get(key: \"bar\"): {\n    bar: \"bar\";\n  };\n};\ntype Bar = {\n  get(key: \"bar\"): { bar: \"bar\" };\n};");
    Ok(())
}
#[test]
fn test_method_signature_with_wrapped_return_type_ts_format_1_d1fce9d8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type ReleaseToolConfig = {\n  get(key: \"changelog\"): {\n    get(key: \"repo\"): string;\n    get(key: \"labels\"): Map<string, string>;\n  };\n};\n\ntype ReleaseToolConfig2 = {\n  get(key: \"changelog\"): \\`\n  \\`\n};") ? ;
    assert_eq ! (formatted , "type ReleaseToolConfig = {\n  get(key: \"changelog\"): {\n    get(key: \"repo\"): string;\n    get(key: \"labels\"): Map<string, string>;\n  };\n};\n\ntype ReleaseToolConfig2 = {\n  get(key: \"changelog\"): \\`\n  \\`;\n};");
    Ok(())
}
#[test]
fn test_semi_ts_format_1_8a21e15e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare module 'foo' {\n  function foo(namespace: string): void;\n  function bar(namespace: string): void;\n}\n\nfunction pickCard(x: {suit: string; card: number; }[]): number;\nfunction pickCard(x: number): {suit: string; card: number; };") ? ;
    assert_eq ! (formatted , "declare module \"foo\" {\n  function foo(namespace: string): void;\n  function bar(namespace: string): void;\n}\n\nfunction pickCard(x: { suit: string; card: number }[]): number;\nfunction pickCard(x: number): { suit: string; card: number };");
    Ok(())
}
#[test]
fn test_type_literal_optional_method_ts_format_1_e37c3568() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var v: { e?(): number };")?;
    assert_eq!(formatted, "var v: { e?(): number };");
    Ok(())
}
