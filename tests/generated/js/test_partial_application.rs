#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_test_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_test_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_test_js_format_1_d618eab4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const addOne = add(1, ?); // apply from the left\naddOne(2); // 3\n\nconst addTen = add(?, 10); // apply from the right\naddTen(2); // 12\n\n// with pipeline\nlet newScore = player.score\n  |> add(7, ?)\n  |> clamp(0, 100, ?); // shallow stack, the pipe to \\`clamp\\` is the same frame as the pipe to \\`add\\`.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const addOne = add(1, ?); // apply from the left\naddOne(2); // 3\n\nconst addTen = add(?, 10); // apply from the right\naddTen(2); // 12\n\n// with pipeline\nlet newScore = player.score |> add(7, ?) |> clamp(0, 100, ?); // shallow stack, the pipe to \\`clamp\\` is the same frame as the pipe to \\`add\\`.");
}
