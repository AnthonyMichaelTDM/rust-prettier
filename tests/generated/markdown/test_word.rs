#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_escape_md_prose_wrapalways_format_1_b50bf03b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .prose_wrap("always")
        .parsers(vec!["markdown"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("hello \\\\* world _ ~~ ya\n\nescape & html < entity > foo\n\nqweqwe \\\\\\\\ \\\\ \\\\1 123123\n\nasd &amp; asd &#132; 123\n\n123_123_123\n\n456 _ 456 _ 456\n\n123*123*123\n\n123 * 123 * 123\n\n## 类的 prototype 属性和\\\\_\\\\_proto\\\\_\\\\_属性\n\n123&#0;123\n\n123&#35;123\n\n123&#992;123\n\n123&#x1F609;123\n\n123&#X22;123\n\n123&#98765432;123") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "hello \\\\* world \\\\_ ~~ ya\n\nescape & html < entity > foo\n\nqweqwe \\\\\\\\ \\\\ \\\\1 123123\n\nasd &amp; asd &#132; 123\n\n123_123_123\n\n456 _ 456 _ 456\n\n123*123*123\n\n123 _ 123 _ 123\n\n## 类的 prototype 属性和\\\\_\\\\_proto\\\\_\\\\_属性\n\n123&#0;123\n\n123&#35;123\n\n123&#992;123\n\n123&#x1F609;123\n\n123&#X22;123\n\n123&#98765432;123");
}
