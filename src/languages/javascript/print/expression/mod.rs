use anyhow::Result;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsName, AnyJsObjectMember, AnyJsObjectMemberName,
    JsCallArgumentList,
};
use biome_rowan::AstNode as _;

use crate::{
    document::{builders::*, Doc},
    PrettyPrinter,
};

use self::array::print_array_expression;

use super::Cache;

mod array;

pub fn print_expression(
    expression: &AnyJsExpression,
    options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    if let Some(doc) = cache.get(expression.syntax()) {
        return Ok(doc.clone());
    }
    let doc: Doc = match expression {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.to_owned().trim_trivia().unwrap().text().into()
        }
        AnyJsExpression::JsArrayExpression(array) => print_array_expression(array, options, cache)?,
        AnyJsExpression::JsArrowFunctionExpression(_) => todo!(),
        AnyJsExpression::JsAssignmentExpression(_) => todo!(),
        AnyJsExpression::JsAwaitExpression(_) => todo!(),
        AnyJsExpression::JsBinaryExpression(_) => todo!(),
        AnyJsExpression::JsBogusExpression(_) => todo!(),
        AnyJsExpression::JsCallExpression(call) => {
            let callee = print_expression(&call.callee()?, options, cache)?;

            concat([
                callee,
                print_call_args(&call.arguments()?.args(), options, cache)?,
                call.optional_chain_token()
                    .map_or("".into(), |_| "?".into()),
            ])
        }
        AnyJsExpression::JsClassExpression(_) => todo!(),
        AnyJsExpression::JsComputedMemberExpression(_) => todo!(),
        AnyJsExpression::JsConditionalExpression(_) => todo!(),
        AnyJsExpression::JsFunctionExpression(_) => todo!(),
        AnyJsExpression::JsIdentifierExpression(ident) => {
            ident.name()?.trim_trivia().unwrap().text().into()
        }
        AnyJsExpression::JsImportCallExpression(_) => todo!(),
        AnyJsExpression::JsImportMetaExpression(_) => todo!(),
        AnyJsExpression::JsInExpression(_) => todo!(),
        AnyJsExpression::JsInstanceofExpression(_) => todo!(),
        AnyJsExpression::JsLogicalExpression(logical_expr) => concat([
            print_expression(&logical_expr.left()?, options, cache)?,
            " ".into(),
            logical_expr.operator_token()?.text_trimmed().into(),
            " ".into(),
            print_expression(&logical_expr.right()?, options, cache)?,
        ]),
        AnyJsExpression::JsNewExpression(new_expr) => concat([
            "new ".into(),
            print_expression(&new_expr.callee()?, options, cache)?,
            if let Some(args) = new_expr.arguments() {
                print_call_args(&args.args(), options, cache)?
            } else {
                "".into()
            },
        ]),
        AnyJsExpression::JsNewTargetExpression(_) => todo!(),
        AnyJsExpression::JsObjectExpression(obj_expr) => group(
            concat([
                "{".into(),
                obj_expr
                    .members()
                    .into_iter()
                    .filter_map(|member| member.ok())
                    .map(|member| print_object_member(&member, options, cache))
                    .collect::<Result<Vec<_>>>()
                    .and_then(|members| {
                        if members.is_empty() {
                            Ok("".into())
                        } else {
                            Ok(concat([
                                if_break(
                                    indent(softline()),
                                    if options.bracket_spacing {
                                        Some(" ".into())
                                    } else {
                                        None
                                    },
                                    None,
                                ),
                                join(
                                    &concat([
                                        ",".into(),
                                        if_break(softline(), Some(" ".into()), None),
                                    ]),
                                    members,
                                ),
                                if_break(
                                    dedent(softline()),
                                    if options.bracket_spacing {
                                        Some(" ".into())
                                    } else {
                                        None
                                    },
                                    None,
                                ),
                            ]))
                        }
                    })?,
                "}".into(),
            ]),
            None,
            false,
            None,
        ),
        AnyJsExpression::JsParenthesizedExpression(_) => todo!(),
        AnyJsExpression::JsPostUpdateExpression(_) => todo!(),
        AnyJsExpression::JsPreUpdateExpression(_) => todo!(),
        AnyJsExpression::JsSequenceExpression(_) => todo!(),
        AnyJsExpression::JsStaticMemberExpression(static_member_expr) => concat([
            print_expression(&static_member_expr.object()?, options, cache)?,
            static_member_expr.operator_token()?.text_trimmed().into(),
            match static_member_expr.member()? {
                AnyJsName::JsName(name) => name.trim_trivia().unwrap().text().into(),
                AnyJsName::JsPrivateName(_) => todo!(),
            },
        ]),
        AnyJsExpression::JsSuperExpression(_) => todo!(),
        AnyJsExpression::JsTemplateExpression(template) => concat([
            "`".into(),
            template
                .elements()
                .into_iter()
                .map(|element| match element {
                    biome_js_syntax::AnyJsTemplateElement::JsTemplateChunkElement(elem) => {
                        Ok(elem.template_chunk_token()?.text_trimmed().into())
                    }
                    biome_js_syntax::AnyJsTemplateElement::JsTemplateElement(elem) => Ok(concat([
                        "${".into(),
                        print_expression(&elem.expression()?, options, cache)?,
                        "}".into(),
                    ])),
                })
                .collect::<Result<Vec<_>>>()
                .and_then(|elements| {
                    if elements.is_empty() {
                        Ok("".into())
                    } else {
                        Ok(concat(elements))
                    }
                })?,
            "`".into(),
        ]),
        AnyJsExpression::JsThisExpression(_) => todo!(),
        AnyJsExpression::JsUnaryExpression(_) => todo!(),
        AnyJsExpression::JsYieldExpression(_) => todo!(),
        AnyJsExpression::JsxTagExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsAsExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsInstantiationExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsNonNullAssertionExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsSatisfiesExpression(_) => unimplemented!("not JS"),
        AnyJsExpression::TsTypeAssertionExpression(_) => unimplemented!("not JS"),
    };

    cache.insert(expression.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_call_args(
    args: &JsCallArgumentList,
    options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    if let Some(doc) = cache.get(args.syntax()) {
        return Ok(doc.clone());
    }

    let doc = group(
        concat([
            "(".into(),
            args.into_iter()
                .filter_map(|arg| arg.ok())
                .map(|arg| match arg {
                    AnyJsCallArgument::AnyJsExpression(expr) => {
                        print_expression(&expr, options, cache)
                    }
                    AnyJsCallArgument::JsSpread(spread) => Ok(concat([
                        "...".into(),
                        print_expression(&spread.argument().unwrap(), options, cache)?,
                    ])),
                })
                .collect::<Result<Vec<_>>>()
                .and_then(|args| {
                    if args.is_empty() {
                        Ok("".into())
                    } else if args.len() == 1 {
                        Ok(args.first().unwrap().clone())
                    } else {
                        Ok(concat([
                            indent(softline()),
                            join(
                                &concat([",".into(), if_break(softline(), Some(" ".into()), None)]),
                                args,
                            ),
                            dedent(softline()),
                        ]))
                    }
                })?,
            ")".into(),
        ]),
        None,
        false,
        None,
    );

    cache.insert(args.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_object_member(
    member: &AnyJsObjectMember,
    options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    if let Some(doc) = cache.get(member.syntax()) {
        return Ok(doc.clone());
    }
    let doc: Doc = match member {
        AnyJsObjectMember::JsBogusMember(_) => todo!(),
        AnyJsObjectMember::JsGetterObjectMember(_) => todo!(),
        AnyJsObjectMember::JsMethodObjectMember(_) => todo!(),
        AnyJsObjectMember::JsPropertyObjectMember(prop) => {
            let key = match prop.name()? {
                AnyJsObjectMemberName::JsComputedMemberName(name) => concat([
                    "[".into(),
                    print_expression(&name.expression()?, options, cache)?,
                    "]".into(),
                ]),
                AnyJsObjectMemberName::JsLiteralMemberName(name) => name.name()?.text().into(),
            };

            let value = print_expression(&prop.value()?, options, cache)?;

            concat([key, ": ".into(), value])
        }
        AnyJsObjectMember::JsSetterObjectMember(_) => todo!(),
        AnyJsObjectMember::JsShorthandPropertyObjectMember(name) => {
            name.name()?.trim_trivia().unwrap().text().into()
        }
        AnyJsObjectMember::JsSpread(spread) => concat([
            "...".into(),
            print_expression(&spread.argument()?, options, cache)?,
        ]),
    };

    cache.insert(member.syntax().clone(), doc.clone());

    Ok(doc)
}
