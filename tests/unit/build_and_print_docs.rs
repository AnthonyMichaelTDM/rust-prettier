use std::num::NonZeroUsize;

use rust_prettier::{
    common::Symbol,
    document::{builders::*, Align, Doc},
    PrettyPrinter, PrettyPrinterBuilder,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref PRINTER: PrettyPrinter = PrettyPrinter::default();
}

#[test]
fn test_string() {
    let doc: Doc = "hello world".into();
    let expected = "hello world";
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    let doc: Doc = "hello\nworld".into();
    let expected = "hello\nworld";
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    let doc: Doc = "hello\nworld\n".into();
    let expected = "hello\nworld\n";
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_concat() {
    let doc: Doc = concat(vec!["hello".into(), "world".into()]);
    let expected = "helloworld";
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    let doc: Doc = concat(vec!["hello".into(), "world".into(), "\n".into()]);
    let expected = "helloworld\n";
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    let doc: Doc = concat(vec![
        "hello".into(),
        "world".into(),
        "\n".into(),
        "\n".into(),
    ]);
    let expected = "helloworld\n\n";
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_indent() {
    let doc: Doc = concat([indent(softline()), "hello".into()]);
    let expected = "\n    hello";
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    // if we just have an indent w/o a newline, it should be ignored because what if it's in the middle of a line?
    let doc: Doc = concat([indent("hello".into()), "world".into()]);
    let expected = "helloworld";
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_align() {
    // align with a string
    let doc = concat([
        hardline(),
        "case SomePattern".into(),
        align(
            concat([
                hardline(),
                "SomeValue".into(),
                hardline(),
                "SomeOtherValue".into(),
                hardline(),
                "SomeOtherOtherValue".into(),
            ]),
            Align::With("|  ".into()),
        ),
    ]);
    let expected = r#"
case SomePattern
|  SomeValue
|  SomeOtherValue
|  SomeOtherOtherValue"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    // align by a number of spaces
    let doc = concat([
        hardline(),
        "case SomePattern".into(),
        align(
            align(
                concat([
                    hardline(),
                    "| SomeValue".into(),
                    hardline(),
                    "| SomeOtherValue".into(),
                    hardline(),
                    "| SomeOtherOtherValue".into(),
                ]),
                Align::By(5),
            ),
            Align::By(4),
        ),
    ]);
    let expected = r#"
case SomePattern
         | SomeValue
         | SomeOtherValue
         | SomeOtherOtherValue"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_dedent_to_root() {
    let doc: Doc = concat([
        softline(),
        "hello".into(),
        indent(concat([
            softline(),
            "world".into(),
            indent(concat([
                softline(),
                "foo".into(),
                indent(concat([
                    softline(),
                    "bar".into(),
                    indent(concat([
                        softline(),
                        "baz".into(),
                        dedent_to_root(softline()),
                    ])),
                ])),
            ])),
        ])),
        "root".into(),
    ]);
    let expected = r#"
hello
    world
        foo
            bar
                baz
root"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_mark_as_root() {
    let doc: Doc = concat([
        softline(),
        "hello".into(),
        indent(concat([
            softline(),
            mark_as_root("world".into()),
            indent(concat([
                softline(),
                "foo".into(),
                indent(concat([
                    softline(),
                    "bar".into(),
                    indent(concat([
                        softline(),
                        "baz".into(),
                        dedent_to_root(softline()),
                    ])),
                ])),
            ])),
        ])),
        "root".into(),
    ]);
    let expected = r#"
hello
    world
        foo
            bar
                baz
    root"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_dedent() {
    let doc: Doc = concat([
        softline(),
        "hello".into(),
        indent(concat([
            softline(),
            "world".into(),
            indent(concat([
                softline(),
                "foo".into(),
                indent(softline()),
                "bar".into(),
                softline(),
                "baz".into(),
                dedent(softline()),
                "root".into(),
            ])),
        ])),
    ]);
    let expected = r#"
hello
    world
        foo
            bar
            baz
        root"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    let doc: Doc = concat([
        softline(),
        "hello".into(),
        indent(concat([
            softline(),
            "world".into(),
            indent(concat([
                softline(),
                "foo".into(),
                softline(),
                "bar".into(),
                softline(),
                "baz".into(),
                dedent(softline()),
                "root".into(),
            ])),
        ])),
    ]);
    let expected = r#"
hello
    world
        foo
        bar
        baz
    root"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_group() {
    let group_contents = concat([
        "[".into(),
        indent(softline()),
        concat([
            "1,".into(),
            if_break(softline(), Some(" ".into()), None),
            "\"foo\",".into(),
            if_break(softline(), Some(" ".into()), None),
            "{ bar: 2 }".into(),
            if_break(",".into(), None, None),
        ]),
        dedent(softline()),
        "]".into(),
    ]);
    // test when not breaking
    let doc: Doc = group(group_contents.clone(), None, false, None);
    let expected = r#"[1, "foo", { bar: 2 }]"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
    // test when breaking
    let doc: Doc = group(group_contents, None, true, None);
    let expected = r#"[
    1,
    "foo",
    { bar: 2 },
]"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    let doc = group(
        concat([
            "[".into(),
            indent(group(
                concat([
                    softline(),
                    "1,".into(),
                    softline(),
                    concat([
                        "function () {".into(),
                        indent(hardline()),
                        "return 2;".into(),
                        dedent(hardline()),
                        "},".into(),
                    ]),
                    softline(),
                    "3,".into(),
                    dedent(softline()),
                ]),
                None,
                false,
                None,
            )),
            "];".into(),
        ]),
        None,
        false,
        None,
    );

    let options = PrettyPrinter::default();

    let formatted = doc.format(&options).unwrap();

    assert_eq!(
        formatted,
        r#"[
    1,
    function () {
        return 2;
    },
    3,
];"#
    );
}

