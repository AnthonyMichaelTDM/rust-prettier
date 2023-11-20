#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_9734_md_trailing_commaall_format_1_ebdc8df0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Markdown\n\n\\`\\`\\`js\n\"test\"\n\\`\\`\\`\n\n\\`\\`\\`js\n'test'\n\\`\\`\\`\n\n\\`\\`\\`js\n\\`test\\`\n\\`\\`\\`\n\n\\`\\`\\`js\nfalse\n\\`\\`\\`\n\n\\`\\`\\`js\ntrue\n\\`\\`\\`\n\n\\`\\`\\`js\nnull\n\\`\\`\\`\n\n\\`\\`\\`js\n[]\n\\`\\`\\`\n\n\\`\\`\\`js\n{}\n\\`\\`\\`\n\n\\`\\`\\`js\n5\n\\`\\`\\`\n\n\\`\\`\\`js\nInfinity\n\\`\\`\\`\n\n\\`\\`\\`js\nNaN\n\\`\\`\\`\n\n\\`\\`\\`js\nundefined\n\\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Markdown\n\n\\`\\`\\`js\n\"test\";\n\\`\\`\\`\n\n\\`\\`\\`js\n\"test\";\n\\`\\`\\`\n\n\\`\\`\\`js\n\\`test\\`;\n\\`\\`\\`\n\n\\`\\`\\`js\nfalse;\n\\`\\`\\`\n\n\\`\\`\\`js\ntrue;\n\\`\\`\\`\n\n\\`\\`\\`js\nnull;\n\\`\\`\\`\n\n\\`\\`\\`js\n[];\n\\`\\`\\`\n\n\\`\\`\\`js\n{\n}\n\\`\\`\\`\n\n\\`\\`\\`js\n5;\n\\`\\`\\`\n\n\\`\\`\\`js\nInfinity;\n\\`\\`\\`\n\n\\`\\`\\`js\nNaN;\n\\`\\`\\`\n\n\\`\\`\\`js\nundefined;\n\\`\\`\\`");
}
#[test]
fn test_jsx_comment_md_trailing_commaall_format_1_b309925f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`\\`\\`jsx\nconst Foo = () => {\n  return (\n    <div>\n      {/*\n        This links to a page that does not yet exist.\n      */}    \n      <hr />\n    </div>\n  );\n};\n\\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`\\`\\`jsx\nconst Foo = () => {\n  return (\n    <div>\n      {/*\n        This links to a page that does not yet exist.\n      */}\n      <hr />\n    </div>\n  );\n};\n\\`\\`\\`");
}
#[test]
fn test_meta_in_code_block_md_trailing_commaall_format_1_a13a5346() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("## plain js block\n\n\\`\\`\\`js    \nconsole.log    (    \"hello world\"    );\n\\`\\`\\`\n\n## js block with meta\n\n\\`\\`\\`js {cmd=node .line-numbers}\nconsole.log    (    \"hello world\"    );\n\\`\\`\\`\n\n## js block with meta but no space (the language should not be detected)\n\n\\`\\`\\`js{cmd=node .line-numbers}\nconsole.log    (    \"hello world\"    );\n\\`\\`\\`\n\n## js block with meta and extra spaces (only the first set of spaces should be changed)\n\n\\`\\`\\`js    cmd=node    something=\"a    b\"\nconsole.log    (    \"hello world\"    );\n\\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "## plain js block\n\n\\`\\`\\`js\nconsole.log(\"hello world\");\n\\`\\`\\`\n\n## js block with meta\n\n\\`\\`\\`js {cmd=node .line-numbers}\nconsole.log(\"hello world\");\n\\`\\`\\`\n\n## js block with meta but no space (the language should not be detected)\n\n\\`\\`\\`js{cmd=node .line-numbers}\nconsole.log    (    \"hello world\"    );\n\\`\\`\\`\n\n## js block with meta and extra spaces (only the first set of spaces should be changed)\n\n\\`\\`\\`js cmd=node    something=\"a    b\"\nconsole.log(\"hello world\");\n\\`\\`\\`");
}
#[test]
fn test_trailing_comma_md_trailing_commaall_format_1_e188516a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("### Some heading\n\n\\`\\`\\`js\nsomeFunctionCall(\n  foo,\n  bar,\n  foobar,\n  sometehingReallyLongAndHairy,\n  somethingElse,\n  breakNow,\n);\n\\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "### Some heading\n\n\\`\\`\\`js\nsomeFunctionCall(\n  foo,\n  bar,\n  foobar,\n  sometehingReallyLongAndHairy,\n  somethingElse,\n  breakNow,\n);\n\\`\\`\\`");
}
