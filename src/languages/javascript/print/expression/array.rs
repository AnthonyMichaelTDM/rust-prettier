use anyhow::Result;
use biome_js_syntax::{AnyJsArrayElement, JsArrayExpression};
use biome_rowan::AstNode as _;

use crate::{
    common::Symbol,
    document::{builders::*, Doc, DocCommand},
    PrettyPrinter,
};

use super::{print_expression, Cache};

pub fn print_array_expression(
    array: &JsArrayExpression,
    options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    if let Some(doc) = cache.get(array.syntax()) {
        return Ok(doc.clone());
    }

    let doc = {
        let array_elements = array
            .elements()
            .into_iter()
            .filter_map(|element| element.ok())
            .map(|element| print_array_element(&element, options, cache))
            .collect::<Result<Vec<_>>>()?;

        let flat = concat([
            "[".into(),
            if array_elements.is_empty() {
                "".into()
            } else {
                let elems = join(&", ".into(), &array_elements);
                if let Some(Doc::DocCommand(DocCommand::Label {
                    label: Symbol::String("array_hole"),
                    ..
                })) = array_elements.last()
                {
                    concat([elems, ",".into()])
                } else {
                    elems
                }
            },
            "]".into(),
        ]);

        let expanded = concat([
            "[".into(),
            if array_elements.is_empty() {
                "".into()
            } else {
                concat([
                    indent(line()),
                    fill(
                        array_elements
                            .iter()
                            .map(|elem| concat([elem.clone(), ",".into()]))
                            .fold(Vec::new(), |mut acc, elem| {
                                if !acc.is_empty() {
                                    acc.push(if_break(line(), Some(" ".into()), None));
                                }
                                acc.push(elem);
                                acc
                            }),
                    ),
                    // join(&", ".into(), &array_elements),
                    dedent(line()),
                ])
            },
            "]".into(),
        ]);

        // let most_expanded = concat([
        //     "[".into(),
        //     if array_elements.is_empty() {
        //         "".into()
        //     } else {
        //         concat([
        //             indent(hardline()),
        //             join(&concat([",".into(), hardline()]), array_elements),
        //             ",".into(),
        //             dedent(hardline()),
        //         ])
        //     },
        //     "]".into(),
        // ]);
        // group(expanded, None, false, None)
        // expanded
        conditional_group([flat, expanded], None, false).unwrap()
    };

    cache.insert(array.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_array_element(
    element: &AnyJsArrayElement,
    options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    if let Some(doc) = cache.get(element.syntax()) {
        return Ok(doc.clone());
    }
    let doc: Doc = match element {
        AnyJsArrayElement::AnyJsExpression(expr) => print_expression(&expr, options, cache)?,
        AnyJsArrayElement::JsSpread(spread) => concat([
            "...".into(),
            print_expression(&spread.argument()?, options, cache)?,
        ]),
        AnyJsArrayElement::JsArrayHole(_) => label(Symbol::from("array_hole"), "".into()),
    };

    cache.insert(element.syntax().clone(), doc.clone());

    Ok(doc)
}
