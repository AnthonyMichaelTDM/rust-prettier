#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_0_indent_js_prose_wrapalways_format_1_8aab4309() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("md\\`\nThis line shouldn't be indented at all in the resulting output.\n\\`\n\nif (true) {\n  md\\`\ntext1\n- 123\n  - 456\n\ntext2\n- 123\n  - 456\n\ntext3\n- 123\n  - 456\n\\`;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "md\\`\nThis line shouldn't be indented at all in the resulting output.\n\\`;\n\nif (true) {\n  md\\`\ntext1\n\n- 123\n  - 456\n\ntext2\n\n- 123\n  - 456\n\ntext3\n\n- 123\n  - 456\n  \\`;\n}");
}
#[test]
fn test_codeblock_js_prose_wrapalways_format_1_7207cd2f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("md\\`\n\\\\\\`\\\\\\`\\\\\\`js\nmarkdown\\\\\\`\n    \\\\\\\\\\\\\\`\\\\\\\\\\\\\\`\\\\\\\\\\\\\\`js\n    console.log('hi');\n    \\\\\\\\\\\\\\`\\\\\\\\\\\\\\`\\\\\\\\\\\\\\`\n\\\\\\`\n\\\\\\`\\\\\\`\\\\\\`\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "md\\`\n~~~js\nmarkdown\\\\\\`\n  ~~~js\n  console.log(\"hi\");\n  ~~~\n\\\\\\`;\n~~~\n\\`;");
}
#[test]
fn test_escape_js_prose_wrapalways_format_1_1822a523() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("markdown\\`\n  const cssString = css\\\\\\`\n    background-color: \\\\$\\\\{color('base')\\\\}\n  \\\\\\`;\n\\`\n\nmarkdown\\`\n  - \\\\\\`\n  - \\\\\\\\\\\\\\`\n  - \\\\\\\\ a\n  - \\\\\\\\\\\\\\\\\n  - \\\\$\n  - \\\\u1234\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "markdown\\`\n  const cssString = css\\\\\\`  background-color: \\\\$\\\\{color('base')\\\\}\\\\\\`;\n\\`;\n\nmarkdown\\`\n  - \\\\\\`\n  - \\\\\\\\\\\\\\`\n  - \\\\\\\\ a\n  - \\\\\\\\\\\\\\\\\n  - \\\\$\n  - \\\\u1234\n\\`;");
}
#[test]
fn test_issue_5021_js_prose_wrapalways_format_1_80bd1de9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (true) {\n  md\\`\ntext1\n- 123\n  - 456\n\ntext2\n- 123\n  - 456\n\ntext3\n- 123\n  - 456\n\\`;\n}\n\nif (true) {\n  md\\`\ntext1\n\n- 123\n  - 456\n\ntext2\n\n- 123\n  - 456\n\ntext3\n\n- 123\n  - 456\n  \\`;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (true) {\n  md\\`\ntext1\n\n- 123\n  - 456\n\ntext2\n\n- 123\n  - 456\n\ntext3\n\n- 123\n  - 456\n  \\`;\n}\n\nif (true) {\n  md\\`\ntext1\n\n- 123\n  - 456\n\ntext2\n\n- 123\n  - 456\n\ntext3\n\n- 123\n  - 456\n  \\`;\n}");
}
#[test]
fn test_markdown_js_prose_wrapalways_format_1_c81db771() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export default function ReadMe() {\n    return md\\`\n        ## Why Prettier?\n        \n        ### Building and enforcing a style guide\n        \n        By far the biggest reason for adopting Prettier is to stop all the on-going debates over styles. It is generally accepted that having a common style guide is valuable for a project and team but getting there is a very painful and unrewarding process. People get very emotional around particular ways of writing code and nobody likes spending time writing and receiving nits.\n        - “We want to free mental threads and end discussions around style. While sometimes fruitful, these discussions are for the most part wasteful.”\n        - “Literally had an engineer go through a huge effort of cleaning up all of our code because we were debating ternary style for the longest time and were inconsistent about it. It was dumb, but it was a weird on-going \"great debate\" that wasted lots of little back and forth bits. It's far easier for us all to agree now: just run Prettier, and go with that style.”\n        - “Getting tired telling people how to style their product code.”\n        - “Our top reason was to stop wasting our time debating style nits.”\n        - “Having a githook set up has reduced the amount of style issues in PRs that result in broken builds due to ESLint rules or things I have to nit-pick or clean up later.”\n        - “I don't want anybody to nitpick any other person ever again.”\n        - “It reminds me of how Steve Jobs used to wear the same clothes every day because he has a million decisions to make and he didn't want to be bothered to make trivial ones like picking out clothes. I think Prettier is like that.”\n    \\`;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export default function ReadMe() {\n  return md\\`\n    ## Why Prettier?\n\n    ### Building and enforcing a style guide\n\n    By far the biggest reason for adopting Prettier is to stop all the on-going\n    debates over styles. It is generally accepted that having a common style\n    guide is valuable for a project and team but getting there is a very painful\n    and unrewarding process. People get very emotional around particular ways of\n    writing code and nobody likes spending time writing and receiving nits.\n\n    - “We want to free mental threads and end discussions around style. While\n      sometimes fruitful, these discussions are for the most part wasteful.”\n    - “Literally had an engineer go through a huge effort of cleaning up all of\n      our code because we were debating ternary style for the longest time and\n      were inconsistent about it. It was dumb, but it was a weird on-going\n      \"great debate\" that wasted lots of little back and forth bits. It's far\n      easier for us all to agree now: just run Prettier, and go with that\n      style.”\n    - “Getting tired telling people how to style their product code.”\n    - “Our top reason was to stop wasting our time debating style nits.”\n    - “Having a githook set up has reduced the amount of style issues in PRs\n      that result in broken builds due to ESLint rules or things I have to\n      nit-pick or clean up later.”\n    - “I don't want anybody to nitpick any other person ever again.”\n    - “It reminds me of how Steve Jobs used to wear the same clothes every day\n      because he has a million decisions to make and he didn't want to be\n      bothered to make trivial ones like picking out clothes. I think Prettier\n      is like that.”\n  \\`;\n}");
}
#[test]
fn test_single_line_js_prose_wrapalways_format_1_0a8b2df4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("markdown\\`# hello\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "markdown\\`\n# hello\n\\`;");
}
