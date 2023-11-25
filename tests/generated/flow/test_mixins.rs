#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_format_1_80773783() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class D1<T> // 1\nmixins C<T> {}\n\ndeclare class D2<T> // 1\nmixins C<T> // 2\n{}\n\ndeclare class D3<T> // 1\n// 2\nmixins C<T> // 3\n{}\n\ndeclare class D4<T> // 1\n// 2\nextends B<T>\nmixins C<T> // 3\n{}\n\ndeclare class D5<T> // 1\nextends B<T>\n// 2\nmixins C<T> // 3\n{}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class D1<T> // 1\n  mixins C<T> {}\n\ndeclare class D2<T> // 1\n  mixins C<T> {\n  // 2\n}\n\ndeclare class D3<T> // 1\n  // 2\n  mixins C<T> {\n  // 3\n}\n\ndeclare class D4<T> // 1\n  // 2\n  extends B<T>\n  mixins C<T> {\n  // 3\n}\n\ndeclare class D5<T> // 1\n  extends B<T>\n  // 2\n  mixins C<T> {\n  // 3\n}");
}
#[test]
fn test_type_js_format_1_086802dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "declare class A<T> extends B<T> mixins C<T> {}\ndeclare class D<T> mixins C<T> {}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare class A<T> extends B<T> mixins C<T> {}\ndeclare class D<T> mixins C<T> {}"
    );
}
