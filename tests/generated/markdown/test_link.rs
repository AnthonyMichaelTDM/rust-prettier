#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_autolink_md_prose_wrapalways_format_1_40dbeecb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<https://www.example.com>\n\n<hello@example.com>\n\n<mailto:hello@example.com>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<https://www.example.com>\n\n<hello@example.com>\n\n<mailto:hello@example.com>"
    );
}
#[test]
fn test_encoded_link_md_prose_wrapalways_format_1_377fa761() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[link](https://www.google.fr/()foo->bar)\n[link](https://www.google.fr/foo->bar)\n[link](https://www.google.fr/foo-%3Ebar)\n[link](https://www.google.fr/foo-<bar)\n[link](https://www.google.fr/foo-%3Cbar)\n![link](https://www.google.fr/()foo->bar)\n![link](https://www.google.fr/foo->bar)\n![link](https://www.google.fr/foo-%3Ebar)\n![link](https://www.google.fr/foo-<bar)\n![link](https://www.google.fr/foo-%3Cbar)\n[link]: https://www.google.fr/()foo->bar\n[link]: https://www.google.fr/foo->bar\n[link]: https://www.google.fr/foo-%3Ebar\n[link]: https://www.google.fr/foo-<bar\n[link]: https://www.google.fr/foo-%3Cba") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[link](<https://www.google.fr/()foo-%3Ebar>)\n[link](https://www.google.fr/foo->bar) [link](https://www.google.fr/foo-%3Ebar)\n[link](https://www.google.fr/foo-<bar) [link](https://www.google.fr/foo-%3Cbar)\n![link](<https://www.google.fr/()foo-%3Ebar>)\n![link](https://www.google.fr/foo->bar)\n![link](https://www.google.fr/foo-%3Ebar)\n![link](https://www.google.fr/foo-<bar)\n![link](https://www.google.fr/foo-%3Cbar) [link]:\nhttps://www.google.fr/()foo->bar [link]: https://www.google.fr/foo->bar [link]:\nhttps://www.google.fr/foo-%3Ebar [link]: https://www.google.fr/foo-<bar [link]:\nhttps://www.google.fr/foo-%3Cbar");
}
#[test]
fn test_entity_md_prose_wrapalways_format_1_ec706949() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[Test](http://localhost:8080/test?language=DE&currency=EUR)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[Test](http://localhost:8080/test?language=DE&currency=EUR)"
    );
}
#[test]
fn test_long_md_prose_wrapalways_format_1_2927a2d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[podium context](https://github.schibsted.io/finn/podium/tree/master/packages/podium-context)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[podium context](https://github.schibsted.io/finn/podium/tree/master/packages/podium-context)");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_babfd39f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[hello](#world)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[hello](#world)");
}
#[test]
fn test_title_md_prose_wrapalways_format_1_11d571d3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[hello](#world \"title\")\n[hello](#world 'title')\n[hello](#world (title))\n\n[a](https://example.com \"\\\\\"\")\n[a](https://example.com '\\\\\"')\n[a](https://example.com (\\\\\"))\n\n[a](https://example.com \"\\\\'\")\n[a](https://example.com '\\\\'')\n[a](https://example.com (\\\\'))\n\n[a](https://example.com \"\\\\'\")\n[a](https://example.com '\\\\)')\n[a](https://example.com (\\\\)))\n\n[a](https://example.com \"\\\\\\\\\\\\\"\")\n[a](https://example.com '\\\\\\\\\\\\'')\n[a](https://example.com (\\\\\\\\\\\\)))\n\n[a](https://example.com \"\\\\\\\\'\")\n[a](https://example.com '\\\\\\\\\"')\n[a](https://example.com (\\\\\\\\\"))\n\n<!-- magical incantations -->\n\n[a](https://example.com \"\\\\\"')\")\n[a](https://example.com '\"\\\\')')\n[a](https://example.com (\"'\\\\)))") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[hello](#world \"title\") [hello](#world \"title\") [hello](#world \"title\")\n\n[a](https://example.com '\"') [a](https://example.com '\"')\n[a](https://example.com '\"')\n\n[a](https://example.com \"'\") [a](https://example.com \"'\")\n[a](https://example.com \"'\")\n\n[a](https://example.com \"'\") [a](https://example.com \")\")\n[a](https://example.com \")\")\n\n[a](https://example.com '\"') [a](https://example.com \"'\")\n[a](https://example.com \")\")\n\n[a](https://example.com \"'\") [a](https://example.com '\"')\n[a](https://example.com '\"')\n\n<!-- magical incantations -->\n\n[a](https://example.com \"\\\\\"')\") [a](https://example.com \"\\\\\"')\")\n[a](https://example.com \"\\\\\"')\")");
}
#[test]
fn test_url_md_prose_wrapalways_format_1_58bfe722() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("https://www.example.com/");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "https://www.example.com/");
}
