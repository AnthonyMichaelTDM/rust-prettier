use rust_prettier::document::{
    builders::{fill, hardline},
    utils::strip_trailing_hardline,
    Doc,
};

mod does_not_mutate_doc {
    use super::*;

    #[test]
    fn test_fill() {
        let doc: Doc = fill(vec!["text".into(), hardline()]);
        #[allow(clippy::redundant_clone)]
        let doc_clone = doc.clone();
        let expected = fill(vec!["text".into(), Doc::Array(Vec::new())]);

        let stripped = strip_trailing_hardline(&doc);

        assert_eq!(doc, doc_clone);
        assert_eq!(stripped, expected);
    }

    #[test]
    fn test_fill_nested_hardline() {
        let doc: Doc = fill(vec!["text".into(), vec![hardline()].into()]);
        #[allow(clippy::redundant_clone)]
        let doc_clone = doc.clone();
        let expected = fill(vec!["text".into(), Vec::new().into()]);

        let stripped = strip_trailing_hardline(&doc);

        assert_eq!(doc, doc_clone);
        assert_eq!(stripped, expected);
    }

    #[test]
    fn test_array() {
        let doc: Doc = Doc::from(vec!["text".into(), hardline()]);
        #[allow(clippy::redundant_clone)]
        let doc_clone = doc.clone();
        let expected = Doc::from(vec!["text".into()]);

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
