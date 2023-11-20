#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_block_lang_vue_embedded_language_formattingoff_format_1_0c08b579() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">\nhello,\nworld!\n</custom>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\nhello,\nworld!\n</custom>"
    );
}
#[test]
fn test_custom_block_lang_vue_format_1_0c08b579() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">\nhello,\nworld!\n</custom>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\nHELLO,\nWORLD!\n</custom>"
    );
}
#[test]
fn test_inline_vue_embedded_language_formattingoff_format_1_dc30219e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">hello, world!</custom>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">hello, world!</custom>"
    );
}
#[test]
fn test_inline_vue_format_1_dc30219e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">hello, world!</custom>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\nHELLO, WORLD!\n</custom>"
    );
}
#[test]
fn test_script_lang_vue_embedded_language_formattingoff_format_1_3f9e6d87() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<script lang=\"uppercase-rocks\">\nhello,\nworld!\n</script>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<script lang=\"uppercase-rocks\">\nhello,\nworld!\n</script>"
    );
}
#[test]
fn test_script_lang_vue_format_1_3f9e6d87() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<script lang=\"uppercase-rocks\">\nhello,\nworld!\n</script>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<script lang=\"uppercase-rocks\">\nHELLO,\nWORLD!\n</script>"
    );
}
#[test]
fn test_style_lang_vue_embedded_language_formattingoff_format_1_00616e63() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<style lang=\"uppercase-rocks\">\n/* Should be uppercased */\nhello,\nworld!\n</style>",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<style lang=\"uppercase-rocks\">\n/* Should be uppercased */\nhello,\nworld!\n</style>"
    );
}
#[test]
fn test_style_lang_vue_format_1_00616e63() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<style lang=\"uppercase-rocks\">\n/* Should be uppercased */\nhello,\nworld!\n</style>",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<style lang=\"uppercase-rocks\">\n/* SHOULD BE UPPERCASED */\nHELLO,\nWORLD!\n</style>"
    );
}
#[test]
fn test_template_lang_vue_embedded_language_formattingoff_format_1_95c23a88() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<template lang=\"uppercase-rocks\">\nhello,\nworld!\n</template>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<template lang=\"uppercase-rocks\">\nhello,\nworld!\n</template>"
    );
}
#[test]
fn test_template_lang_vue_format_1_95c23a88() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<template lang=\"uppercase-rocks\">\nhello,\nworld!\n</template>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<template lang=\"uppercase-rocks\">\nHELLO,\nWORLD!\n</template>"
    );
}
#[test]
fn test_whitspace_vue_embedded_language_formattingoff_format_1_d28cef31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">\n  hello,\n  world!\n</custom>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\n  hello,\n  world!\n</custom>"
    );
}
#[test]
fn test_whitspace_vue_format_1_d28cef31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<custom lang=\"uppercase-rocks\">\n  hello,\n  world!\n</custom>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<custom lang=\"uppercase-rocks\">\n  HELLO,\n  WORLD!\n</custom>"
    );
}
#[test]
fn test_with_src_vue_embedded_language_formattingoff_format_1_e42a005b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .embedded_language_formatting("off")
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></template>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></style>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></script>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></custom>\n<template lang=\"uppercase-rocks\" src></template>\n<template lang=\"uppercase-rocks\" src=\"\"></template>\n<template lang=\"uppercase-rocks\" src>PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src>\n\n     </template>\n<template lang=\"uppercase-rocks\" src=\"\">\n\n     </template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\">\nPrEtTiEr\n</template>\n\n\n<template lang=\"uppercase-rocks\" :src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" @src=\"\">PrEtTiEr</template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<custom lang=\"uppercase-rocks\" src/>\n<template lang=\"uppercase-rocks\" src=\"\"/>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></template>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></style>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></script>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></custom>\n<template lang=\"uppercase-rocks\" src></template>\n<template lang=\"uppercase-rocks\" src=\"\"></template>\n<template lang=\"uppercase-rocks\" src>PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src>\n\n     </template>\n<template lang=\"uppercase-rocks\" src=\"\">\n\n     </template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\">\nPrEtTiEr\n</template>\n\n<template lang=\"uppercase-rocks\" :src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" @src=\"\">PrEtTiEr</template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<custom lang=\"uppercase-rocks\" src />\n<template lang=\"uppercase-rocks\" src=\"\" />");
}
#[test]
fn test_with_src_vue_format_1_e42a005b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></template>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></style>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></script>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></custom>\n<template lang=\"uppercase-rocks\" src></template>\n<template lang=\"uppercase-rocks\" src=\"\"></template>\n<template lang=\"uppercase-rocks\" src>PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src>\n\n     </template>\n<template lang=\"uppercase-rocks\" src=\"\">\n\n     </template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\">\nPrEtTiEr\n</template>\n\n\n<template lang=\"uppercase-rocks\" :src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" @src=\"\">PrEtTiEr</template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"/>\n<custom lang=\"uppercase-rocks\" src/>\n<template lang=\"uppercase-rocks\" src=\"\"/>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></template>\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></style>\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></script>\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\"></custom>\n<template lang=\"uppercase-rocks\" src></template>\n<template lang=\"uppercase-rocks\" src=\"\"></template>\n<template lang=\"uppercase-rocks\" src>PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src=\"\">PrEtTiEr</template>\n<template lang=\"uppercase-rocks\" src>\n\n     </template>\n<template lang=\"uppercase-rocks\" src=\"\">\n\n     </template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\">\nPrEtTiEr\n</template>\n\n<template lang=\"uppercase-rocks\" :src=\"\">\nPRETTIER\n</template>\n<template lang=\"uppercase-rocks\" @src=\"\">\nPRETTIER\n</template>\n\n<template lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<style lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<script lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<custom lang=\"uppercase-rocks\" src=\"foo.uppercase-rocks\" />\n<custom lang=\"uppercase-rocks\" src />\n<template lang=\"uppercase-rocks\" src=\"\" />");
}
