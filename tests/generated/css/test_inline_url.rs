use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_css_format_1_43c40f28() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("@font-face {\n  src: url();\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "@font-face {\n  src: url();\n}");
}
#[test]
fn test_inline_url_css_format_1_a0892fdc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".stringContentIsNotModifiedEvenIfInvalid {\n  background: url('@images');\n  background: url(\"$images\");\n  background: url('/+0PX!/\"\\\\a?~^[]{} $%#');\n  background: url(\"/+0PX!/'\\\\a?~^[]{} $%#\");\n  background: url(\n    \"whitespace-around-string\"\n  );\n}\n\n.validUnquotedUrls {\n  background: url(\n    whitespace-around-string\n  );\n  background-image: url(/images/product/simple_product_manager/breadcrumb/chevron_right.png);\n  background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mO4/B8AAqgB0yr7dJgAAAAASUVORK5CYII=);\n  background-image: url(data:application/font-woff;charset=utf-8;base64,ThisIsNormalBut/+0ThisIsLowerCased);\n  background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mO4/B8AAqgB0yr7dJgAAAAASUVORK5CYII=) center center no-repeat;\n  background: url(data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20width=%229%22%20height=%229%22%3E%3Cpath%20d=%22M0%203h3%22%20stroke=%22red%22/%3E%3C/svg%3E);\n  offset-path: url(#path);\n  background: url(data/+0ThisShouldNotBeLowerCased);\n  background: url(https://foo/A*3I8oSY6AKRMAAAAAAAAAAABkARQnAQ);\n  background: url(https://example.com/some/quite,long,url,with,commas.jpg);\n  background: url(http://123.example.com);\n}\n@import url(https://fonts.googleapis.com/css?family=Roboto:100,300,400,500,700,900&display=swap);\n\n.validUnqotedUrlsThatAreParsedByLess {\n  background: url(@foo);\n}\n\n.validUnquotedUrlsThatAreParsedBySass {\n  background: url($foo);\n  background: url($foo+$bar);\n  background: url($foo*3);\n  background: url($foo/$bar);\n}\n\n.invalidUnquotedUrlsButWeParseThemAnyway {\n  background: url(--var(foo-bar,#dadce0));\n  -fb-sprite: url(fbglyph:cross-outline, fig-white);\n}\n\n.number {\n  background-image: url(http://123.com);\n  background: url(path/to/123.jpg);\n  background: url(#123.foo);\n  background: no-repeat center/80% url(http://123.com);\n  background: no-repeat center/80% url(path/to/123.jpg);\n  background: no-repeat center/80% url(#123.foo);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".stringContentIsNotModifiedEvenIfInvalid {\n  background: url(\"@images\");\n  background: url(\"$images\");\n  background: url('/+0PX!/\"\\\\a?~^[]{} $%#');\n  background: url(\"/+0PX!/'\\\\a?~^[]{} $%#\");\n  background: url(\"whitespace-around-string\");\n}\n\n.validUnquotedUrls {\n  background: url(whitespace-around-string);\n  background-image: url(/images/product/simple_product_manager/breadcrumb/chevron_right.png);\n  background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mO4/B8AAqgB0yr7dJgAAAAASUVORK5CYII=);\n  background-image: url(data:application/font-woff;charset=utf-8;base64,ThisIsNormalBut/+0ThisIsLowerCased);\n  background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mO4/B8AAqgB0yr7dJgAAAAASUVORK5CYII=)\n    center center no-repeat;\n  background: url(data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20width=%229%22%20height=%229%22%3E%3Cpath%20d=%22M0%203h3%22%20stroke=%22red%22/%3E%3C/svg%3E);\n  offset-path: url(#path);\n  background: url(data/+0ThisShouldNotBeLowerCased);\n  background: url(https://foo/A*3I8oSY6AKRMAAAAAAAAAAABkARQnAQ);\n  background: url(https://example.com/some/quite,long,url,with,commas.jpg);\n  background: url(http://123.example.com);\n}\n@import url(https://fonts.googleapis.com/css?family=Roboto:100,300,400,500,700,900&display=swap);\n\n.validUnqotedUrlsThatAreParsedByLess {\n  background: url(@foo);\n}\n\n.validUnquotedUrlsThatAreParsedBySass {\n  background: url($foo);\n  background: url($foo+$bar);\n  background: url($foo*3);\n  background: url($foo/$bar);\n}\n\n.invalidUnquotedUrlsButWeParseThemAnyway {\n  background: url(--var(foo-bar, #dadce0));\n  -fb-sprite: url(fbglyph:cross-outline, fig-white);\n}\n\n.number {\n  background-image: url(http://123.com);\n  background: url(path/to/123.jpg);\n  background: url(#123.foo);\n  background: no-repeat center/80% url(http://123.com);\n  background: no-repeat center/80% url(path/to/123.jpg);\n  background: no-repeat center/80% url(#123.foo);\n}");
}
