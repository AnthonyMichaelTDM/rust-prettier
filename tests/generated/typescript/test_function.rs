#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_single_expand_ts_format_1_05cd8f6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function onDidInsertSuggestion({\n  editor,\n  triggerPosition,\n  re\n}): Promise<void> {\n}\n\nclass X {\n  async onDidInsertSuggestion({editor, triggerPosition, suggestion}): Promise<\n    void\n  > {\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function onDidInsertSuggestion({\n  editor,\n  triggerPosition,\n  re,\n}): Promise<void> {}\n\nclass X {\n  async onDidInsertSuggestion({\n    editor,\n    triggerPosition,\n    suggestion,\n  }): Promise<void> {}\n}");
}
