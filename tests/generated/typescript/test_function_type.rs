#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_consistent_ts_format_1_e7f4bcc1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// TSFunctionType\ntype A = (\n  tpl: TemplateStringsArray,\n  ...args: Array<unknown>\n) => (replacements?: PublicReplacements) => T;\n\n// TSConstructorType\ntype B = new (\n  tpl: TemplateStringsArray,\n  ...args: Array<unknown>\n) => (replacements?: PublicReplacements) => T;\n\ntype X = {\n  // TSCallSignatureDeclaration\n  (\n    tpl: TemplateStringsArray,\n    ...args: Array<unknown>\n  ): (replacements?: PublicReplacements) => T;\n\n  // TSConstructSignatureDeclaration\n  new (\n    tpl: TemplateStringsArray,\n    ...args: Array<unknown>\n  ): (replacements?: PublicReplacements) => T;\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// TSFunctionType\ntype A = (\n  tpl: TemplateStringsArray,\n  ...args: Array<unknown>\n) => (replacements?: PublicReplacements) => T;\n\n// TSConstructorType\ntype B = new (\n  tpl: TemplateStringsArray,\n  ...args: Array<unknown>\n) => (replacements?: PublicReplacements) => T;\n\ntype X = {\n  // TSCallSignatureDeclaration\n  (\n    tpl: TemplateStringsArray,\n    ...args: Array<unknown>\n  ): (replacements?: PublicReplacements) => T;\n\n  // TSConstructSignatureDeclaration\n  new (\n    tpl: TemplateStringsArray,\n    ...args: Array<unknown>\n  ): (replacements?: PublicReplacements) => T;\n};");
}
#[test]
fn test_single_parameter_ts_format_1_645a1145() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type X = (options:{ a: string; b: AbstractCompositeThingamabobberFactoryProvider}) => {};\ntype Y = new (options:{ a: string; b: AbstractCompositeThingamabobberFactoryProvider}) => {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type X = (options: {\n  a: string;\n  b: AbstractCompositeThingamabobberFactoryProvider;\n}) => {};\ntype Y = new (options: {\n  a: string;\n  b: AbstractCompositeThingamabobberFactoryProvider;\n}) => {};");
}
#[test]
fn test_type_annotation_ts_format_1_080d14ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const foo = (): () => void => (): void => null;\nconst bar = (): (() => void) => (): void => null;\nconst baz = (): ((() => void)) => (): void => null;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo = (): (() => void) => (): void => null;\nconst bar = (): (() => void) => (): void => null;\nconst baz = (): (() => void) => (): void => null;");
}