#[test]
fn test_conditional_group() {
    let most_flat: Doc = "let closure = |n: usize| some_long_expr;".into();
    let most_expanded: Doc = concat([
        "let closure = |n: usize| {".into(),
        indent(hardline()),
        "some_long_expr".into(),
        dedent(hardline()),
        "};".into(),
    ]);
    let doc = conditional_group([most_flat.clone(), most_expanded.clone()], None, false).unwrap();

    // test when there's enough space for the most flat option
    let options = PrettyPrinterBuilder::default()
        .print_width(40)
        .build()
        .unwrap();
    assert_eq!(
        doc.format(&options).unwrap(),
        most_flat.format(&PRINTER).unwrap()
    );

    // test when there's not enough space for the most flat option
    let options = PrettyPrinterBuilder::default()
        .print_width(30)
        .build()
        .unwrap();
    assert_eq!(
        doc.format(&options).unwrap(),
        most_expanded.format(&PRINTER).unwrap()
    );
}

#[test]
fn test_fill() {
    let doc = fill(vec![
        "a".repeat(40).into(),
        softline(),
        "b".repeat(40).into(),
        softline(),
        "c".repeat(60).into(),
        softline(),
        "d".repeat(80).into(),
    ]);

    let options = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    assert_eq!(
        doc.format(&options).unwrap(),
        format!(
            "{}{}\n{}\n{}",
            "a".repeat(40),
            "b".repeat(40),
            "c".repeat(60),
            "d".repeat(80)
        )
    );

    let options = PrettyPrinterBuilder::default()
        .print_width(60)
        .build()
        .unwrap();
    assert_eq!(
        doc.format(&options).unwrap(),
        format!(
            "{}\n{}\n{}\n{}",
            "a".repeat(40),
            "b".repeat(40),
            "c".repeat(60),
            "d".repeat(80)
        )
    );
}

