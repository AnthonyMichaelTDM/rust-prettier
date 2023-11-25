#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_svg_html_format_1_9e3dfa16() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <head>\n    <title>SVG</title>\n  </head>\n  <body>\n    <svg width=\"100\" height=\"100\">\n      <circle cx=\"50\" cy=\"50\" r=\"40\" stroke=\"green\" stroke-width=\"4\" fill=\"yellow\" />\n    </svg>\n  </body>\n</html>\n\n<svg viewBox=\"0 0 200 200\" xmlns=\"http://www.w3.org/2000/svg\">\n<defs /> \n\n <g>\n  <g><polygon points=\"5,5 195,10 185,185 10,195\" />\n      <text>    Text</text></g>\n  </g>\n\n  <!-- Common use case: embed HTML text into SVG -->\n  <foreignObject x=\"20\" y=\"20\" width=\"160\" height=\"160\">\n    <!--\n      In the context of SVG embeded into HTML, the XHTML namespace could be avoided, but it is mandatory in the context of an SVG document\n    -->\n    <div xmlns=\"http://www.w3.org/1999/xhtml\">\n    <p>\n  123\n      </p>\n      <span>\n        123\n        </span>\n    </div>\n  </foreignObject>\n</svg>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <head>\n    <title>SVG</title>\n  </head>\n  <body>\n    <svg width=\"100\" height=\"100\">\n      <circle\n        cx=\"50\"\n        cy=\"50\"\n        r=\"40\"\n        stroke=\"green\"\n        stroke-width=\"4\"\n        fill=\"yellow\"\n      />\n    </svg>\n  </body>\n</html>\n\n<svg viewBox=\"0 0 200 200\" xmlns=\"http://www.w3.org/2000/svg\">\n  <defs />\n\n  <g>\n    <g>\n      <polygon points=\"5,5 195,10 185,185 10,195\" />\n      <text>Text</text>\n    </g>\n  </g>\n\n  <!-- Common use case: embed HTML text into SVG -->\n  <foreignObject x=\"20\" y=\"20\" width=\"160\" height=\"160\">\n    <!--\n      In the context of SVG embeded into HTML, the XHTML namespace could be avoided, but it is mandatory in the context of an SVG document\n    -->\n    <div xmlns=\"http://www.w3.org/1999/xhtml\">\n      <p>123</p>\n      <span> 123 </span>\n    </div>\n  </foreignObject>\n</svg>");
}
