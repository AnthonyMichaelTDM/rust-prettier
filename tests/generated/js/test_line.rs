#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_windows_js_format_1_a7a3b5a9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const vscode = require(\"vscode\");\nconst {getDir, getActiveFile, uint8arrayToString} = require(\"./utils\");\n\nlet outChannel;\nlet _commands;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const vscode = require(\"vscode\");\nconst { getDir, getActiveFile, uint8arrayToString } = require(\"./utils\");\n\nlet outChannel;\nlet _commands;");
}
