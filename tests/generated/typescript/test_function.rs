#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_single_expand_ts_format_1_05cd8f6f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function onDidInsertSuggestion({\n  editor,\n  triggerPosition,\n  re\n}): Promise<void> {\n}\n\nclass X {\n  async onDidInsertSuggestion({editor, triggerPosition, suggestion}): Promise<\n    void\n  > {\n  }\n}") ? ;
    assert_eq ! (formatted , "function onDidInsertSuggestion({\n  editor,\n  triggerPosition,\n  re,\n}): Promise<void> {}\n\nclass X {\n  async onDidInsertSuggestion({\n    editor,\n    triggerPosition,\n    suggestion,\n  }): Promise<void> {}\n}");
    Ok(())
}
