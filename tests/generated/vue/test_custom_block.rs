#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_graphql_vue_embedded_language_formattingoff_format_1_accd92af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<page-query lang=\"graphql\">\nquery { posts: allWordPressPost {\n    edges {\n   node {\n          id\n      title\n      }\n    }\n  }\n}\n</page-query>") ? ;
    assert_eq ! (formatted , "<page-query lang=\"graphql\">\nquery { posts: allWordPressPost {\n    edges {\n   node {\n          id\n      title\n      }\n    }\n  }\n}\n</page-query>");
    Ok(())
}
#[test]
fn test_graphql_vue_semifalse_format_1_accd92af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<page-query lang=\"graphql\">\nquery { posts: allWordPressPost {\n    edges {\n   node {\n          id\n      title\n      }\n    }\n  }\n}\n</page-query>") ? ;
    assert_eq ! (formatted , "<page-query lang=\"graphql\">\nquery {\n  posts: allWordPressPost {\n    edges {\n      node {\n        id\n        title\n      }\n    }\n  }\n}\n</page-query>");
    Ok(())
}
#[test]
fn test_graphql_vue_trailing_commaes_5_format_1_accd92af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<page-query lang=\"graphql\">\nquery { posts: allWordPressPost {\n    edges {\n   node {\n          id\n      title\n      }\n    }\n  }\n}\n</page-query>") ? ;
    assert_eq ! (formatted , "<page-query lang=\"graphql\">\nquery {\n  posts: allWordPressPost {\n    edges {\n      node {\n        id\n        title\n      }\n    }\n  }\n}\n</page-query>");
    Ok(())
}
#[test]
fn test_graphql_vue_trailing_commanone_format_1_accd92af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<page-query lang=\"graphql\">\nquery { posts: allWordPressPost {\n    edges {\n   node {\n          id\n      title\n      }\n    }\n  }\n}\n</page-query>") ? ;
    assert_eq ! (formatted , "<page-query lang=\"graphql\">\nquery {\n  posts: allWordPressPost {\n    edges {\n      node {\n        id\n        title\n      }\n    }\n  }\n}\n</page-query>");
    Ok(())
}
#[test]
fn test_graphql_vue_vue_indent_script_and_styletrue_format_1_accd92af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<page-query lang=\"graphql\">\nquery { posts: allWordPressPost {\n    edges {\n   node {\n          id\n      title\n      }\n    }\n  }\n}\n</page-query>") ? ;
    assert_eq ! (formatted , "<page-query lang=\"graphql\">\nquery {\n  posts: allWordPressPost {\n    edges {\n      node {\n        id\n        title\n      }\n    }\n  }\n}\n</page-query>");
    Ok(())
}
#[test]
fn test_handlebars_vue_embedded_language_formattingoff_format_1_bc1d68ff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{     doesWhat}}</b>\n</custom>") ? ;
    assert_eq ! (formatted , "<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{     doesWhat}}</b>\n</custom>");
    Ok(())
}
#[test]
fn test_handlebars_vue_semifalse_format_1_bc1d68ff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{     doesWhat}}</b>\n</custom>") ? ;
    assert_eq!(
        formatted,
        "<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{doesWhat}}</b>\n</custom>"
    );
    Ok(())
}
#[test]
fn test_handlebars_vue_trailing_commaes_5_format_1_bc1d68ff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{     doesWhat}}</b>\n</custom>") ? ;
    assert_eq!(
        formatted,
        "<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{doesWhat}}</b>\n</custom>"
    );
    Ok(())
}
#[test]
fn test_handlebars_vue_trailing_commanone_format_1_bc1d68ff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{     doesWhat}}</b>\n</custom>") ? ;
    assert_eq!(
        formatted,
        "<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{doesWhat}}</b>\n</custom>"
    );
    Ok(())
}
#[test]
fn test_handlebars_vue_vue_indent_script_and_styletrue_format_1_bc1d68ff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{     doesWhat}}</b>\n</custom>") ? ;
    assert_eq!(
        formatted,
        "<custom type=\"text/x-handlebars-template\">\nHandlebars <b>{{doesWhat}}</b>\n</custom>"
    );
    Ok(())
}
#[test]
fn test_json_vue_embedded_language_formattingoff_format_1_dfcc90d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n    \n    \n    }, \"ja\": {\n   \n   \n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n\n\n    }, \"ja\": {\n\n\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}</custom>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}\n</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n    \n    \n    }, \"ja\": {\n   \n   \n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n\n\n    }, \"ja\": {\n\n\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}</custom>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}\n</custom>");
    Ok(())
}
#[test]
fn test_json_vue_semifalse_format_1_dfcc90d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n    \n    \n    }, \"ja\": {\n   \n   \n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n\n\n    }, \"ja\": {\n\n\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}</custom>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}\n</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"json\">\n{\n  \"en\": {\n    \"hello\": \"hello world!\"\n  },\n  \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{\n  \"en\": {\n    \"hello\": \"hello world!\"\n  },\n  \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">\n{\n  \"a\": 1\n}\n</custom>\n\n<custom lang=\"json\">\n{\n  \"a\": 1\n}\n</custom>");
    Ok(())
}
#[test]
fn test_json_vue_trailing_commaes_5_format_1_dfcc90d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n    \n    \n    }, \"ja\": {\n   \n   \n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n\n\n    }, \"ja\": {\n\n\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}</custom>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}\n</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"json\">\n{\n  \"en\": {\n    \"hello\": \"hello world!\"\n  },\n  \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{\n  \"en\": {\n    \"hello\": \"hello world!\"\n  },\n  \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">\n{\n  \"a\": 1\n}\n</custom>\n\n<custom lang=\"json\">\n{\n  \"a\": 1\n}\n</custom>");
    Ok(())
}
#[test]
fn test_json_vue_trailing_commanone_format_1_dfcc90d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n    \n    \n    }, \"ja\": {\n   \n   \n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n\n\n    }, \"ja\": {\n\n\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}</custom>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}\n</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"json\">\n{\n  \"en\": {\n    \"hello\": \"hello world!\"\n  },\n  \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{\n  \"en\": {\n    \"hello\": \"hello world!\"\n  },\n  \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">\n{\n  \"a\": 1\n}\n</custom>\n\n<custom lang=\"json\">\n{\n  \"a\": 1\n}\n</custom>");
    Ok(())
}
#[test]
fn test_json_vue_vue_indent_script_and_styletrue_format_1_dfcc90d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n    \n    \n    }, \"ja\": {\n   \n   \n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{ \"en\": {\n    \"hello\": \"hello world!\"\n\n\n    }, \"ja\": {\n\n\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}</custom>\n\n<custom lang=\"json\">{\n  \"a\": 1\n}\n</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"json\">\n{\n  \"en\": {\n    \"hello\": \"hello world!\"\n  },\n  \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<i18n type=\"application/json\">\n{\n  \"en\": {\n    \"hello\": \"hello world!\"\n  },\n  \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom lang=\"json\">\n{\n  \"a\": 1\n}\n</custom>\n\n<custom lang=\"json\">\n{\n  \"a\": 1\n}\n</custom>");
    Ok(())
}
#[test]
fn test_lang_attribute_vue_embedded_language_formattingoff_format_1_04eed247() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"js\">\nconst foo\n      = 'foo'\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"js\">\nconst foo\n      = 'foo'\n</custom>"
    );
    Ok(())
}
#[test]
fn test_lang_attribute_vue_semifalse_format_1_04eed247() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"js\">\nconst foo\n      = 'foo'\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"js\">\nconst foo = \"foo\"\n</custom>"
    );
    Ok(())
}
#[test]
fn test_lang_attribute_vue_trailing_commaes_5_format_1_04eed247() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"js\">\nconst foo\n      = 'foo'\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"js\">\nconst foo = \"foo\";\n</custom>"
    );
    Ok(())
}
#[test]
fn test_lang_attribute_vue_trailing_commanone_format_1_04eed247() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"js\">\nconst foo\n      = 'foo'\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"js\">\nconst foo = \"foo\";\n</custom>"
    );
    Ok(())
}
#[test]
fn test_lang_attribute_vue_vue_indent_script_and_styletrue_format_1_04eed247() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"js\">\nconst foo\n      = 'foo'\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"js\">\nconst foo = \"foo\";\n</custom>"
    );
    Ok(())
}
#[test]
fn test_markdown_vue_embedded_language_formattingoff_format_1_08d17e29() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs lang=\"markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>") ? ;
    assert_eq ! (formatted , "<docs lang=\"markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>");
    Ok(())
}
#[test]
fn test_markdown_vue_semifalse_format_1_08d17e29() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs lang=\"markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>") ? ;
    assert_eq ! (formatted , "<docs lang=\"markdown\">\n# Foo\n\n- bar\n- baz\n\n| Age          | Time  | Food | Gold | Requirement             |\n| ------------ | ----- | ---- | ---- | ----------------------- |\n| Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n| Castle Age   | 02:40 | 800  | 200  | -                       |\n| Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n\n- bar\n- baz\n\n| Age          | Time  | Food | Gold | Requirement             |\n| ------------ | ----- | ---- | ---- | ----------------------- |\n| Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n| Castle Age   | 02:40 | 800  | 200  | -                       |\n| Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n</docs>");
    Ok(())
}
#[test]
fn test_markdown_vue_trailing_commaes_5_format_1_08d17e29() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs lang=\"markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>") ? ;
    assert_eq ! (formatted , "<docs lang=\"markdown\">\n# Foo\n\n- bar\n- baz\n\n| Age          | Time  | Food | Gold | Requirement             |\n| ------------ | ----- | ---- | ---- | ----------------------- |\n| Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n| Castle Age   | 02:40 | 800  | 200  | -                       |\n| Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n\n- bar\n- baz\n\n| Age          | Time  | Food | Gold | Requirement             |\n| ------------ | ----- | ---- | ---- | ----------------------- |\n| Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n| Castle Age   | 02:40 | 800  | 200  | -                       |\n| Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n</docs>");
    Ok(())
}
#[test]
fn test_markdown_vue_trailing_commanone_format_1_08d17e29() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs lang=\"markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>") ? ;
    assert_eq ! (formatted , "<docs lang=\"markdown\">\n# Foo\n\n- bar\n- baz\n\n| Age          | Time  | Food | Gold | Requirement             |\n| ------------ | ----- | ---- | ---- | ----------------------- |\n| Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n| Castle Age   | 02:40 | 800  | 200  | -                       |\n| Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n\n- bar\n- baz\n\n| Age          | Time  | Food | Gold | Requirement             |\n| ------------ | ----- | ---- | ---- | ----------------------- |\n| Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n| Castle Age   | 02:40 | 800  | 200  | -                       |\n| Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n</docs>");
    Ok(())
}
#[test]
fn test_markdown_vue_vue_indent_script_and_styletrue_format_1_08d17e29() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs lang=\"markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n- bar\n- baz\n\n| Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n</docs>") ? ;
    assert_eq ! (formatted , "<docs lang=\"markdown\">\n# Foo\n\n- bar\n- baz\n\n| Age          | Time  | Food | Gold | Requirement             |\n| ------------ | ----- | ---- | ---- | ----------------------- |\n| Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n| Castle Age   | 02:40 | 800  | 200  | -                       |\n| Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n</docs>\n\n<docs type=\"text/markdown\">\n# Foo\n\n- bar\n- baz\n\n| Age          | Time  | Food | Gold | Requirement             |\n| ------------ | ----- | ---- | ---- | ----------------------- |\n| Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n| Castle Age   | 02:40 | 800  | 200  | -                       |\n| Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n</docs>");
    Ok(())
}
#[test]
fn test_tag_like_vue_embedded_language_formattingoff_format_1_422e4173() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo =            \"</\"\n</custom>") ? ;
    assert_eq ! (formatted , "<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo =            \"</\"\n</custom>");
    Ok(())
}
#[test]
fn test_tag_like_vue_semifalse_format_1_422e4173() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo =            \"</\"\n</custom>") ? ;
    assert_eq ! (formatted , "<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo = \"</\"\n</custom>");
    Ok(())
}
#[test]
fn test_tag_like_vue_trailing_commaes_5_format_1_422e4173() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo =            \"</\"\n</custom>") ? ;
    assert_eq ! (formatted , "<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo = \"</\";\n</custom>");
    Ok(())
}
#[test]
fn test_tag_like_vue_trailing_commanone_format_1_422e4173() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo =            \"</\"\n</custom>") ? ;
    assert_eq ! (formatted , "<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo = \"</\";\n</custom>");
    Ok(())
}
#[test]
fn test_tag_like_vue_vue_indent_script_and_styletrue_format_1_422e4173() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo =            \"</\"\n</custom>") ? ;
    assert_eq ! (formatted , "<docs>\nThis component takes an \\`items\\` prop (\\`Array<Object>\\`).\n  or\nThis component should be placed inside a \\`<my-component>\\`.\n</docs>\n\n<custom lang=\"javascript\">\nconst foo = \"</\";\n</custom>");
    Ok(())
}
#[test]
fn test_unknown_vue_embedded_language_formattingoff_format_1_aaada124() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>");
    Ok(())
}
#[test]
fn test_unknown_vue_semifalse_format_1_aaada124() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>");
    Ok(())
}
#[test]
fn test_unknown_vue_trailing_commaes_5_format_1_aaada124() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>");
    Ok(())
}
#[test]
fn test_unknown_vue_trailing_commanone_format_1_aaada124() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>");
    Ok(())
}
#[test]
fn test_unknown_vue_vue_indent_script_and_styletrue_format_1_aaada124() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"foooo\">\n{\"en\": {\n    \"hello\": \"hello world!\"\n  }, \"ja\": {\n    \"hello\": \"こんにちは、世界！\"\n  }\n}\n</i18n>\n\n<custom>\nconst foo =\n\n\n     \"foo\";\n</custom>\n\n<custom lang=\"zzz\">\nconst foo = \"foo\";\n  const foo = \"foo\";</custom>\n\n<custom lang=\"zzz\">123</custom>\n\n<custom>{\n     foo: \"bar\"\n}</custom>");
    Ok(())
}
#[test]
fn test_yaml_vue_embedded_language_formattingoff_format_1_0f1150eb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"yaml\">\nen:\n                       hello: \"hello world!\"\nja:\n        hello: \"こんにちは、世界！\"\n</i18n>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"yaml\">\nen:\n                       hello: \"hello world!\"\nja:\n        hello: \"こんにちは、世界！\"\n</i18n>");
    Ok(())
}
#[test]
fn test_yaml_vue_semifalse_format_1_0f1150eb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"yaml\">\nen:\n                       hello: \"hello world!\"\nja:\n        hello: \"こんにちは、世界！\"\n</i18n>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"yaml\">\nen:\n  hello: \"hello world!\"\nja:\n  hello: \"こんにちは、世界！\"\n</i18n>");
    Ok(())
}
#[test]
fn test_yaml_vue_trailing_commaes_5_format_1_0f1150eb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"yaml\">\nen:\n                       hello: \"hello world!\"\nja:\n        hello: \"こんにちは、世界！\"\n</i18n>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"yaml\">\nen:\n  hello: \"hello world!\"\nja:\n  hello: \"こんにちは、世界！\"\n</i18n>");
    Ok(())
}
#[test]
fn test_yaml_vue_trailing_commanone_format_1_0f1150eb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"yaml\">\nen:\n                       hello: \"hello world!\"\nja:\n        hello: \"こんにちは、世界！\"\n</i18n>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"yaml\">\nen:\n  hello: \"hello world!\"\nja:\n  hello: \"こんにちは、世界！\"\n</i18n>");
    Ok(())
}
#[test]
fn test_yaml_vue_vue_indent_script_and_styletrue_format_1_0f1150eb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .vue_indent_script_and_style(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n lang=\"yaml\">\nen:\n                       hello: \"hello world!\"\nja:\n        hello: \"こんにちは、世界！\"\n</i18n>") ? ;
    assert_eq ! (formatted , "<i18n lang=\"yaml\">\nen:\n  hello: \"hello world!\"\nja:\n  hello: \"こんにちは、世界！\"\n</i18n>");
    Ok(())
}
