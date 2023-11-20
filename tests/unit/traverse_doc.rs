use rust_prettier::document::{find_in_doc, traverse_doc, Doc};

fn doc() -> Doc {
    Doc::Array(vec![Doc::Array(vec![
        Doc::String("foo".to_string()).into(),
        Doc::String("bar".to_string()).into(),
    ])
    .into()])
}

#[test]
fn test_traverse_doc() {
    let mut visited: Vec<Doc> = Vec::new();
    traverse_doc(
        &doc(),
        &mut visited,
        |d, visited| {
            visited.push(d.clone());
            true
        },
        None,
        None,
    );

    assert_eq!(
        visited,
        vec![
            Doc::Array(vec![Doc::Array(vec![
                Doc::String("foo".to_string()).into(),
                Doc::String("bar".to_string()).into(),
            ])
            .into()])
            .into(),
            Doc::Array(vec![
                Doc::String("foo".to_string()).into(),
                Doc::String("bar".to_string()).into(),
            ])
            .into(),
            Doc::String("foo".to_string()).into(),
            Doc::String("bar".to_string()).into(),
        ]
    );
}

#[test]
fn test_traverse_doc_skip_children() {
    let mut visited: Vec<Doc> = Vec::new();
    traverse_doc(
        &doc(),
        &mut visited,
        |d, visited| {
            visited.push(d.clone());
            false
        },
        None,
        None,
    );

    assert_eq!(
        visited,
        vec![Doc::Array(vec![Doc::Array(vec![
            Doc::String("foo".to_string()).into(),
            Doc::String("bar".to_string()).into(),
        ])
        .into()])
        .into(),]
    );
}

#[test]
fn test_traverse_doc_still_visit_siblings() {
    let mut visited: Vec<Doc> = Vec::new();
    traverse_doc(
        &doc(),
        &mut visited,
        |d, visited| {
            visited.push(d.clone());
            if matches!(d, Doc::String(s) if s == "foo") {
                false
            } else {
                true
            }
        },
        None,
        None,
    );

    assert_eq!(
        visited,
        vec![
            Doc::Array(vec![Doc::Array(vec![
                Doc::String("foo".to_string()).into(),
                Doc::String("bar".to_string()).into(),
            ])
            .into()])
            .into(),
            Doc::Array(vec![
                Doc::String("foo".to_string()).into(),
                Doc::String("bar".to_string()).into(),
            ])
            .into(),
            Doc::String("foo".to_string()).into(),
            Doc::String("bar".to_string()).into(),
        ]
    );
}

#[test]
fn test_find_in_doc() {
    let mut visited: Vec<Doc> = Vec::new();

    let result = find_in_doc(&doc(), &mut visited, |d, visited| {
        visited.push(d.clone());
        matches!(d, Doc::String(s) if s == "foo")
    });

    assert_eq!(result, Some(Doc::String("foo".to_string()).into()));
    assert_eq!(
        visited,
        vec![
            Doc::Array(vec![Doc::Array(vec![
                Doc::String("foo".to_string()).into(),
                Doc::String("bar".to_string()).into(),
            ])
            .into()])
            .into(),
            Doc::Array(vec![
                Doc::String("foo".to_string()).into(),
                Doc::String("bar".to_string()).into(),
            ])
            .into(),
            Doc::String("foo".to_string()).into(),
        ],
        "should stop visiting siblings after finding the first match"
    );
}

// Note: it is impossible to create an "invalid" doc because of rust's robust type system, therefore it doesn't make sense to test for it
