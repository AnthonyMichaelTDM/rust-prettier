#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_boundary_js_format_1_6d11aa18() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`\\${\na +  // a\n  a\n}\n\n\\${a // comment\n}\n\n\\${b /* comment */}\n\n\\${/* comment */ c /* comment */}\n\n\\${// comment\nd //comment\n}\n\n\\${// $FlowFixMe found when converting React.createClass to ES6\nExampleStory.getFragment('story')}\n\\`;\n\n<div>\n{ExampleStory.getFragment('story') // $FlowFixMe found when converting React.createClass to ES6\n}\n</div>;") ? ;
    assert_eq ! (formatted , "\\`\\${\n  a + // a\n  a\n}\n\n\\${\n  a // comment\n}\n\n\\${b /* comment */}\n\n\\${/* comment */ c /* comment */}\n\n\\${\n  // comment\n  d //comment\n}\n\n\\${\n  // $FlowFixMe found when converting React.createClass to ES6\n  ExampleStory.getFragment(\"story\")\n}\n\\`;\n\n<div>\n  {\n    ExampleStory.getFragment(\"story\") // $FlowFixMe found when converting React.createClass to ES6\n  }\n</div>;");
    Ok(())
}