#[test]
fn test_if_break() {
    let group_contents = concat([
        "let closure = |n: usize| ".into(),
        if_break(
            concat([
                "{".into(),
                indent(softline()),
                "some_long_expr".into(),
                dedent(softline()),
                "};".into(),
            ]),
            Some("some_long_expr;".into()),
            None,
        ),
    ]);

    // test when not breaking
    let doc: Doc = group(group_contents.clone(), None, false, None);
    let expected = r#"let closure = |n: usize| some_long_expr;"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);

    // test when breaking
    let doc: Doc = group(group_contents, None, true, None);
    let expected = r#"let closure = |n: usize| {
    some_long_expr
};"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_indent_if_break() {
    let breaking: Doc = concat([
        hardline(),
        group(
            concat([
                "{".into(),
                softline(),
                "foo_group".into(),
                softline(),
                "}".into(),
            ]),
            Some(Symbol::String("foo".into())),
            true,
            None,
        ),
        hardline(),
        "hello".into(),
        indent_if_break(
            concat([hardline(), "world".into()]),
            Symbol::String("foo".into()),
            false,
        ),
    ]);

    let not_breaking: Doc = concat([
        hardline(),
        group(
            concat([
                "{".into(),
                softline(),
                "foo_group".into(),
                softline(),
                "}".into(),
            ]),
            Some(Symbol::String("foo".into())),
            false,
            None,
        ),
        hardline(),
        "hello".into(),
        indent_if_break(
            concat([hardline(), "world".into()]),
            Symbol::String("foo".into()),
            false,
        ),
    ]);

    let breaking_formatted = breaking.format(&PRINTER).unwrap();
    let not_breaking_formatted = not_breaking.format(&PRINTER).unwrap();

    assert_eq!(
        breaking_formatted,
        r#"
{
foo_group
}
hello
    world"#
    );
    assert_eq!(
        not_breaking_formatted,
        r#"
{foo_group}
hello
world"#
    );
}

#[test]
fn test_line_suffix() {
    let doc: Doc = Doc::from(vec![
        "a".into(),
        line_suffix(" // comment".into()),
        ";".into(),
        hardline(),
    ]);

    assert_eq!(doc.format(&PRINTER).unwrap(), "a; // comment\n");
}

#[test]
fn test_line_suffix_boundary() {
    let doc = concat([
        "{".into(),
        line_suffix(" // comment".into()),
        line_suffix_boundary(),
        "}".into(),
        hardline(),
    ]);

    assert_eq!(
        doc.format(&PrettyPrinter::default()).unwrap(),
        "{ // comment\n}\n"
    );
}

#[test]
fn test_break_parent() {
    let doc = group(
        concat([
            "{".into(),
            softline(),
            "foo: 2".into(),
            softline(),
            "}".into(),
            break_parent(),
        ]),
        None,
        false,
        None,
    );

    assert_eq!(
        doc.format(&PrettyPrinter::default()).unwrap(),
        "{\nfoo: 2\n}"
    );
}

