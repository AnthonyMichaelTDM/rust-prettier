#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_type_modifier_ts_format_1_bc36ca7d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export type { SomeThing };\nexport type { A as B };\nexport type { B as C } from './a';\nexport type { foo } from 'bar';\nexport type { foo };\n\n// this should be treated as a normal import statement\nimport type from './foo';\n\nimport type { SomeThing } from \"./some-module.js\";\nimport type { foo, bar } from 'baz';\nimport type { foo as bar } from 'baz';\nimport type * as foo from './bar';\nimport type foo from 'bar';\nimport type foo, { bar } from 'bar';") ? ;
    assert_eq ! (formatted , "export type { SomeThing };\nexport type { A as B };\nexport type { B as C } from \"./a\";\nexport type { foo } from \"bar\";\nexport type { foo };\n\n// this should be treated as a normal import statement\nimport type from \"./foo\";\n\nimport type { SomeThing } from \"./some-module.js\";\nimport type { foo, bar } from \"baz\";\nimport type { foo as bar } from \"baz\";\nimport type * as foo from \"./bar\";\nimport type foo from \"bar\";\nimport type foo, { bar } from \"bar\";");
    Ok(())
}
