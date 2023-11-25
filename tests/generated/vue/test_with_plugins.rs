#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_block_lang_vue_embedded_language_formattingoff_format_1_0c08b579() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">\nhello,\nworld!\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\nhello,\nworld!\n</custom>"
    );
    Ok(())
}
#[test]
fn test_custom_block_lang_vue_format_1_0c08b579() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">\nhello,\nworld!\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\nHELLO,\nWORLD!\n</custom>"
    );
    Ok(())
}
#[test]
fn test_inline_vue_embedded_language_formattingoff_format_1_dc30219e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">hello, world!</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">hello, world!</custom>"
    );
    Ok(())
}
#[test]
fn test_inline_vue_format_1_dc30219e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">hello, world!</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\nHELLO, WORLD!\n</custom>"
    );
    Ok(())
}
#[test]
fn test_script_lang_vue_embedded_language_formattingoff_format_1_3f9e6d87() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<script lang=\"uppercase-rocks\">\nhello,\nworld!\n</script>")?;
    assert_eq!(
        formatted,
        "<script lang=\"uppercase-rocks\">\nhello,\nworld!\n</script>"
    );
    Ok(())
}
#[test]
fn test_script_lang_vue_format_1_3f9e6d87() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<script lang=\"uppercase-rocks\">\nhello,\nworld!\n</script>")?;
    assert_eq!(
        formatted,
        "<script lang=\"uppercase-rocks\">\nHELLO,\nWORLD!\n</script>"
    );
    Ok(())
}
#[test]
fn test_style_lang_vue_embedded_language_formattingoff_format_1_00616e63() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<style lang=\"uppercase-rocks\">\n/* Should be uppercased */\nhello,\nworld!\n</style>",
    )?;
    assert_eq!(
        formatted,
        "<style lang=\"uppercase-rocks\">\n/* Should be uppercased */\nhello,\nworld!\n</style>"
    );
    Ok(())
}
#[test]
fn test_style_lang_vue_format_1_00616e63() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<style lang=\"uppercase-rocks\">\n/* Should be uppercased */\nhello,\nworld!\n</style>",
    )?;
    assert_eq!(
        formatted,
        "<style lang=\"uppercase-rocks\">\n/* SHOULD BE UPPERCASED */\nHELLO,\nWORLD!\n</style>"
    );
    Ok(())
}
#[test]
fn test_template_lang_vue_embedded_language_formattingoff_format_1_95c23a88() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<template lang=\"uppercase-rocks\">\nhello,\nworld!\n</template>")?;
    assert_eq!(
        formatted,
        "<template lang=\"uppercase-rocks\">\nhello,\nworld!\n</template>"
    );
    Ok(())
}
#[test]
fn test_template_lang_vue_format_1_95c23a88() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<template lang=\"uppercase-rocks\">\nhello,\nworld!\n</template>")?;
    assert_eq!(
        formatted,
        "<template lang=\"uppercase-rocks\">\nHELLO,\nWORLD!\n</template>"
    );
    Ok(())
}
#[test]
fn test_whitspace_vue_embedded_language_formattingoff_format_1_d28cef31() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<custom lang=\"uppercase-rocks\">\n  hello,\n  world!\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\n  hello,\n  world!\n</custom>"
    );
    Ok(())
}
#[test]
fn test_whitspace_vue_format_1_d28cef31() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<custom lang=\"uppercase-rocks\">\n  hello,\n  world!\n</custom>")?;
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\n  HELLO,\n  WORLD!\n</custom>"
    );
    Ok(())
}
#[test]
fn test_with_src_vue_embedded_language_formattingoff_format_1_e42a005b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></template>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></style>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></script>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></custom>\n<template lang=\"uppercase-rocks\" src></template>\n<template lang=\"uppercase-rocks\" src=\"\"></template>\n<template lang=\"uppercase-rocks\" src>PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src>\n\n     </template>\n<template lang=\"uppercase-rocks\" src=\"\">\n\n     </template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\">\nPrEtTiEr\n</template>\n\n\n<template lang=\"uppercase-rocks\" :src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" @src=\"\">PrEtTiEr</template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<custom lang=\"uppercase-rocks\" src/>\n<template lang=\"uppercase-rocks\" src=\"\"/>") ? ;
    assert_eq ! (formatted , "<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></template>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></style>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></script>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></custom>\n<template lang=\"uppercase-rocks\" src></template>\n<template lang=\"uppercase-rocks\" src=\"\"></template>\n<template lang=\"uppercase-rocks\" src>PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src>\n\n     </template>\n<template lang=\"uppercase-rocks\" src=\"\">\n\n     </template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\">\nPrEtTiEr\n</template>\n\n<template lang=\"uppercase-rocks\" :src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" @src=\"\">PrEtTiEr</template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<custom lang=\"uppercase-rocks\" src />\n<template lang=\"uppercase-rocks\" src=\"\" />");
    Ok(())
}
#[test]
fn test_with_src_vue_format_1_e42a005b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></template>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></style>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></script>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></custom>\n<template lang=\"uppercase-rocks\" src></template>\n<template lang=\"uppercase-rocks\" src=\"\"></template>\n<template lang=\"uppercase-rocks\" src>PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src>\n\n     </template>\n<template lang=\"uppercase-rocks\" src=\"\">\n\n     </template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\">\nPrEtTiEr\n</template>\n\n\n<template lang=\"uppercase-rocks\" :src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" @src=\"\">PrEtTiEr</template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<custom lang=\"uppercase-rocks\" src/>\n<template lang=\"uppercase-rocks\" src=\"\"/>") ? ;
    assert_eq ! (formatted , "<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></template>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></style>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></script>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></custom>\n<template lang=\"uppercase-rocks\" src></template>\n<template lang=\"uppercase-rocks\" src=\"\"></template>\n<template lang=\"uppercase-rocks\" src>PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src>\n\n     </template>\n<template lang=\"uppercase-rocks\" src=\"\">\n\n     </template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\">\nPrEtTiEr\n</template>\n\n<template lang=\"uppercase-rocks\" :src=\"\">\nPRETTIER\n</template>\n<template lang=\"uppercase-rocks\" @src=\"\">\nPRETTIER\n</template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<custom lang=\"uppercase-rocks\" src />\n<template lang=\"uppercase-rocks\" src=\"\" />");
    Ok(())
}
