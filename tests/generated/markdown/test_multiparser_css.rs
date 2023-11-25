use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_css_comment_md_format_1_5f4ae9e5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`\\`\\`pcss\n.Avatar {\n  /* ... */\n\n  &__image {\n    /* ... */\n\n    @container (width > 100px) {\n      /*\n      Change some styles on the image element when the container is\n      wider than 100px\n      */\n    }\n  }\n\n  @container (aspect-ratio > 3) {\n    /* Change styles on the avatar itself, when the aspect-ratio is grater than 3 */\n  }\n\n  @container (width > 100px) and (height > 100px) {\n    /* ... */\n  }\n}\n\\`\\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`\\`\\`pcss\n.Avatar {\n  /* ... */\n\n  &__image {\n    /* ... */\n\n    @container (width > 100px) {\n      /*\n      Change some styles on the image element when the container is\n      wider than 100px\n      */\n    }\n  }\n\n  @container (aspect-ratio > 3) {\n    /* Change styles on the avatar itself, when the aspect-ratio is grater than 3 */\n  }\n\n  @container (width > 100px) and (height > 100px) {\n    /* ... */\n  }\n}\n\\`\\`\\`");
}
