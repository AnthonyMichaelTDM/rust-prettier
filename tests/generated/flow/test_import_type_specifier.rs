#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_f5049ed4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nimport { Foo, type Baz } from \"../module\";\nimport type {} from 'foo';\n\nimport type {somethingSuperLongsomethingSuperLong} from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'\nimport type {a, somethingSuperLongsomethingSuperLong} from 'somethingSuperLongsomethingSuperLongsomethingSuperLong'\n\nimport transformRouterContext, { type TransformedContextRouter } from '../../helpers/transformRouterContext';") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nimport { Foo, type Baz } from \"../module\";\nimport type {} from \"foo\";\n\nimport type { somethingSuperLongsomethingSuperLong } from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";\nimport type {\n  a,\n  somethingSuperLongsomethingSuperLong,\n} from \"somethingSuperLongsomethingSuperLongsomethingSuperLong\";\n\nimport transformRouterContext, {\n  type TransformedContextRouter,\n} from \"../../helpers/transformRouterContext\";");
    Ok(())
}
