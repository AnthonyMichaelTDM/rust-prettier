#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_break_js_trailing_commaall_format_1_4bb09f7f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var X = {\n  perform: function<    \n     A, B, C, D, E, F, G,     \n     T: (a: A, b: B, c: C, d: D, e: E, f: F) => G // eslint-disable-line space-before-function-paren\n   >(     \n     method: T, scope: any,     \n     a: A, b: B, c: C, d: D, e: E, f: F,    \n   ): G {\n  }\n}") ? ;
    assert_eq ! (formatted , "var X = {\n  perform: function <\n    A,\n    B,\n    C,\n    D,\n    E,\n    F,\n    G,\n    T: (a: A, b: B, c: C, d: D, e: E, f: F) => G, // eslint-disable-line space-before-function-paren\n  >(method: T, scope: any, a: A, b: B, c: C, d: D, e: E, f: F): G {},\n};");
    Ok(())
}
#[test]
fn test_break_js_trailing_commaes_5_format_1_4bb09f7f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var X = {\n  perform: function<    \n     A, B, C, D, E, F, G,     \n     T: (a: A, b: B, c: C, d: D, e: E, f: F) => G // eslint-disable-line space-before-function-paren\n   >(     \n     method: T, scope: any,     \n     a: A, b: B, c: C, d: D, e: E, f: F,    \n   ): G {\n  }\n}") ? ;
    assert_eq ! (formatted , "var X = {\n  perform: function <\n    A,\n    B,\n    C,\n    D,\n    E,\n    F,\n    G,\n    T: (a: A, b: B, c: C, d: D, e: E, f: F) => G, // eslint-disable-line space-before-function-paren\n  >(method: T, scope: any, a: A, b: B, c: C, d: D, e: E, f: F): G {},\n};");
    Ok(())
}
#[test]
fn test_function_default_type_parameters_js_trailing_commaall_format_1_988f34a3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function foo<T: any = number>(): any {}")?;
    assert_eq!(formatted, "function foo<T: any = number>(): any {}");
    Ok(())
}
#[test]
fn test_function_default_type_parameters_js_trailing_commaes_5_format_1_988f34a3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("function foo<T: any = number>(): any {}")?;
    assert_eq!(formatted, "function foo<T: any = number>(): any {}");
    Ok(())
}
#[test]
fn test_generic_js_trailing_commaall_format_1_65531e0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const identity = <T>(t: T): T => t;\nconst a = 1;")?;
    assert_eq!(
        formatted,
        "const identity = <T>(t: T): T => t;\nconst a = 1;"
    );
    Ok(())
}
#[test]
fn test_generic_js_trailing_commaes_5_format_1_65531e0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const identity = <T>(t: T): T => t;\nconst a = 1;")?;
    assert_eq!(
        formatted,
        "const identity = <T>(t: T): T => t;\nconst a = 1;"
    );
    Ok(())
}
#[test]
fn test_interface_js_trailing_commaall_format_1_d87a9298() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("interface A { 'C': string; }\ninterface B { \"D\": boolean; }")?;
    assert_eq!(
        formatted,
        "interface A {\n  C: string;\n}\ninterface B {\n  D: boolean;\n}"
    );
    Ok(())
}
#[test]
fn test_interface_js_trailing_commaes_5_format_1_d87a9298() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("interface A { 'C': string; }\ninterface B { \"D\": boolean; }")?;
    assert_eq!(
        formatted,
        "interface A {\n  C: string;\n}\ninterface B {\n  D: boolean;\n}"
    );
    Ok(())
}
#[test]
fn test_nullable_js_trailing_commaall_format_1_a12194cd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function formatEntireFile(\n  fileVersion: FileVersion,\n  range: atom$Range,\n): Promise<?{\n  newCursor?: number,\n  formatted: string,\n}> {}\n\nfunction foo(): Promise<?boolean> {}") ? ;
    assert_eq ! (formatted , "function formatEntireFile(\n  fileVersion: FileVersion,\n  range: atom$Range,\n): Promise<?{\n  newCursor?: number,\n  formatted: string,\n}> {}\n\nfunction foo(): Promise<?boolean> {}");
    Ok(())
}
#[test]
fn test_nullable_js_trailing_commaes_5_format_1_a12194cd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function formatEntireFile(\n  fileVersion: FileVersion,\n  range: atom$Range,\n): Promise<?{\n  newCursor?: number,\n  formatted: string,\n}> {}\n\nfunction foo(): Promise<?boolean> {}") ? ;
    assert_eq ! (formatted , "function formatEntireFile(\n  fileVersion: FileVersion,\n  range: atom$Range\n): Promise<?{\n  newCursor?: number,\n  formatted: string,\n}> {}\n\nfunction foo(): Promise<?boolean> {}");
    Ok(())
}
#[test]
fn test_single_identifier_js_trailing_commaall_format_1_cbd3ab26() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const longVariableName: Array<number> = this.foo.bar.baz.collider.body.vertices.reduce();",
    )?;
    assert_eq ! (formatted , "const longVariableName: Array<number> =\n  this.foo.bar.baz.collider.body.vertices.reduce();");
    Ok(())
}
#[test]
fn test_single_identifier_js_trailing_commaes_5_format_1_cbd3ab26() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const longVariableName: Array<number> = this.foo.bar.baz.collider.body.vertices.reduce();",
    )?;
    assert_eq ! (formatted , "const longVariableName: Array<number> =\n  this.foo.bar.baz.collider.body.vertices.reduce();");
    Ok(())
}
#[test]
fn test_trailing_js_trailing_commaall_format_1_86b01db1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type State = {\n  errors: Immutable.Map<\n    Ahohohhohohohohohohohohohohooh,\n    Fbt | Immutable.Map<ErrorIndex, Fbt>\n  >,\n  shouldValidate: boolean,\n};") ? ;
    assert_eq ! (formatted , "type State = {\n  errors: Immutable.Map<\n    Ahohohhohohohohohohohohohohooh,\n    Fbt | Immutable.Map<ErrorIndex, Fbt>,\n  >,\n  shouldValidate: boolean,\n};");
    Ok(())
}
#[test]
fn test_trailing_js_trailing_commaes_5_format_1_86b01db1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type State = {\n  errors: Immutable.Map<\n    Ahohohhohohohohohohohohohohooh,\n    Fbt | Immutable.Map<ErrorIndex, Fbt>\n  >,\n  shouldValidate: boolean,\n};") ? ;
    assert_eq ! (formatted , "type State = {\n  errors: Immutable.Map<\n    Ahohohhohohohohohohohohohohooh,\n    Fbt | Immutable.Map<ErrorIndex, Fbt>,\n  >,\n  shouldValidate: boolean,\n};");
    Ok(())
}
#[test]
fn test_type_js_trailing_commaall_format_1_032ad29e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type F = <T>(T) => T;\ntype G = (<A, B>(A) => B);\ntype H = { 'A': string, \"B\": number };") ? ;
    assert_eq!(
        formatted,
        "type F = <T>(T) => T;\ntype G = <A, B>(A) => B;\ntype H = { A: string, B: number };"
    );
    Ok(())
}
#[test]
fn test_type_js_trailing_commaes_5_format_1_032ad29e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type F = <T>(T) => T;\ntype G = (<A, B>(A) => B);\ntype H = { 'A': string, \"B\": number };") ? ;
    assert_eq!(
        formatted,
        "type F = <T>(T) => T;\ntype G = <A, B>(A) => B;\ntype H = { A: string, B: number };"
    );
    Ok(())
}
#[test]
fn test_union_js_trailing_commaall_format_1_75aefe76() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = Promise<\n  { ok: true, bar: string, baz: SomeOtherLongType } | \n  { ok: false, bar: SomeOtherLongType }\n>;") ? ;
    assert_eq ! (formatted , "type Foo = Promise<\n  | { ok: true, bar: string, baz: SomeOtherLongType }\n  | { ok: false, bar: SomeOtherLongType },\n>;");
    Ok(())
}
#[test]
fn test_union_js_trailing_commaes_5_format_1_75aefe76() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = Promise<\n  { ok: true, bar: string, baz: SomeOtherLongType } | \n  { ok: false, bar: SomeOtherLongType }\n>;") ? ;
    assert_eq ! (formatted , "type Foo = Promise<\n  | { ok: true, bar: string, baz: SomeOtherLongType }\n  | { ok: false, bar: SomeOtherLongType },\n>;");
    Ok(())
}
