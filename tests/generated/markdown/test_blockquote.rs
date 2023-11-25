#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_code_md_prose_wrapalways_format_1_b57902b3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> NOTE: To use \\`unobtrusive\\`, \\`unobtrusive/import\\`, \\`unobtrusive/react\\`, and \\`unobtrusive/flowtype\\` together, your eslint config would look like this:\n>\\`\\`\\`json\n>{\n>  \"extends\": [\n>    \"unobtrusive\",\n>    \"unobtrusive/import\",\n>    \"unobtrusive/react\",\n>    \"unobtrusive/flowtype\"\n>  ],\n>  \"env\": {\n>    \"browser\": true\n>  }\n>}\n>\\`\\`\\`") ? ;
    assert_eq ! (formatted , "> NOTE: To use \\`unobtrusive\\`, \\`unobtrusive/import\\`, \\`unobtrusive/react\\`, and\n> \\`unobtrusive/flowtype\\` together, your eslint config would look like this:\n>\n> \\`\\`\\`json\n> {\n>   \"extends\": [\n>     \"unobtrusive\",\n>     \"unobtrusive/import\",\n>     \"unobtrusive/react\",\n>     \"unobtrusive/flowtype\"\n>   ],\n>   \"env\": {\n>     \"browser\": true\n>   }\n> }\n> \\`\\`\\`");
    Ok(())
}
#[test]
fn test_ignore_code_md_prose_wrapalways_format_1_41b9dcf9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> \\`\\`\\`\\`md\n> <!-- prettier-ignore -->\n> \\`\\`\\`js\n> ugly   ( code ) ;\n> \\`\\`\\`\n> \\`\\`\\`\\`\n\n> \\`\\`\\`md\n> <!-- prettier-ignore -->\n> - This is a long long\n>   long long long long\n>   long long paragraph.\n> \\`\\`\\`\n\n> - test\n>   \\`\\`\\`md\n>   <!-- prettier-ignore -->\n>   - This is a long long\n>     long long long long\n>     long long paragraph.\n>   \\`\\`\\`\n\n\\`\\`\\`\\`md\n> \\`\\`\\`md\n> <!-- prettier-ignore -->\n> - This is a long long\n>   long long long long\n>   long long paragraph.\n> \\`\\`\\`\n\\`\\`\\`\\`\n\n> \\`\\`\\`\\`md\n> > \\`\\`\\`md\n> > <!-- prettier-ignore -->\n> > - This is a long long\n> >   long long long long\n> >   long long paragraph.\n> > \\`\\`\\`\n> \\`\\`\\`\\`\n\n> \n> <!-- prettier-ignore -->\n> - This is a long long\n>   long long long long\n>   long long paragraph.\n> \n\n> \\`\\`\\`\\`js\n> // prettier-ignore\n> const x = 1,\n> b = 2\n> \\`\\`\\`\\`") ? ;
    assert_eq ! (formatted , "> \\`\\`\\`\\`md\n> <!-- prettier-ignore -->\n> \\`\\`\\`js\n> ugly   ( code ) ;\n> \\`\\`\\`\n> \\`\\`\\`\\`\n\n> \\`\\`\\`md\n> <!-- prettier-ignore -->\n> - This is a long long\n>   long long long long\n>   long long paragraph.\n> \\`\\`\\`\n\n> - test\n>   \\`\\`\\`md\n>   <!-- prettier-ignore -->\n>   - This is a long long\n>     long long long long\n>     long long paragraph.\n>   \\`\\`\\`\n\n\\`\\`\\`\\`md\n> \\`\\`\\`md\n> <!-- prettier-ignore -->\n> - This is a long long\n>   long long long long\n>   long long paragraph.\n> \\`\\`\\`\n\\`\\`\\`\\`\n\n> \\`\\`\\`\\`md\n> > \\`\\`\\`md\n> > <!-- prettier-ignore -->\n> > - This is a long long\n> >   long long long long\n> >   long long paragraph.\n> > \\`\\`\\`\n> \\`\\`\\`\\`\n\n> <!-- prettier-ignore -->\n> - This is a long long\n>   long long long long\n>   long long paragraph.\n\n> \\`\\`\\`js\n> // prettier-ignore\n> const x = 1,\n> b = 2\n> \\`\\`\\`");
    Ok(())
}
#[test]
fn test_nested_md_prose_wrapalways_format_1_bf33912c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(">>> 123")?;
    assert_eq!(formatted, "> > > 123");
    Ok(())
}
#[test]
fn test_paragraph_md_prose_wrapalways_format_1_146e0aee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> This is a long long long long long long long long long long long long long long long paragraph.\n> This is a long long long long long long long long long long long long long long long paragraph.") ? ;
    assert_eq ! (formatted , "> This is a long long long long long long long long long long long long long\n> long long paragraph. This is a long long long long long long long long long\n> long long long long long long paragraph.");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_7c4e633a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 123")?;
    assert_eq!(formatted, "> 123");
    Ok(())
}
