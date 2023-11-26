use std::collections::HashMap;

use anyhow::Result;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsAssignmentPattern, AnyJsCallArgument, AnyJsExportClause,
    AnyJsExpression, AnyJsForInOrOfInitializer, AnyJsFormalParameter, AnyJsImportClause,
    AnyJsModuleItem, AnyJsName, AnyJsNamedImport, AnyJsNamedImportSpecifier, AnyJsObjectMember,
    AnyJsObjectMemberName, AnyJsParameter, AnyJsStatement, JsCallArgumentList, JsExport, JsImport,
    JsModule, JsScript, JsSyntaxNode,
};
use biome_rowan::{AstNode, AstNodeList};

use crate::{
    common::Symbol,
    document::{
        builders::{
            concat, conditional_group, dedent, fill, group, hardline, if_break, indent, join,
            label, line, softline,
        },
        Doc, DocCommand,
    },
    PrettyPrinter,
};

fn print_export(
    export: &JsExport,
    _options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(export.syntax()) {
        return Ok(doc.clone());
    }
    let doc: Doc = match export.export_clause()? {
        AnyJsExportClause::AnyJsDeclarationClause(_) => todo!(),
        AnyJsExportClause::JsExportDefaultDeclarationClause(_) => todo!(),
        AnyJsExportClause::JsExportDefaultExpressionClause(expr) => concat([
            "export default ".into(),
            print_expression(&expr.expression()?, _options, cache)?,
            ";".into(),
        ]),
        AnyJsExportClause::JsExportFromClause(_) => todo!(),
        AnyJsExportClause::JsExportNamedClause(_) => todo!(),
        AnyJsExportClause::JsExportNamedFromClause(_) => todo!(),
        AnyJsExportClause::TsExportAsNamespaceClause(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsExportClause::TsExportAssignmentClause(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsExportClause::TsExportDeclareClause(_) => {
            unimplemented!("TS, not JS")
        }
    };

    cache.insert(export.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_import(
    import: &JsImport,
    _options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(import.syntax()) {
        return Ok(doc.clone());
    }
    let doc = match import.import_clause().unwrap() {
        AnyJsImportClause::JsImportBareClause(_) => todo!(),
        AnyJsImportClause::JsImportDefaultClause(_) => todo!(),
        AnyJsImportClause::JsImportNamedClause(named_import) => {
            let imports = match named_import.named_import().unwrap() {
                AnyJsNamedImport::JsNamedImportSpecifiers(imports) => imports
                    .specifiers()
                    .into_iter()
                    .filter_map(|specifier| specifier.ok())
                    .map(|specifier| match specifier {
                        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => todo!(),
                        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(_) => todo!(),
                        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(name) => name
                            .local_name()
                            .unwrap()
                            .trim_trivia()
                            .unwrap()
                            .text()
                            .into(),
                    })
                    .collect::<Vec<_>>(),
                AnyJsNamedImport::JsNamespaceImportSpecifier(_) => todo!(),
            };

            let source: Doc = named_import.source()?.trim_trivia().unwrap().text().into();

            let doc = group(
                concat([
                    "import".into(),
                    " {".into(),
                    softline(),
                    indent(join(
                        &concat([",".into(), if_break(softline(), Some(" ".into()), None)]),
                        imports,
                    )),
                    dedent(softline()),
                    "}".into(),
                    " from ".into(),
                    source,
                    ";".into(),
                ]),
                None,
                false,
                None,
            );

            doc
        }
        AnyJsImportClause::JsImportNamespaceClause(_) => todo!(),
    };

    cache.insert(import.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_js_assignment_pattern(
    assignment: &AnyJsAssignmentPattern,
    options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(assignment.syntax()) {
        return Ok(doc.clone());
    }
    let doc: Doc = match assignment {
        AnyJsAssignmentPattern::AnyJsAssignment(_) => todo!(),
        AnyJsAssignmentPattern::JsArrayAssignmentPattern(_) => todo!(),
        AnyJsAssignmentPattern::JsObjectAssignmentPattern(_) => todo!(),
    };

    cache.insert(assignment.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_for_in_or_of_initializer(
    declaration: &AnyJsForInOrOfInitializer,
    options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(declaration.syntax()) {
        return Ok(doc.clone());
    }

    let doc = match declaration {
        AnyJsForInOrOfInitializer::AnyJsAssignmentPattern(assign) => {
            print_js_assignment_pattern(&assign, options, cache)?
        }
        AnyJsForInOrOfInitializer::JsForVariableDeclaration(declaration) => {
            let prefix: Doc = format! {"{}{} ", if declaration.await_token().is_some() { "await " } else { "" }, declaration.kind_token()?.text_trimmed()}.into();
            let declarator = declaration.declarator()?;
            let declarator = concat([
                declarator.id()?.trim_trivia().unwrap().text().into(),
                if let Some(annotation) = declarator.variable_annotation() {
                    todo!()
                } else {
                    "".into()
                },
                if let Some(initializer) = declarator.initializer() {
                    concat([
                        " = ".into(),
                        print_expression(&initializer.expression()?, options, cache)?,
                    ])
                } else {
                    "".into()
                },
            ]);

            concat([prefix, declarator])
        }
    };

    cache.insert(declaration.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_statement(
    statement: &AnyJsStatement,
    options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(statement.syntax()) {
        return Ok(doc.clone());
    }
    let doc = match statement {
        AnyJsStatement::JsBlockStatement(block) => block
            .statements()
            .into_iter()
            .map(|stmt| print_statement(&stmt, options, cache))
            .collect::<Result<Vec<_>>>()
            .and_then(|statements| {
                if statements.is_empty() {
                    Ok(concat(["{".into(), softline(), "}".into()]))
                } else {
                    Ok(concat([
                        "{".into(),
                        indent(hardline()),
                        join(&hardline(), statements),
                        dedent(hardline()),
                        "}".into(),
                    ]))
                }
            })?,
        AnyJsStatement::JsBogusStatement(_) => todo!(),
        AnyJsStatement::JsBreakStatement(_) => todo!(),
        AnyJsStatement::JsClassDeclaration(_) => todo!(),
        AnyJsStatement::JsContinueStatement(_) => todo!(),
        AnyJsStatement::JsDebuggerStatement(_) => todo!(),
        AnyJsStatement::JsDoWhileStatement(_) => todo!(),
        AnyJsStatement::JsEmptyStatement(_) => todo!(),
        AnyJsStatement::JsExpressionStatement(expr) => concat([
            print_expression(&expr.expression().unwrap(), options, cache)?,
            ";".into(),
        ]),
        AnyJsStatement::JsForInStatement(_) => todo!(),
        AnyJsStatement::JsForOfStatement(for_statement) => concat([
            "for ".into(),
            "(".into(),
            print_for_in_or_of_initializer(&for_statement.initializer()?, options, cache)?,
            " of ".into(),
            print_expression(&for_statement.expression()?, options, cache)?,
            ")".into(),
            " ".into(),
            print_statement(&for_statement.body()?, options, cache)?,
        ]),
        AnyJsStatement::JsForStatement(_) => todo!(),
        AnyJsStatement::JsFunctionDeclaration(function) => {
            // get params
            let params = function
                .parameters()?
                .items()
                .into_iter()
                .filter_map(|item| match item.ok()? {
                    AnyJsParameter::AnyJsFormalParameter(
                        AnyJsFormalParameter::JsBogusParameter(_),
                    ) => todo!(),
                    AnyJsParameter::AnyJsFormalParameter(
                        AnyJsFormalParameter::JsFormalParameter(param),
                    ) => {
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
        }
        AnyJsStatement::JsIfStatement(_) => todo!(),
        AnyJsStatement::JsLabeledStatement(_) => todo!(),
        AnyJsStatement::JsReturnStatement(_) => todo!(),
        AnyJsStatement::JsSwitchStatement(_) => todo!(),
        AnyJsStatement::JsThrowStatement(_) => todo!(),
        AnyJsStatement::JsTryFinallyStatement(_) => todo!(),
        AnyJsStatement::JsTryStatement(_) => todo!(),
        AnyJsStatement::JsVariableStatement(stmt) => {
            let stmt = stmt.declaration()?;
            let prefix: Doc = format! {"{async_}{kind} ",async_ = if stmt.await_token().is_some() { "await " } else { "" }, kind = stmt.kind()?.text_trimmed()}.into();
            let declarations = stmt
                .declarators()
                .into_iter()
                .filter_map(|decl| decl.ok())
                .map(|decl| {
                    let name: Doc = decl.id()?.trim_trivia().unwrap().text().into();

                    Ok((name, decl))
                })
                .collect::<Result<Vec<_>>>()
                .and_then(|declarations| {
                    if declarations.is_empty() {
                        Ok(" ".into())
                    } else if declarations.len() == 1 {
                        Ok(fill([
                            concat([declarations.first().unwrap().0.clone(), " =".into()]),
                            if_break(indent(softline()), Some(" ".into()), None),
                            print_expression(
                                &declarations
                                    .first()
                                    .unwrap()
                                    .1
                                    .initializer()
                                    .unwrap()
                                    .expression()?,
                                options,
                                cache,
                            )?,
                        ]))
                    } else {
                        Ok(indent(join(
                            &concat([",".into(), if_break(softline(), Some(" ".into()), None)]),
                            declarations
                                .iter()
                                .map(|(name, decl)| {
                                    Ok(group(
                                        concat([
                                            name.clone(),
                                            " = ".into(),
                                            print_expression(
                                                &decl.initializer().unwrap().expression()?,
                                                options,
                                                cache,
                                            )?,
                                        ]),
                                        None,
                                        false,
                                        None,
                                    ))
                                })
                                .collect::<Result<Vec<_>>>()?,
                        )))
                    }
                })?;

            let doc = group(
                concat([prefix, declarations, dedent(";".into())]),
                None,
                false,
                None,
            );

            doc
        }
        AnyJsStatement::JsWhileStatement(_) => todo!(),
        AnyJsStatement::JsWithStatement(_) => todo!(),
        AnyJsStatement::TsDeclareFunctionDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsDeclareStatement(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsEnumDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsExternalModuleDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsGlobalDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsImportEqualsDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsInterfaceDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsModuleDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
        AnyJsStatement::TsTypeAliasDeclaration(_) => {
            unimplemented!("TS, not JS")
        }
    };

    cache.insert(statement.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_call_args(
    args: &JsCallArgumentList,
    options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
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

fn print_array_element(
    element: &AnyJsArrayElement,
    options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
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

fn print_object_member(
    member: &AnyJsObjectMember,
    options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
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

fn print_expression(
    expression: &AnyJsExpression,
    options: &PrettyPrinter,
    cache: &mut HashMap<JsSyntaxNode, Doc>,
) -> Result<Doc> {
    if let Some(doc) = cache.get(expression.syntax()) {
        return Ok(doc.clone());
    }
    let doc: Doc = match expression {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.to_owned().trim_trivia().unwrap().text().into()
        }
        AnyJsExpression::JsArrayExpression(arr) => {
            let array_elements = arr
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
        }
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

pub(crate) fn print_module(js_module: JsModule, options: &PrettyPrinter) -> Result<Doc> {
    fn print_module_recursive(
        item: &AnyJsModuleItem,
        options: &PrettyPrinter,
        cache: &mut HashMap<JsSyntaxNode, Doc>,
    ) -> Result<Doc> {
        // check if we have already printed this node
        if let Some(doc) = cache.get(item.syntax()) {
            return Ok(doc.clone());
        }

        // if not, process it
        let result = process(item, options, cache);
        if let Ok(result) = &result {
            // and cache the result
            cache.insert(item.syntax().clone(), result.clone());
        }
        result
    }
    fn process(
        item: &AnyJsModuleItem,
        options: &PrettyPrinter,
        cache: &mut HashMap<JsSyntaxNode, Doc>,
    ) -> Result<Doc> {
        match item {
            AnyJsModuleItem::JsExport(export) => print_export(export, options, cache),
            AnyJsModuleItem::JsImport(import) => print_import(import, options, cache),
            AnyJsModuleItem::AnyJsStatement(statement) => {
                print_statement(statement, options, cache)
            }
        }
    }

    // Within an abstract syntax tree, the same subtrees can be found multiple times.
    // As an optimization (those subtrees can be huge) and to maintain the
    // reference structure of the tree, the results are cached in
    // a map and reused if they are encountered again.
    let mut cache = HashMap::new();

    let imports = js_module
        .items()
        .iter()
        .filter(|item| matches!(item, AnyJsModuleItem::JsImport(_)))
        .map(|ref item| print_module_recursive(item, options, &mut cache))
        .collect::<Result<Vec<_>>>()
        .and_then(|imports| {
            if !imports.is_empty() {
                Ok(concat([join(&hardline(), imports), hardline(), hardline()]))
            } else {
                Ok("".into())
            }
        })?;

    let statements = js_module
        .items()
        .iter()
        .filter(|item| matches!(item, AnyJsModuleItem::AnyJsStatement(_)))
        .map(|ref item| print_module_recursive(item, options, &mut cache))
        .collect::<Result<Vec<_>>>()
        .and_then(|statements| {
            if !statements.is_empty() {
                Ok(join(&hardline(), statements))
            } else {
                Ok("".into())
            }
        })?;

    let exports = js_module
        .items()
        .iter()
        .filter(|item| matches!(item, AnyJsModuleItem::JsExport(_)))
        .map(|ref item| print_module_recursive(item, options, &mut cache))
        .collect::<Result<Vec<_>>>()
        .and_then(|exports| {
            if !exports.is_empty() {
                Ok(join(&hardline(), exports))
            } else {
                Ok("".into())
            }
        })?;

    let eof = js_module
        .eof_token()
        .map_or(String::new(), |token| token.text().into())
        .into();

    Ok(concat([imports, statements, exports, eof]))
}

pub(crate) fn print_script(_js_script: JsScript, _options: &PrettyPrinter) -> Result<Doc> {
    todo!()
}
