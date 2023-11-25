#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_beginning_tag_after_a_list_item_md_prose_wrapalways_format_1_542ae1de() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- A list item.\n<details><summary>Summary</summary>\n<p>\n\n- A list item.\n\n</p>\n</details>\n\n- A list item.\n<blockquote>\n\n<p>quoted sentence1<br>\nquoted sentence2</p>\n</blockquote>\n\n- indented html is formatted as it is\n  <blockquote>asdf</blockquote>") ? ;
    assert_eq ! (formatted , "- A list item.\n<details><summary>Summary</summary>\n<p>\n\n- A list item.\n\n</p>\n</details>\n\n- A list item.\n<blockquote>\n\n<p>quoted sentence1<br>\nquoted sentence2</p>\n</blockquote>\n\n- indented html is formatted as it is\n  <blockquote>asdf</blockquote>");
    Ok(())
}
#[test]
fn test_blank_line_between_htmls_md_prose_wrapalways_format_1_5115987e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!--lint disable no-html-->\n<p align=\"center\"><img src=\"logo/vertical.png\" alt=\"labelify\" height=\"150px\"></p>\n<!--lint enable no-html-->") ? ;
    assert_eq ! (formatted , "<!--lint disable no-html-->\n<p align=\"center\"><img src=\"logo/vertical.png\" alt=\"labelify\" height=\"150px\"></p>\n<!--lint enable no-html-->");
    Ok(())
}
#[test]
fn test_multiline_md_prose_wrapalways_format_1_89315fa6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("1.  Some test text, the goal is to have the html table below nested within this number. When formating on save Prettier will continue to add an indent each time pushing the table further and further out of sync. \n\n    <table class=\"table table-striped\">\n    <tr>\n    <th>Test</th>\n    <th>Table</th>\n    </tr>\n    <tbody>\n        <tr>\n        <td>will</td>\n        <td>be</td>\n        </tr>\n        <tr>\n        <td>pushed</td>\n        <td>When</td>\n        </tr>\n        <tr>\n        <td>Format on</td>\n        <td>Save</td>\n        </tr>\n    </tbody>\n    </table>") ? ;
    assert_eq ! (formatted , "1.  Some test text, the goal is to have the html table below nested within this\n    number. When formating on save Prettier will continue to add an indent each\n    time pushing the table further and further out of sync.\n\n    <table class=\"table table-striped\">\n    <tr>\n    <th>Test</th>\n    <th>Table</th>\n    </tr>\n    <tbody>\n        <tr>\n        <td>will</td>\n        <td>be</td>\n        </tr>\n        <tr>\n        <td>pushed</td>\n        <td>When</td>\n        </tr>\n        <tr>\n        <td>Format on</td>\n        <td>Save</td>\n        </tr>\n    </tbody>\n    </table>");
    Ok(())
}
#[test]
fn test_multiline_with_trailing_space_md_prose_wrapalways_format_1_ff433f23() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("1.  Some test text, the goal is to have the html table below nested within this number. When formating on save Prettier will continue to add an indent each time pushing the table further and further out of sync. \n\n    <table class=\"table table-striped\">\n    <tr>\n    <th>Test</th>\n    <th>Table</th>\n    </tr>\n    <tbody>\n        <tr>\n        <td>will</td>  \n        <td>be</td>\n        </tr>\n        <tr>  \n        <td>pushed</td>  \n        <td>When</td>\n        </tr>  \n        <tr>  \n        <td>Format on</td>  \n        <td>Save</td>  \n        </tr>\n    </tbody>\n    </table>") ? ;
    assert_eq ! (formatted , "1.  Some test text, the goal is to have the html table below nested within this\n    number. When formating on save Prettier will continue to add an indent each\n    time pushing the table further and further out of sync.\n\n    <table class=\"table table-striped\">\n    <tr>\n    <th>Test</th>\n    <th>Table</th>\n    </tr>\n    <tbody>\n        <tr>\n        <td>will</td>  \n        <td>be</td>\n        </tr>\n        <tr>  \n        <td>pushed</td>  \n        <td>When</td>\n        </tr>  \n        <tr>  \n        <td>Format on</td>  \n        <td>Save</td>  \n        </tr>\n    </tbody>\n    </table>");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_c527a880() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!-- hello world -->")?;
    assert_eq!(formatted, "<!-- hello world -->");
    Ok(())
}
