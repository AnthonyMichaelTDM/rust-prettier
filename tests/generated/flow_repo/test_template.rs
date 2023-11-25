use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_template_js_format_1_3f6f0778() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(\\`foo\\`: string); // ok\n(\\`bar\\`: 'bar'); // ok\n(\\`baz\\`: number); // error\n\n\\`foo \\${123} bar\\`; // ok, number can be appended to string\n\\`foo \\${{bar: 123}} baz\\`; // error, object can't be appended\n\nlet tests = [\n  function(x: string) {\n    \\`foo \\${x}\\`; // ok\n    \\`\\${x} bar\\`; // ok\n    \\`foo \\${'bar'} \\${x}\\`; // ok\n  },\n  function(x: number) {\n    \\`foo \\${x}\\`; // ok\n    \\`\\${x} bar\\`; // ok\n    \\`foo \\${'bar'} \\${x}\\`; // ok\n  },\n  function(x: boolean) {\n    \\`foo \\${x}\\`; // error\n    \\`\\${x} bar\\`; // error\n    \\`foo \\${'bar'} \\${x}\\`; // error\n  },\n  function(x: mixed) {\n    \\`foo \\${x}\\`; // error\n    \\`\\${x} bar\\`; // error\n    \\`foo \\${'bar'} \\${x}\\`; // error\n  },\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n(\\`foo\\`: string); // ok\n(\\`bar\\`: \"bar\"); // ok\n(\\`baz\\`: number); // error\n\n\\`foo \\${123} bar\\`; // ok, number can be appended to string\n\\`foo \\${{ bar: 123 }} baz\\`; // error, object can't be appended\n\nlet tests = [\n  function (x: string) {\n    \\`foo \\${x}\\`; // ok\n    \\`\\${x} bar\\`; // ok\n    \\`foo \\${\"bar\"} \\${x}\\`; // ok\n  },\n  function (x: number) {\n    \\`foo \\${x}\\`; // ok\n    \\`\\${x} bar\\`; // ok\n    \\`foo \\${\"bar\"} \\${x}\\`; // ok\n  },\n  function (x: boolean) {\n    \\`foo \\${x}\\`; // error\n    \\`\\${x} bar\\`; // error\n    \\`foo \\${\"bar\"} \\${x}\\`; // error\n  },\n  function (x: mixed) {\n    \\`foo \\${x}\\`; // error\n    \\`\\${x} bar\\`; // error\n    \\`foo \\${\"bar\"} \\${x}\\`; // error\n  },\n];");
}
