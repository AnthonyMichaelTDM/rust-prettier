#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_windows_js_format_1_a7a3b5a9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const vscode = require(\"vscode\");\nconst {getDir, getActiveFile, uint8arrayToString} = require(\"./utils\");\n\nlet outChannel;\nlet _commands;") ? ;
    assert_eq ! (formatted , "const vscode = require(\"vscode\");\nconst { getDir, getActiveFile, uint8arrayToString } = require(\"./utils\");\n\nlet outChannel;\nlet _commands;");
    Ok(())
}
