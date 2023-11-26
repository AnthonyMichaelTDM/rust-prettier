use anyhow::Result;
use biome_js_syntax::{AnyJsAssignmentPattern, AnyJsForInOrOfInitializer, JsForOfStatement};
use biome_rowan::AstNode as _;

use crate::{
    document::{builders::*, Doc},
    languages::javascript::print::expression::print_expression,
    PrettyPrinter,
};

use super::{print_statement, Cache};

pub fn print_for_of_statement(
    statement: &JsForOfStatement,
    options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    if let Some(doc) = cache.get(statement.syntax()) {
        return Ok(doc.clone());
    }

    let doc = concat([
        "for ".into(),
        "(".into(),
        print_for_in_or_of_initializer(&statement.initializer()?, options, cache)?,
        " of ".into(),
        print_expression(&statement.expression()?, options, cache)?,
        ")".into(),
        " ".into(),
        print_statement(&statement.body()?, options, cache)?,
    ]);

    cache.insert(statement.syntax().clone(), doc.clone());

    Ok(doc)
}

fn print_for_in_or_of_initializer(
    declaration: &AnyJsForInOrOfInitializer,
    options: &PrettyPrinter,
    cache: &mut Cache,
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
                if let Some(_annotation) = declarator.variable_annotation() {
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

#[allow(unreachable_code)]
fn print_js_assignment_pattern(
    assignment: &AnyJsAssignmentPattern,
    _options: &PrettyPrinter,
    cache: &mut Cache,
) -> Result<Doc> {
    if let Some(doc) = cache.get(assignment.syntax()) {
        return Ok(doc.clone());
    }
    let _doc: Doc = match assignment {
        AnyJsAssignmentPattern::AnyJsAssignment(_) => todo!(),
        AnyJsAssignmentPattern::JsArrayAssignmentPattern(_) => todo!(),
        AnyJsAssignmentPattern::JsObjectAssignmentPattern(_) => todo!(),
    };

    cache.insert(assignment.syntax().clone(), _doc.clone());

    Ok(_doc)
}
