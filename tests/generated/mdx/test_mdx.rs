#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_ignore_mdx_semifalse_format_1_6696b60b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("An ignore comment (HTML):\n\n<!-- prettier-ignore -->\n\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```\n\nAn ignore comment (ES):\n\n{/* prettier-ignore */}\n\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```\n\nNo comment:\n\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```") ? ;
    assert_eq ! (formatted , "An ignore comment (HTML):\n\n<!-- prettier-ignore -->\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```\n\nAn ignore comment (ES):\n\n{/* prettier-ignore */}\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```\n\nNo comment:\n\n```js\nfoo(\n  reallyLongArg(),\n  omgSoManyParameters(),\n  IShouldRefactorThis(),\n  isThereSeriouslyAnotherOne(),\n)\n```");
    Ok(())
}
#[test]
fn test_ignore_mdx_format_1_6696b60b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("An ignore comment (HTML):\n\n<!-- prettier-ignore -->\n\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```\n\nAn ignore comment (ES):\n\n{/* prettier-ignore */}\n\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```\n\nNo comment:\n\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```") ? ;
    assert_eq ! (formatted , "An ignore comment (HTML):\n\n<!-- prettier-ignore -->\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```\n\nAn ignore comment (ES):\n\n{/* prettier-ignore */}\n```js\nfoo(reallyLongArg(), omgSoManyParameters(), IShouldRefactorThis(), isThereSeriouslyAnotherOne());\n```\n\nNo comment:\n\n```js\nfoo(\n  reallyLongArg(),\n  omgSoManyParameters(),\n  IShouldRefactorThis(),\n  isThereSeriouslyAnotherOne(),\n);\n```");
    Ok(())
}
#[test]
fn test_import_export_mdx_semifalse_format_1_08365a02() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import D from 'd'\nimport {A,B,C}    from \"hello-world\"\nimport {AAAAAAAAAAAAAAAAAAAAAAAA, BBBBBBBBBBBBBBBBBBBBBB, CCCCCCCCCCCCCCCCCCCCCCC}   from  'xyz';\n\n---\n\nimport D from 'd'\n\n\nimport {A,B,C}    from \"hello-world\"\n\n\nimport {AAAAAAAAAAAAAAAAAAAAAAAA, BBBBBBBBBBBBBBBBBBBBBB, CCCCCCCCCCCCCCCCCCCCCCC}   from  'xyz';\n\n---\n\nexport const meta = {\nauthors: [fred, sue],\nlayout: Layout\n}\n\nexport default () =>\n  <Doc     components={{\n        h1: ui.Heading,\n         p:    ui.Text,\n      code:     ui.Code\n         }}\n      />\n\n---\n\nexport const a = 1;\nexport const b = 1;") ? ;
    assert_eq ! (formatted , "import D from \"d\"\nimport { A, B, C } from \"hello-world\"\nimport {\n  AAAAAAAAAAAAAAAAAAAAAAAA,\n  BBBBBBBBBBBBBBBBBBBBBB,\n  CCCCCCCCCCCCCCCCCCCCCCC,\n} from \"xyz\"\n\n---\n\nimport D from \"d\"\n\nimport { A, B, C } from \"hello-world\"\n\nimport {\n  AAAAAAAAAAAAAAAAAAAAAAAA,\n  BBBBBBBBBBBBBBBBBBBBBB,\n  CCCCCCCCCCCCCCCCCCCCCCC,\n} from \"xyz\"\n\n---\n\nexport const meta = {\n  authors: [fred, sue],\n  layout: Layout,\n}\n\nexport default () => (\n  <Doc\n    components={{\n      h1: ui.Heading,\n      p: ui.Text,\n      code: ui.Code,\n    }}\n  />\n)\n\n---\n\nexport const a = 1\nexport const b = 1");
    Ok(())
}
#[test]
fn test_import_export_mdx_format_1_08365a02() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import D from 'd'\nimport {A,B,C}    from \"hello-world\"\nimport {AAAAAAAAAAAAAAAAAAAAAAAA, BBBBBBBBBBBBBBBBBBBBBB, CCCCCCCCCCCCCCCCCCCCCCC}   from  'xyz';\n\n---\n\nimport D from 'd'\n\n\nimport {A,B,C}    from \"hello-world\"\n\n\nimport {AAAAAAAAAAAAAAAAAAAAAAAA, BBBBBBBBBBBBBBBBBBBBBB, CCCCCCCCCCCCCCCCCCCCCCC}   from  'xyz';\n\n---\n\nexport const meta = {\nauthors: [fred, sue],\nlayout: Layout\n}\n\nexport default () =>\n  <Doc     components={{\n        h1: ui.Heading,\n         p:    ui.Text,\n      code:     ui.Code\n         }}\n      />\n\n---\n\nexport const a = 1;\nexport const b = 1;") ? ;
    assert_eq ! (formatted , "import D from \"d\";\nimport { A, B, C } from \"hello-world\";\nimport {\n  AAAAAAAAAAAAAAAAAAAAAAAA,\n  BBBBBBBBBBBBBBBBBBBBBB,\n  CCCCCCCCCCCCCCCCCCCCCCC,\n} from \"xyz\";\n\n---\n\nimport D from \"d\";\n\nimport { A, B, C } from \"hello-world\";\n\nimport {\n  AAAAAAAAAAAAAAAAAAAAAAAA,\n  BBBBBBBBBBBBBBBBBBBBBB,\n  CCCCCCCCCCCCCCCCCCCCCCC,\n} from \"xyz\";\n\n---\n\nexport const meta = {\n  authors: [fred, sue],\n  layout: Layout,\n};\n\nexport default () => (\n  <Doc\n    components={{\n      h1: ui.Heading,\n      p: ui.Text,\n      code: ui.Code,\n    }}\n  />\n);\n\n---\n\nexport const a = 1;\nexport const b = 1;");
    Ok(())
}
#[test]
fn test_inline_html_mdx_semifalse_format_1_0da8814c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("This is an example of a component _being used in some italic markdown with some <Bolded />, \nand as you can see_ once you close the italics, it will break incorrectly when prettier formats it.\n\n| Column 1 | Column 2 |\n| -- | -- |\n| **`Row 1 Code`** | Some text. |\n| **<code>Row 2 Code</code>** | Some text. |\n| **<InlineCode>Row 2 Code</InlineCode>** | Some text. |") ? ;
    assert_eq ! (formatted , "This is an example of a component _being used in some italic markdown with some <Bolded />,\nand as you can see_ once you close the italics, it will break incorrectly when prettier formats it.\n\n| Column 1                                | Column 2   |\n| --------------------------------------- | ---------- |\n| **`Row 1 Code`**                        | Some text. |\n| **<code>Row 2 Code</code>**             | Some text. |\n| **<InlineCode>Row 2 Code</InlineCode>** | Some text. |");
    Ok(())
}
#[test]
fn test_inline_html_mdx_format_1_0da8814c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("This is an example of a component _being used in some italic markdown with some <Bolded />, \nand as you can see_ once you close the italics, it will break incorrectly when prettier formats it.\n\n| Column 1 | Column 2 |\n| -- | -- |\n| **`Row 1 Code`** | Some text. |\n| **<code>Row 2 Code</code>** | Some text. |\n| **<InlineCode>Row 2 Code</InlineCode>** | Some text. |") ? ;
    assert_eq ! (formatted , "This is an example of a component _being used in some italic markdown with some <Bolded />,\nand as you can see_ once you close the italics, it will break incorrectly when prettier formats it.\n\n| Column 1                                | Column 2   |\n| --------------------------------------- | ---------- |\n| **`Row 1 Code`**                        | Some text. |\n| **<code>Row 2 Code</code>**             | Some text. |\n| **<InlineCode>Row 2 Code</InlineCode>** | Some text. |");
    Ok(())
}
#[test]
fn test_jsx_mdx_semifalse_format_1_4741024f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<Heading hi='there'>Hello, world!\n</Heading>\n\n---\n\n<Hello>\n    test   <World />   test\n</Hello>123\n\n---\n\n<Hello>\n    test   <World />   test\n</Hello>\n<Hello>\n    test   <World />   test\n</Hello>123\n\n---\n\n<Hello>\n    test   <World />   test\n</Hello>       123\n<Hello>\n    test   <World />   test\n</Hello>       234\n\n---\n\n<>\n    test   <World        />   test\n</>       123\n\n---\n\n| Column 1 | Column 2 |\n|---|---|\n| Text | <Hello>Text</Hello> |\n\n---\n\nA {/* JS-style comment */} comment.\n\n{\n  /* Another JS-style comment */\n}") ? ;
    assert_eq ! (formatted , "<Heading hi=\"there\">Hello, world!</Heading>\n\n---\n\n<Hello>\n  test <World /> test\n</Hello>\n123\n\n---\n\n<Hello>\n  test <World /> test\n</Hello>\n<Hello>\n  test <World /> test\n</Hello>123\n\n---\n\n<Hello>\n  test <World /> test\n</Hello> 123\n<Hello>\n  test <World /> test\n</Hello> 234\n\n---\n\n<>\n  test <World /> test\n</> 123\n\n---\n\n| Column 1 | Column 2            |\n| -------- | ------------------- |\n| Text     | <Hello>Text</Hello> |\n\n---\n\nA {/* JS-style comment */} comment.\n\n{/* Another JS-style comment */}");
    Ok(())
}
#[test]
fn test_jsx_mdx_format_1_4741024f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<Heading hi='there'>Hello, world!\n</Heading>\n\n---\n\n<Hello>\n    test   <World />   test\n</Hello>123\n\n---\n\n<Hello>\n    test   <World />   test\n</Hello>\n<Hello>\n    test   <World />   test\n</Hello>123\n\n---\n\n<Hello>\n    test   <World />   test\n</Hello>       123\n<Hello>\n    test   <World />   test\n</Hello>       234\n\n---\n\n<>\n    test   <World        />   test\n</>       123\n\n---\n\n| Column 1 | Column 2 |\n|---|---|\n| Text | <Hello>Text</Hello> |\n\n---\n\nA {/* JS-style comment */} comment.\n\n{\n  /* Another JS-style comment */\n}") ? ;
    assert_eq ! (formatted , "<Heading hi=\"there\">Hello, world!</Heading>\n\n---\n\n<Hello>\n  test <World /> test\n</Hello>\n123\n\n---\n\n<Hello>\n  test <World /> test\n</Hello>\n<Hello>\n  test <World /> test\n</Hello>123\n\n---\n\n<Hello>\n  test <World /> test\n</Hello> 123\n<Hello>\n  test <World /> test\n</Hello> 234\n\n---\n\n<>\n  test <World /> test\n</> 123\n\n---\n\n| Column 1 | Column 2            |\n| -------- | ------------------- |\n| Text     | <Hello>Text</Hello> |\n\n---\n\nA {/* JS-style comment */} comment.\n\n{/* Another JS-style comment */}");
    Ok(())
}
#[test]
fn test_jsx_codeblock_mdx_semifalse_format_1_c5c7ba73() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("```jsx\n<div>foo</div>\n```\n\n```jsx\nconst a = 1;\n<div>foo</div>;\n```")?;
    assert_eq!(
        formatted,
        "```jsx\n<div>foo</div>\n```\n\n```jsx\nconst a = 1\n;<div>foo</div>\n```"
    );
    Ok(())
}
#[test]
fn test_jsx_codeblock_mdx_format_1_c5c7ba73() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("```jsx\n<div>foo</div>\n```\n\n```jsx\nconst a = 1;\n<div>foo</div>;\n```")?;
    assert_eq!(
        formatted,
        "```jsx\n<div>foo</div>\n```\n\n```jsx\nconst a = 1;\n<div>foo</div>;\n```"
    );
    Ok(())
}
#[test]
fn test_levels_mdx_semifalse_format_1_04ed8301() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import     {     Foo,  Bar } from     './Fixture'\n\n# Hello,    world!\n\n<Foo bg='red'>\n   <div style={{   display:   'block'}   }>\n      <Bar    >hi    </Bar>\n       {  hello       }\n       {     /* another comment */}\n       </div>\n</Foo>\n\nasdfsdf <strong style={{fontWeight: 'bolder'}}>asdfasdf</strong>\n\n<Foo/>\ntest\n\n<ul>\n      <li   >item    {' '} </li>\n      <li/>\n        </ul >\n") ? ;
    assert_eq ! (formatted , "import { Foo, Bar } from \"./Fixture\"\n\n# Hello, world!\n\n<Foo bg=\"red\">\n  <div style={{ display: \"block\" }}>\n    <Bar>hi </Bar>\n    {hello}\n    {/* another comment */}\n  </div>\n</Foo>\n\nasdfsdf <strong style={{fontWeight: 'bolder'}}>asdfasdf</strong>\n\n<Foo />\ntest\n\n<ul>\n  <li>item </li>\n  <li />\n</ul>");
    Ok(())
}
#[test]
fn test_levels_mdx_format_1_04ed8301() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import     {     Foo,  Bar } from     './Fixture'\n\n# Hello,    world!\n\n<Foo bg='red'>\n   <div style={{   display:   'block'}   }>\n      <Bar    >hi    </Bar>\n       {  hello       }\n       {     /* another comment */}\n       </div>\n</Foo>\n\nasdfsdf <strong style={{fontWeight: 'bolder'}}>asdfasdf</strong>\n\n<Foo/>\ntest\n\n<ul>\n      <li   >item    {' '} </li>\n      <li/>\n        </ul >\n") ? ;
    assert_eq ! (formatted , "import { Foo, Bar } from \"./Fixture\";\n\n# Hello, world!\n\n<Foo bg=\"red\">\n  <div style={{ display: \"block\" }}>\n    <Bar>hi </Bar>\n    {hello}\n    {/* another comment */}\n  </div>\n</Foo>\n\nasdfsdf <strong style={{fontWeight: 'bolder'}}>asdfasdf</strong>\n\n<Foo />\ntest\n\n<ul>\n  <li>item </li>\n  <li />\n</ul>");
    Ok(())
}
#[test]
fn test_mixed_mdx_semifalse_format_1_7c993d9a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import     {     Baz } from     './Fixture'\nimport { Buz  }   from './Fixture'\n\nexport  const   foo    = {\n  hi:     `Fudge ${Baz.displayName || 'Baz'}`,\n  authors: [\n     'fred',\n           'sally'\n    ]\n}\n\n# Hello,    world!\n\n\n I'm an awesome   paragraph.\n\n<!-- I'm a comment -->\n\n<Foo bg='red'>\n      <Bar    >hi    </Bar>\n       {  hello       }\n       {     /* another comment */}\n</Foo>\n\n```\ntest codeblock\n```\n\n```js\nmodule.exports = 'test'\n```\n\n```sh\nnpm i -g foo\n```\n\n| Test  | Table   |\n|    :---     | :----  |\n|   Col1  | Col2    |\n\nexport   default     ({children   }) => < div>{    children}</div>") ? ;
    assert_eq ! (formatted , "import { Baz } from \"./Fixture\"\nimport { Buz } from \"./Fixture\"\n\nexport const foo = {\n  hi: `Fudge ${Baz.displayName || \"Baz\"}`,\n  authors: [\"fred\", \"sally\"],\n}\n\n# Hello, world!\n\nI'm an awesome paragraph.\n\n<!-- I'm a comment -->\n\n<Foo bg=\"red\">\n  <Bar>hi </Bar>\n  {hello}\n  {/* another comment */}\n</Foo>\n\n```\ntest codeblock\n```\n\n```js\nmodule.exports = \"test\"\n```\n\n```sh\nnpm i -g foo\n```\n\n| Test | Table |\n| :--- | :---- |\n| Col1 | Col2  |\n\nexport default ({ children }) => <div>{children}</div>");
    Ok(())
}
#[test]
fn test_mixed_mdx_format_1_7c993d9a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mdx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import     {     Baz } from     './Fixture'\nimport { Buz  }   from './Fixture'\n\nexport  const   foo    = {\n  hi:     `Fudge ${Baz.displayName || 'Baz'}`,\n  authors: [\n     'fred',\n           'sally'\n    ]\n}\n\n# Hello,    world!\n\n\n I'm an awesome   paragraph.\n\n<!-- I'm a comment -->\n\n<Foo bg='red'>\n      <Bar    >hi    </Bar>\n       {  hello       }\n       {     /* another comment */}\n</Foo>\n\n```\ntest codeblock\n```\n\n```js\nmodule.exports = 'test'\n```\n\n```sh\nnpm i -g foo\n```\n\n| Test  | Table   |\n|    :---     | :----  |\n|   Col1  | Col2    |\n\nexport   default     ({children   }) => < div>{    children}</div>") ? ;
    assert_eq ! (formatted , "import { Baz } from \"./Fixture\";\nimport { Buz } from \"./Fixture\";\n\nexport const foo = {\n  hi: `Fudge ${Baz.displayName || \"Baz\"}`,\n  authors: [\"fred\", \"sally\"],\n};\n\n# Hello, world!\n\nI'm an awesome paragraph.\n\n<!-- I'm a comment -->\n\n<Foo bg=\"red\">\n  <Bar>hi </Bar>\n  {hello}\n  {/* another comment */}\n</Foo>\n\n```\ntest codeblock\n```\n\n```js\nmodule.exports = \"test\";\n```\n\n```sh\nnpm i -g foo\n```\n\n| Test | Table |\n| :--- | :---- |\n| Col1 | Col2  |\n\nexport default ({ children }) => <div>{children}</div>;");
    Ok(())
}
