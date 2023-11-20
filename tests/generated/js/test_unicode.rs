#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_combining_characters_js_format_1_e51e6f0b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const x = [\"ÁÀĀÉÈĒẸE\u{329}Ẹ\u{301}É\u{329}Ẹ\u{300}È\u{329}Ẹ\u{304}Ē\u{329}ÍÌĪÓÒŌỌO\u{329}Ọ\u{301}Ó\u{329}Ọ\u{300}Ò\u{329}Ọ\u{304}Ō\u{329}ÚÙŪṢS\u{329}áàāéèēẹe\u{329}ẹ\u{301}é\u{329}ẹ\u{300}è\u{329}ẹ\u{304}ē\u{329}íìīóòōọo\u{329}ọ\u{301}ó\u{329}ọ\u{300}ò\u{329}ọ\u{304}ō\u{329}úùū\"];\n//345678901234567890123456789012345678901234567890123456789012345678901234567890\n//       1         2         3         4         5         6         7         8") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const x = [\"ÁÀĀÉÈĒẸE\u{329}Ẹ\u{301}É\u{329}Ẹ\u{300}È\u{329}Ẹ\u{304}Ē\u{329}ÍÌĪÓÒŌỌO\u{329}Ọ\u{301}Ó\u{329}Ọ\u{300}Ò\u{329}Ọ\u{304}Ō\u{329}ÚÙŪṢS\u{329}áàāéèēẹe\u{329}ẹ\u{301}é\u{329}ẹ\u{300}è\u{329}ẹ\u{304}ē\u{329}íìīóòōọo\u{329}ọ\u{301}ó\u{329}ọ\u{300}ò\u{329}ọ\u{304}ō\u{329}úùū\"];\n//345678901234567890123456789012345678901234567890123456789012345678901234567890\n//       1         2         3         4         5         6         7         8");
}
#[test]
fn test_keys_js_format_1_3c3120cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("({'この事はつもり素晴らしいことさ': '35jL9V'})");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "({ この事はつもり素晴らしいことさ: \"35jL9V\" });"
    );
}
#[test]
fn test_nbsp_jsx_js_format_1_e12d051b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Note: there are non breaking spaces in the JSX text\nx = <p>\u{a0}aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\u{a0}</p>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Note: there are non breaking spaces in the JSX text\nx = (\n  <p>\n    \u{a0}aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\u{a0}\n  </p>\n);");
}
