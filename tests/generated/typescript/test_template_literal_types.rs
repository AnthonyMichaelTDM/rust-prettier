#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_template_literal_types_ts_format_1_51a3ffaf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel-ts", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let x: \\`foo-\\${infer bar}\\`;\ntype HelloWorld = \\`\\${Hello}, \\${World}\\`\ntype SeussFish = \\`\\${Quantity | Color} fish\\`;\ndeclare function setAlignment(value: \\`\\${VerticalAlignment}-\\${HorizontalAlignment}\\`): void;\ntype PropEventSource<T> = {\n  on(eventName: \\`\\${string & keyof T}Changed\\`, callback: () => void): void;\n};\ntype PropEventSource<T> = {\n  on<K extends string & keyof T>\n    (eventName: \\`\\${K}Changed\\`, callback: (newValue: T[K]) => void ): void;\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let x: \\`foo-\\${infer bar}\\`;\ntype HelloWorld = \\`\\${Hello}, \\${World}\\`;\ntype SeussFish = \\`\\${Quantity | Color} fish\\`;\ndeclare function setAlignment(\n  value: \\`\\${VerticalAlignment}-\\${HorizontalAlignment}\\`,\n): void;\ntype PropEventSource<T> = {\n  on(eventName: \\`\\${string & keyof T}Changed\\`, callback: () => void): void;\n};\ntype PropEventSource<T> = {\n  on<K extends string & keyof T>(\n    eventName: \\`\\${K}Changed\\`,\n    callback: (newValue: T[K]) => void,\n  ): void;\n};");
}
