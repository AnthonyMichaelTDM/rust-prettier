#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_9260_mdx_embedded_language_formattingoff_format_1_dc8de643() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# title\n\n<Parenthesis>\n\nCR: Carriage Return, \\\\r\nLF: Line Feed, \\\\n\n\n</Parenthesis>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# title\n\n<Parenthesis>\n\nCR: Carriage Return, \\\\r\nLF: Line Feed, \\\\n\n\n</Parenthesis>");
}
#[test]
fn test_multiline_comments_mdx_embedded_language_formattingoff_format_1_e389f99b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# title\n\n{/* prettier-ignore-start */}\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n{/* prettier-ignore-end */}\n\n{/* prettier-ignore-start */}\n\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n{/* prettier-ignore-end */}\n\n{/* prettier-ignore-start */}\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n\n{/* prettier-ignore-end */}\n\n{/* prettier-ignore-start */}\n\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n\n{/* prettier-ignore-end */}\n\n<!-- prettier-ignore-start -->\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n<!-- prettier-ignore-end -->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# title\n\n{/* prettier-ignore-start */}\nexport const Test = () => {\nreturn <p>test</p>;\n};\n\n   <Test />\n{/* prettier-ignore-end */}\n\n{/* prettier-ignore-start */}\n\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n{/* prettier-ignore-end */}\n\n{/* prettier-ignore-start */}\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n\n{/* prettier-ignore-end */}\n\n{/* prettier-ignore-start */}\n\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n\n{/* prettier-ignore-end */}\n\n<!-- prettier-ignore-start -->\n\nexport const Test = () => {\n   return <p>test</p>;\n };\n   <Test />\n<!-- prettier-ignore-end -->");
}
#[test]
fn test_pull_11563_mdx_embedded_language_formattingoff_format_1_a8d5a200() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# title\n\n{ /* Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. */ }\n\n{/* Some more. */}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "# title\n\n{/* Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. */}\n\n{/* Some more. */}");
}
