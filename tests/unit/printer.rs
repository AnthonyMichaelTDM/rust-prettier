use rust_prettier::{
    document::{
        builders::{cursor, hardline},
        Doc,
    },
    PrettyPrinter, PrettyPrinterBuilder,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref OPTIONS: PrettyPrinter = {
        PrettyPrinterBuilder::default()
            .print_width(80)
            .tab_width(2)
            .build()
            .unwrap()
    };
}

#[test]
fn test_too_many_cursors() {
    let doc = Doc::from(vec![cursor(), cursor(), cursor(), cursor()]);

    assert_eq!(
        doc.format(OPTIONS.clone()).unwrap_err().to_string(),
        "There are too many 'cursor' in doc."
    );
}

#[test]
fn test_trim_first_blank_line() {
    let doc: Doc = Doc::from(vec![
        "   \t".into(),
        hardline(),
        "Prettier".into(),
        hardline(),
    ]);

    assert_eq!(doc.format(OPTIONS.clone()).unwrap(), "\nPrettier\n");
}

#[test]
fn test_properly_trim_with_cursor() {
    let doc: Doc = Doc::from(vec![
        cursor(),
        "Prettier  \t".into(),
        cursor(),
        "\t \t".into(),
        hardline(),
    ]);

    assert_eq!(doc.format(OPTIONS.clone()).unwrap(), "Prettier\n");
}
