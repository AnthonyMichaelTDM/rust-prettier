use anyhow::Result;
use biome_js_syntax::{AnyJsFormalParameter, AnyJsParameter, JsFunctionDeclaration};
use biome_rowan::AstNode as _;

use crate::{
    document::{builders::*, Doc},
    languages::javascript::print::expression::print_expression,
    PrettyPrinter,
};

use super::{print_statement, Cache};

pub fn print_function_declaration(
    function: &JsFunctionDeclaration,
    options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    if let Some(doc) = cache.get(function.syntax()) {
        return Ok(doc.clone());
    }

    let doc = {
        // get params
        let params = function
            .parameters()?
            .items()
            .into_iter()
            .filter_map(|item| match item.ok()? {
                AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsBogusParameter(_)) => {
                    todo!()
                }
                AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
                    param,
                )) => {
                    let param_name = param.binding().ok()?.trim_trivia()?.text().into();

                    if let Some(initializer) =
                        param.initializer().and_then(|init| init.expression().ok())
                    {
                        Some(concat([
                            param_name,
                            " = ".into(),
                            print_expression(&initializer, options, cache).ok()?,
                        ]))
                    } else {
                        Some(param_name)
                    }
                }
                AnyJsParameter::JsRestParameter(_) => todo!(),
                AnyJsParameter::TsThisParameter(_) => {
                    unimplemented!("TS, not JS")
                }
            })
            .collect::<Vec<_>>();

        let body = group(
            concat([
                "{".into(),
                function
                    .body()?
                    .statements()
                    .into_iter()
                    .map(|statement| print_statement(&statement, options, cache))
                    .collect::<Result<Vec<_>>>()
                    .and_then(|statements| {
                        if statements.is_empty() {
                            Ok(" ".into())
                        } else {
                            Ok(concat([
                                indent(hardline()),
                                join(&hardline(), statements),
                                dedent(hardline()),
                            ]))
                        }
                    })?,
                "}".into(),
            ]),
            None,
            false,
            None,
        );
        let doc = concat([
            format!(
                "{}function {name}",
                function.async_token().map_or("", |_| "async "),
                name = function.id()?.trim_trivia().unwrap().text()
            )
            .into(),
            if !params.is_empty() {
                group(
                    concat([
                        "(".into(),
                        indent(softline()),
                        join(
                            &concat([",".into(), if_break(softline(), Some(" ".into()), None)]),
                            params,
                        ),
                        dedent(softline()),
                        ")".into(),
                    ]),
                    None,
                    false,
                    None,
                )
            } else {
                "()".into()
            },
            " ".into(),
            body,
        ]);

        doc
    };

    cache.insert(function.syntax().clone(), doc.clone());

    Ok(doc)
}
