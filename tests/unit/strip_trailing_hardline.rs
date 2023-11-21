use rust_prettier::document::{
    builders::{fill, hardline},
    utils::strip_trailing_hardline,
    Doc,
};

mod does_not_mutate_doc {
    use super::*;

    #[test]
    fn test_fill() {
        let doc: Doc = fill(vec![Box::new("text".into()), Box::new(hardline())]);
        let doc_clone = doc.clone();
        let expected = fill(vec![
            Box::new("text".into()),
            // Doc::from(vec![Doc::Array(Vec::new())]).into(),
            Box::new(Doc::Array(Vec::new())),
        ]);

        let stripped = strip_trailing_hardline(&doc);

        assert_eq!(doc, doc_clone);
        assert_eq!(stripped, expected);
    }

    #[test]
    fn test_fill_nested_hardline() {
        let doc: Doc = fill(vec![
            Box::new("text".into()),
            Box::new(Doc::from(vec![hardline()])),
        ]);
        let doc_clone = doc.clone();
        let expected = fill(vec![
            Box::new("text".into()),
            // Doc::from(vec![Doc::Array(Vec::new())]).into(),
            Box::new(Doc::Array(Vec::new())),
        ]);

        let stripped = strip_trailing_hardline(&doc);

        assert_eq!(doc, doc_clone);
        assert_eq!(stripped, expected);
    }

    #[test]
    fn test_array() {
        let doc: Doc = Doc::from(vec![Box::new("text".into()), Box::new(hardline())]);
        let doc_clone = doc.clone();
        let expected = Doc::from(vec![Box::new("text".into())]);

        let stripped = strip_trailing_hardline(&doc);

        assert_eq!(doc, doc_clone);
        assert_eq!(stripped, expected);
    }
}

#[test]
fn test_works_with_strings() {
    let doc: Doc = "\ntext\n\n\r\r\r\n\r\n".into();
    let expected: Doc = "\ntext".into();
    let stripped = strip_trailing_hardline(&doc);

    assert_eq!(stripped, expected);
}
