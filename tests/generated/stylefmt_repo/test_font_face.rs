#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_font_face_css_format_1_3edfb8f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (" @font-face          {font-family:'HelveticaNeueW02-45Ligh';src:url(\"/fonts/pictos-web.eot\");src:local(\"☺\"),url(\"/fonts/pictos-web.woff\") format(\"woff\"),url(\"/fonts/pictos-web.ttf\") format(\"truetype\"),url(\"/fonts/pictos-web.svg#webfontIyfZbseF\") format(\"svg\");font-weight:normal;font-style:normal;}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@font-face {\n  font-family: \"HelveticaNeueW02-45Ligh\";\n  src: url(\"/fonts/pictos-web.eot\");\n  src:\n    local(\"☺\"),\n    url(\"/fonts/pictos-web.woff\") format(\"woff\"),\n    url(\"/fonts/pictos-web.ttf\") format(\"truetype\"),\n    url(\"/fonts/pictos-web.svg#webfontIyfZbseF\") format(\"svg\");\n  font-weight: normal;\n  font-style: normal;\n}");
}