#[test]
fn test_trim() {
    let doc: Doc = concat([
        softline(),
        "foo".into(),
        indent(concat([
            softline(),
            "bar".into(),
            indent(concat([softline(), "baz".into(), softline(), trim()])),
        ])),
        "root".into(),
    ]);
    let expected = r#"
foo
    bar
        baz
root"#;
    let actual = doc.format(&PRINTER).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_lines() {
    let doc = concat([
        hardline(),
        group(
            concat(["foo".into(), line(), "bar".into(), line(), "baz".into()]),
            None,
            false,
            None,
        ),
        hardline(),
        hardline(),
        group(
            concat([
                "foo".into(),
                hardline(),
                "bar".into(),
                softline(),
                "baz".into(),
            ]),
            None,
            false,
            None,
        ),
        hardline(),
        hardline(),
        group(
            concat([
                "foo".into(),
                softline(),
                "bar".into(),
                softline(),
                "baz".into(),
            ]),
            None,
            false,
            None,
        ),
        hardline(),
        hardline(),
        group(
            concat([
                "foo".into(),
                literalline(),
                "bar".into(),
                line(),
                "baz".into(),
            ]),
            None,
            false,
            None,
        ),
        hardline(),
        hardline(),
        group(
            concat([
                "foo".into(),
                literalline_without_break_parent(),
                "bar".into(),
                line(),
                "baz".into(),
            ]),
            None,
            false,
            None,
        ),
        hardline(),
        hardline(),
        group(
            concat([
                "foo".into(),
                hardline_without_break_parent(),
                "bar".into(),
                line(),
                "baz".into(),
            ]),
            None,
            false,
            None,
        ),
    ]);

    let expected = r#"
foobarbaz

foo
bar
baz

foobarbaz

foo
bar
baz

foo
barbaz

foo
barbaz"#;

    let actual = doc.format(&PrettyPrinter::default()).unwrap();
    assert_eq!(actual, expected);
}

#[test]
pub fn test_cursor() {
    let doc = concat(["hello".into(), cursor(), "world".into()]);
    let doc_too_many_cursors =
        concat(["hello".into(), cursor(), "world".into(), cursor(), cursor()]);

    let expected = "helloworld";

    let actual = doc.format(&PrettyPrinter::default()).unwrap();
    assert_eq!(actual, expected);

    let actual = doc_too_many_cursors.format(&PrettyPrinter::default());
    assert!(actual.is_err());
    assert_eq!(
        actual.unwrap_err().to_string(),
        "There are too many 'cursor' in doc."
    );
}

#[test]
pub fn test_join() {
    let doc = join(
        &", ".into(),
        vec![
            "hello".into(),
            "world".into(),
            "foo".into(),
            "bar".into(),
            "baz".into(),
        ],
    );

    let expected = "hello, world, foo, bar, baz";

    let actual = doc.format(&PrettyPrinter::default()).unwrap();
    assert_eq!(actual, expected);
}

#[test]
pub fn test_add_alignment_to_doc() {
    let doc = concat([
        hardline(),
        "case SomePattern".into(),
        add_alignment_to_doc(
            concat([
                hardline(),
                "| SomeValue".into(),
                hardline(),
                "| SomeOtherValue".into(),
                hardline(),
                "| SomeOtherOtherValue".into(),
            ]),
            9,
            NonZeroUsize::new(4).unwrap(),
        ),
    ]);
    let expected =
        "\ncase SomePattern\n\t\t | SomeValue\n\t\t | SomeOtherValue\n\t\t | SomeOtherOtherValue";
    let options = PrettyPrinterBuilder::default()
        .use_tabs(true)
        .build()
        .unwrap();
    let actual = doc.format(&options).unwrap();
    assert_eq!(actual, expected);

    // test with multiple nested add_alignment_to_doc's
    let doc = concat([
        hardline(),
        "case SomePattern".into(),
        add_alignment_to_doc(
            add_alignment_to_doc(
                concat([
                    hardline(),
                    "| SomeValue".into(),
                    hardline(),
                    "| SomeOtherValue".into(),
                    hardline(),
                    "| SomeOtherOtherValue".into(),
                ]),
                6,
                NonZeroUsize::new(4).unwrap(),
            ),
            6,
            NonZeroUsize::new(4).unwrap(),
        ),
    ]);
    let expected =
        "\ncase SomePattern\n\t  | SomeValue\n\t  | SomeOtherValue\n\t  | SomeOtherOtherValue";
    let actual = doc.format(&options).unwrap();
    assert_eq!(actual, expected);
}

#[test]
fn test_label() {
    let doc = concat([
        hardline(),
        "case SomePattern".into(),
        label(
            "foo".into(),
            concat([
                hardline(),
                "| SomeValue".into(),
                hardline(),
                "| SomeOtherValue".into(),
                hardline(),
                "| SomeOtherOtherValue".into(),
            ]),
        ),
    ]);
    let expected = "\ncase SomePattern\n| SomeValue\n| SomeOtherValue\n| SomeOtherOtherValue";
    let actual = doc.format(&PrettyPrinter::default()).unwrap();
    assert_eq!(actual, expected);
}
