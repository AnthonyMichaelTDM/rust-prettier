#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_catch_js_format_1_2e08dfa9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("try {}\ncatch(\n  // comment\n  foo\n) {}\n\ntry {}\ncatch(foo //comment\n) {}\n\ntry {}\ncatch(\n  /* comment */ foo\n) {}\n\ntry {}\ncatch(\n  foo /* comment */\n) {}\n\ntry {}\ncatch(\n  foo\n  /* comment */\n) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "try {\n} catch (\n  // comment\n  foo\n) {}\n\ntry {\n} catch (\n  foo //comment\n) {}\n\ntry {\n} catch (/* comment */ foo) {}\n\ntry {\n} catch (foo /* comment */) {}\n\ntry {\n} catch (\n  foo\n  /* comment */\n) {}");
}
#[test]
fn test_empty_js_format_1_20b8ad19() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("try {\n} catch (e) {\n}\nfinally {\n}\n\ntry {\n} catch (e) {\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "try {\n} catch (e) {\n} finally {\n}\n\ntry {\n} catch (e) {}"
    );
}
#[test]
fn test_try_js_format_1_d41d90e1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("try\n/* missing comment */\n{;}\nfinally {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "try {\n  /* missing comment */\n} finally {\n}");
}
