#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cjk_md_prose_wrapalways_format_1_1198871f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落\n\n這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n\nThis ia an english paragraph with a CJK quote \"中文\".\n\nThis ia an english paragraph with a CJK quote “中文“.\n\n扩展运算符（spread）是三个点（\\`...\\`）。\n\n::: warning 注意\n该网站在国外无法访问，故以下演示无效\n:::\n\nIVS 麻\u{e0101}羽\u{e0100}‼\u{fe0f}\n\n⿰あ⿱あ⿲あ⿳あ⿴あ⿵あ⿶あ⿷あ⿸あ⿹あ⿺あ⿻あ") ? ;
    assert_eq ! (formatted , "這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長\n很長的段落\n\n這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段\nParagraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的\n一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中\n文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合\n著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English\n混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個\nEnglish混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是\n一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n這是一個English混合著中文的一段Paragraph！\n\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n\nThis ia an english paragraph with a CJK quote \"中文\".\n\nThis ia an english paragraph with a CJK quote “中文“.\n\n扩展运算符（spread）是三个点（\\`...\\`）。\n\n::: warning 注意该网站在国外无法访问，故以下演示无效 :::\n\nIVS 麻\u{e0101}羽\u{e0100}‼\u{fe0f}\n\n⿰あ⿱あ⿲あ⿳あ⿴あ⿵あ⿶あ⿷あ⿸あ⿹あ⿺あ⿻あ");
    Ok(())
}
#[test]
fn test_cjk_md_prose_wrapnever_format_1_1198871f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落\n\n這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n\nThis ia an english paragraph with a CJK quote \"中文\".\n\nThis ia an english paragraph with a CJK quote “中文“.\n\n扩展运算符（spread）是三个点（\\`...\\`）。\n\n::: warning 注意\n该网站在国外无法访问，故以下演示无效\n:::\n\nIVS 麻\u{e0101}羽\u{e0100}‼\u{fe0f}\n\n⿰あ⿱あ⿲あ⿳あ⿴あ⿵あ⿶あ⿷あ⿸あ⿹あ⿺あ⿻あ") ? ;
    assert_eq ! (formatted , "這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落\n\n這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n\nThis ia an english paragraph with a CJK quote \"中文\".\n\nThis ia an english paragraph with a CJK quote “中文“.\n\n扩展运算符（spread）是三个点（\\`...\\`）。\n\n::: warning 注意该网站在国外无法访问，故以下演示无效 :::\n\nIVS 麻\u{e0101}羽\u{e0100}‼\u{fe0f}\n\n⿰あ⿱あ⿲あ⿳あ⿴あ⿵あ⿶あ⿷あ⿸あ⿹あ⿺あ⿻あ");
    Ok(())
}
#[test]
fn test_cjk_md_prose_wrappreserve_format_1_1198871f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落\n\n這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n\nThis ia an english paragraph with a CJK quote \"中文\".\n\nThis ia an english paragraph with a CJK quote “中文“.\n\n扩展运算符（spread）是三个点（\\`...\\`）。\n\n::: warning 注意\n该网站在国外无法访问，故以下演示无效\n:::\n\nIVS 麻\u{e0101}羽\u{e0100}‼\u{fe0f}\n\n⿰あ⿱あ⿲あ⿳あ⿴あ⿵あ⿶あ⿷あ⿸あ⿹あ⿺あ⿻あ") ? ;
    assert_eq ! (formatted , "這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落\n\n這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！這是一個English混合著中文的一段Paragraph！\n\n全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白全\u{3000}\u{3000}形\u{3000}空白\n\nThis ia an english paragraph with a CJK quote \"中文\".\n\nThis ia an english paragraph with a CJK quote “中文“.\n\n扩展运算符（spread）是三个点（\\`...\\`）。\n\n::: warning 注意\n该网站在国外无法访问，故以下演示无效\n:::\n\nIVS 麻\u{e0101}羽\u{e0100}‼\u{fe0f}\n\n⿰あ⿱あ⿲あ⿳あ⿴あ⿵あ⿶あ⿷あ⿸あ⿹あ⿺あ⿻あ");
    Ok(())
}
#[test]
fn test_inline_nodes_md_prose_wrapalways_format_1_f5bf40e8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("It removes all original styling[*](#styling-footnote) and ensures that all outputted code conforms to a consistent style. (See this [blog post](http://jlongster.com/A-Prettier-Formatter))") ? ;
    assert_eq ! (formatted , "It removes all original styling[\\\\*](#styling-footnote) and ensures that all\noutputted code conforms to a consistent style. (See this\n[blog post](http://jlongster.com/A-Prettier-Formatter))");
    Ok(())
}
#[test]
fn test_inline_nodes_md_prose_wrapnever_format_1_f5bf40e8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("It removes all original styling[*](#styling-footnote) and ensures that all outputted code conforms to a consistent style. (See this [blog post](http://jlongster.com/A-Prettier-Formatter))") ? ;
    assert_eq ! (formatted , "It removes all original styling[\\\\*](#styling-footnote) and ensures that all outputted code conforms to a consistent style. (See this [blog post](http://jlongster.com/A-Prettier-Formatter))");
    Ok(())
}
#[test]
fn test_inline_nodes_md_prose_wrappreserve_format_1_f5bf40e8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("It removes all original styling[*](#styling-footnote) and ensures that all outputted code conforms to a consistent style. (See this [blog post](http://jlongster.com/A-Prettier-Formatter))") ? ;
    assert_eq ! (formatted , "It removes all original styling[\\\\*](#styling-footnote) and ensures that all outputted code conforms to a consistent style. (See this [blog post](http://jlongster.com/A-Prettier-Formatter))");
    Ok(())
}
#[test]
fn test_lorem_md_prose_wrapalways_format_1_54dbea8d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Hic dicta et recusandae incidunt. Reiciendis saepe voluptatem tempore rem aut.\nIusto sapiente impedit. Laudantium ut id non et aperiam ab.\n \nSit minus architecto quas quibusdam sed ipsam aut eum.\nDolores tempora reiciendis magni blanditiis laborum aliquid rem corporis enim. Et consectetur quo sed excepturi soluta repudiandae commodi id.\nEum possimus optio distinctio incidunt quasi optio culpa accusamus.\nArchitecto esse ut aut autem ullam consequatur reiciendis aliquid dolorum.\n \nEt quam mollitia velit iste enim exercitationem nemo.\nHic dignissimos eos et. Eos eos consequatur.") ? ;
    assert_eq ! (formatted , "Hic dicta et recusandae incidunt. Reiciendis saepe voluptatem tempore rem aut.\nIusto sapiente impedit. Laudantium ut id non et aperiam ab.\n\nSit minus architecto quas quibusdam sed ipsam aut eum. Dolores tempora\nreiciendis magni blanditiis laborum aliquid rem corporis enim. Et consectetur\nquo sed excepturi soluta repudiandae commodi id. Eum possimus optio distinctio\nincidunt quasi optio culpa accusamus. Architecto esse ut aut autem ullam\nconsequatur reiciendis aliquid dolorum.\n\nEt quam mollitia velit iste enim exercitationem nemo. Hic dignissimos eos et.\nEos eos consequatur.");
    Ok(())
}
#[test]
fn test_lorem_md_prose_wrapnever_format_1_54dbea8d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Hic dicta et recusandae incidunt. Reiciendis saepe voluptatem tempore rem aut.\nIusto sapiente impedit. Laudantium ut id non et aperiam ab.\n \nSit minus architecto quas quibusdam sed ipsam aut eum.\nDolores tempora reiciendis magni blanditiis laborum aliquid rem corporis enim. Et consectetur quo sed excepturi soluta repudiandae commodi id.\nEum possimus optio distinctio incidunt quasi optio culpa accusamus.\nArchitecto esse ut aut autem ullam consequatur reiciendis aliquid dolorum.\n \nEt quam mollitia velit iste enim exercitationem nemo.\nHic dignissimos eos et. Eos eos consequatur.") ? ;
    assert_eq ! (formatted , "Hic dicta et recusandae incidunt. Reiciendis saepe voluptatem tempore rem aut. Iusto sapiente impedit. Laudantium ut id non et aperiam ab.\n\nSit minus architecto quas quibusdam sed ipsam aut eum. Dolores tempora reiciendis magni blanditiis laborum aliquid rem corporis enim. Et consectetur quo sed excepturi soluta repudiandae commodi id. Eum possimus optio distinctio incidunt quasi optio culpa accusamus. Architecto esse ut aut autem ullam consequatur reiciendis aliquid dolorum.\n\nEt quam mollitia velit iste enim exercitationem nemo. Hic dignissimos eos et. Eos eos consequatur.");
    Ok(())
}
#[test]
fn test_lorem_md_prose_wrappreserve_format_1_54dbea8d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Hic dicta et recusandae incidunt. Reiciendis saepe voluptatem tempore rem aut.\nIusto sapiente impedit. Laudantium ut id non et aperiam ab.\n \nSit minus architecto quas quibusdam sed ipsam aut eum.\nDolores tempora reiciendis magni blanditiis laborum aliquid rem corporis enim. Et consectetur quo sed excepturi soluta repudiandae commodi id.\nEum possimus optio distinctio incidunt quasi optio culpa accusamus.\nArchitecto esse ut aut autem ullam consequatur reiciendis aliquid dolorum.\n \nEt quam mollitia velit iste enim exercitationem nemo.\nHic dignissimos eos et. Eos eos consequatur.") ? ;
    assert_eq ! (formatted , "Hic dicta et recusandae incidunt. Reiciendis saepe voluptatem tempore rem aut.\nIusto sapiente impedit. Laudantium ut id non et aperiam ab.\n\nSit minus architecto quas quibusdam sed ipsam aut eum.\nDolores tempora reiciendis magni blanditiis laborum aliquid rem corporis enim. Et consectetur quo sed excepturi soluta repudiandae commodi id.\nEum possimus optio distinctio incidunt quasi optio culpa accusamus.\nArchitecto esse ut aut autem ullam consequatur reiciendis aliquid dolorum.\n\nEt quam mollitia velit iste enim exercitationem nemo.\nHic dignissimos eos et. Eos eos consequatur.");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_8d3f2fe7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("This is a long long long long long long long long long long long long long long long paragraph.") ? ;
    assert_eq ! (formatted , "This is a long long long long long long long long long long long long long long\nlong paragraph.");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapnever_format_1_8d3f2fe7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("This is a long long long long long long long long long long long long long long long paragraph.") ? ;
    assert_eq ! (formatted , "This is a long long long long long long long long long long long long long long long paragraph.");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrappreserve_format_1_8d3f2fe7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("This is a long long long long long long long long long long long long long long long paragraph.") ? ;
    assert_eq ! (formatted , "This is a long long long long long long long long long long long long long long long paragraph.");
    Ok(())
}
#[test]
fn test_special_prefix_md_prose_wrapalways_format_1_57c1354b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc - abc abc abc \n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md) - disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md) - disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md) - disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) - ensure expect is called correctly.\n\n## Supported Rules\n\n* [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md)\n  - disallow disabled tests.\n* [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md)\n  - disallow focused tests.\n* [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md)\n  - disallow identical titles.\n* [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) -\n  ensure expect is called correctly.\n\nShe grew up in an isolated village in the 19th century and met her father aged 29. Oh no, why are we in a numbered list now?") ? ;
    assert_eq ! (formatted , "abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc\nabc - abc abc abc\n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md) -\n  disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md) -\n  disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md) -\n  disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) -\n  ensure expect is called correctly.\n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md)\n  - disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md)\n  - disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md)\n  - disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) -\n  ensure expect is called correctly.\n\nShe grew up in an isolated village in the 19th century and met her father\naged 29. Oh no, why are we in a numbered list now?");
    Ok(())
}
#[test]
fn test_special_prefix_md_prose_wrapnever_format_1_57c1354b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc - abc abc abc \n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md) - disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md) - disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md) - disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) - ensure expect is called correctly.\n\n## Supported Rules\n\n* [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md)\n  - disallow disabled tests.\n* [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md)\n  - disallow focused tests.\n* [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md)\n  - disallow identical titles.\n* [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) -\n  ensure expect is called correctly.\n\nShe grew up in an isolated village in the 19th century and met her father aged 29. Oh no, why are we in a numbered list now?") ? ;
    assert_eq ! (formatted , "abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc - abc abc abc\n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md) - disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md) - disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md) - disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) - ensure expect is called correctly.\n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md)\n  - disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md)\n  - disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md)\n  - disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) - ensure expect is called correctly.\n\nShe grew up in an isolated village in the 19th century and met her father aged 29. Oh no, why are we in a numbered list now?");
    Ok(())
}
#[test]
fn test_special_prefix_md_prose_wrappreserve_format_1_57c1354b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc - abc abc abc \n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md) - disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md) - disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md) - disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) - ensure expect is called correctly.\n\n## Supported Rules\n\n* [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md)\n  - disallow disabled tests.\n* [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md)\n  - disallow focused tests.\n* [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md)\n  - disallow identical titles.\n* [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) -\n  ensure expect is called correctly.\n\nShe grew up in an isolated village in the 19th century and met her father aged 29. Oh no, why are we in a numbered list now?") ? ;
    assert_eq ! (formatted , "abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc abc - abc abc abc\n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md) - disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md) - disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md) - disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) - ensure expect is called correctly.\n\n## Supported Rules\n\n- [no-disabled-tests](/packages/eslint-plugin-jest/docs/rules/no-disabled-tests.md)\n  - disallow disabled tests.\n- [no-focused-tests](/packages/eslint-plugin-jest/docs/rules/no-focused-tests.md)\n  - disallow focused tests.\n- [no-identical-title](/packages/eslint-plugin-jest/docs/rules/no-identical-title.md)\n  - disallow identical titles.\n- [valid-expect](/packages/eslint-plugin-jest/docs/rules/valid-expect.md) -\n  ensure expect is called correctly.\n\nShe grew up in an isolated village in the 19th century and met her father aged 29. Oh no, why are we in a numbered list now?");
    Ok(())
}
#[test]
fn test_whitespace_md_prose_wrapalways_format_1_226aaed9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- 0xA0 non-breaking whitespace -->\n\nkeep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\n\n<!-- 0x20 standard whitespace -->\n\nkeep these words together keep these words together keep these words together keep these words together") ? ;
    assert_eq ! (formatted , "<!-- 0xA0 non-breaking whitespace -->\n\nkeep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\n\n<!-- 0x20 standard whitespace -->\n\nkeep these words together keep these words together keep these words together\nkeep these words together");
    Ok(())
}
#[test]
fn test_whitespace_md_prose_wrapnever_format_1_226aaed9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- 0xA0 non-breaking whitespace -->\n\nkeep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\n\n<!-- 0x20 standard whitespace -->\n\nkeep these words together keep these words together keep these words together keep these words together") ? ;
    assert_eq ! (formatted , "<!-- 0xA0 non-breaking whitespace -->\n\nkeep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\n\n<!-- 0x20 standard whitespace -->\n\nkeep these words together keep these words together keep these words together keep these words together");
    Ok(())
}
#[test]
fn test_whitespace_md_prose_wrappreserve_format_1_226aaed9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- 0xA0 non-breaking whitespace -->\n\nkeep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\n\n<!-- 0x20 standard whitespace -->\n\nkeep these words together keep these words together keep these words together keep these words together") ? ;
    assert_eq ! (formatted , "<!-- 0xA0 non-breaking whitespace -->\n\nkeep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\u{a0}keep\u{a0}these\u{a0}words\u{a0}together\n\n<!-- 0x20 standard whitespace -->\n\nkeep these words together keep these words together keep these words together keep these words together");
    Ok(())
}
