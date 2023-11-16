#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_5d1cb46c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const comp1 = (\n  <div style={styles} key=\"something\">\n    Keep the wrapping parens.\n  </div>\n);\n\nconst comp2 = <div style={styles} key=\"something\">\n  Create wrapping parens.\n</div>;\n\ncomp2A = <div style={styles} key=\"something\">\n  Create wrapping parens.\n</div>;\n\nconst comp3 = <div style={styles} key=\"something\">Bump to next line without parens</div>;\n\nconst comp4 = <div style={styles} key=\"something\">Create wrapping parens and indent <strong>all the things</strong>.</div>;\n\nconst comp5 = <div>Keep it on one line.</div>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const comp1 = (\n  <div style={styles} key=\"something\">\n    Keep the wrapping parens.\n  </div>\n);\n\nconst comp2 = (\n  <div style={styles} key=\"something\">\n    Create wrapping parens.\n  </div>\n);\n\ncomp2A = (\n  <div style={styles} key=\"something\">\n    Create wrapping parens.\n  </div>\n);\n\nconst comp3 = (\n  <div style={styles} key=\"something\">\n    Bump to next line without parens\n  </div>\n);\n\nconst comp4 = (\n  <div style={styles} key=\"something\">\n    Create wrapping parens and indent <strong>all the things</strong>.\n  </div>\n);\n\nconst comp5 = <div>Keep it on one line.</div>;");
}
