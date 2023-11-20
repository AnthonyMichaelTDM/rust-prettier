#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_inside_template_vue_vue_indent_script_and_stylefalse_format_1_e6d00990() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .vue_indent_script_and_style(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n    <style>\n        br {\n            background: #00abc9;\n        }\n\n    div {\n        background: #00abc9;\n    }\n    </style>\n\n    <script>\n        console.log('hello');\n    console.log('hello');\n    </script>\n\n    <br />\n    <footer>\n        foo\n        <br/>\n    </footer>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <style>\n    br {\n      background: #00abc9;\n    }\n\n    div {\n      background: #00abc9;\n    }\n  </style>\n\n  <script>\n    console.log(\"hello\");\n    console.log(\"hello\");\n  </script>\n\n  <br />\n  <footer>\n    foo\n    <br />\n  </footer>\n</template>");
}
#[test]
fn test_inside_template_vue_vue_indent_script_and_styletrue_format_1_e6d00990() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n    <style>\n        br {\n            background: #00abc9;\n        }\n\n    div {\n        background: #00abc9;\n    }\n    </style>\n\n    <script>\n        console.log('hello');\n    console.log('hello');\n    </script>\n\n    <br />\n    <footer>\n        foo\n        <br/>\n    </footer>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <style>\n    br {\n      background: #00abc9;\n    }\n\n    div {\n      background: #00abc9;\n    }\n  </style>\n\n  <script>\n    console.log(\"hello\");\n    console.log(\"hello\");\n  </script>\n\n  <br />\n  <footer>\n    foo\n    <br />\n  </footer>\n</template>");
}
#[test]
fn test_vue_tag_indent_vue_vue_indent_script_and_stylefalse_format_1_299ab9e0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .vue_indent_script_and_style(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\n  console.log('hello');\nconsole.log('hello');\n</script>\n\n<style lang=\"scss\">\n  footer {\n    background: #00abc9;\n  }\n\nfooter {\n  background: #00abc9;\n}\n</style>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br/>\n  </footer>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script>\nconsole.log(\"hello\");\nconsole.log(\"hello\");\n</script>\n\n<style lang=\"scss\">\nfooter {\n  background: #00abc9;\n}\n\nfooter {\n  background: #00abc9;\n}\n</style>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br />\n  </footer>\n</template>");
}
#[test]
fn test_vue_tag_indent_vue_vue_indent_script_and_styletrue_format_1_299ab9e0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .vue_indent_script_and_style(true)
        .parsers(vec!["vue"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\n  console.log('hello');\nconsole.log('hello');\n</script>\n\n<style lang=\"scss\">\n  footer {\n    background: #00abc9;\n  }\n\nfooter {\n  background: #00abc9;\n}\n</style>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br/>\n  </footer>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script>\n  console.log(\"hello\");\n  console.log(\"hello\");\n</script>\n\n<style lang=\"scss\">\n  footer {\n    background: #00abc9;\n  }\n\n  footer {\n    background: #00abc9;\n  }\n</style>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br />\n  </footer>\n</template>");
}
