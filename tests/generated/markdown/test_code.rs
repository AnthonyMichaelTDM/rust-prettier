#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_0_indent_js_md_prose_wrapalways_format_1_221118de() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("- 1\n  - 2\n    - 3\n      ```js\n      md`\n      # this is the root indent\n\n      # this is the root indent\n\n      # this is the root indent\n      `\n\n      something`\n      asd\n\n      asd\n\n      asd\n      `\n      ```") ? ;
    assert_eq ! (formatted , "- 1\n\n  - 2\n\n    - 3\n\n      ```js\n      md`\n      # this is the root indent\n\n      # this is the root indent\n\n      # this is the root indent\n      `;\n\n      something`\n      asd\n      \n      asd\n      \n      asd\n      `;\n      ```");
    Ok(())
}
#[test]
fn test_additional_space_md_prose_wrapalways_format_1_807673d4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("1. Fork the repo and create your branch from `master`.\n\n   Open terminal (e.g. Terminal, iTerm, Git Bash or Git Shell) and type:\n   ```sh\n   git clone https://github.com/<your_username>/jest\n   cd jest\n   git checkout -b my_branch\n   ```\n   Note:\n   Replace `<your_username>` with your GitHub username\n\n3. Run `yarn install`.\n   On Windows:\n   To install [Yarn](https://yarnpkg.com/en/docs/install#windows-tab) on Windows you may need to download either node.js or Chocolatey<br />\n\n    ```sh\n    yarn install\n    ```\n    To check your version of Yarn and ensure it's installed you can type:\n    ```sh\n    yarn --version\n    ``\\") ? ;
    assert_eq ! (formatted , "1. Fork the repo and create your branch from `master`.\n\n   Open terminal (e.g. Terminal, iTerm, Git Bash or Git Shell) and type:\n\n   ```sh\n   git clone https://github.com/<your_username>/jest\n   cd jest\n   git checkout -b my_branch\n   ```\n\n   Note: Replace `<your_username>` with your GitHub username\n\n2. Run `yarn install`. On Windows: To install\n   [Yarn](https://yarnpkg.com/en/docs/install#windows-tab) on Windows you may\n   need to download either node.js or Chocolatey<br />\n\n   ```sh\n   yarn install\n   ```\n\n   To check your version of Yarn and ensure it's installed you can type:\n\n   ```sh\n   yarn --version\n   ```");
    Ok(())
}
#[test]
fn test_backtick_md_prose_wrapalways_format_1_81c24c48() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("``````````\n\n```js\nconsole.log(\"hello world!\");\n```\n\n``````````")?;
    assert_eq!(
        formatted,
        "````\n\n```js\nconsole.log(\"hello world!\");\n```\n\n````"
    );
    Ok(())
}
#[test]
fn test_format_md_prose_wrapalways_format_1_fdaae407() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("```js\nconst foo      = 'bar'\n\n\n\n   console     .log(    213    )\n```")?;
    assert_eq!(
        formatted,
        "```js\nconst foo = \"bar\";\n\nconsole.log(213);\n```"
    );
    Ok(())
}
#[test]
fn test_indent_md_prose_wrapalways_format_1_90617716() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("    Indented Code Block\n    Indented Code Block\n    Indented Code Block\n    Indented Code Block\n    Indented Code Block\n\n- ```\n  Fenced Code Block\n  Fenced Code Block\n  Fenced Code Block\n  Fenced Code Block\n  Fenced Code Block\n  ```\n\n<!-- prettier/prettier#3459 -->\n\n1. Change to your home directory:\n\n        cd\n\n2. List the contents:\n\n        ls -l") ? ;
    assert_eq ! (formatted , "    Indented Code Block\n    Indented Code Block\n    Indented Code Block\n    Indented Code Block\n    Indented Code Block\n\n- ```\n  Fenced Code Block\n  Fenced Code Block\n  Fenced Code Block\n  Fenced Code Block\n  Fenced Code Block\n  ```\n\n<!-- prettier/prettier#3459 -->\n\n1.  Change to your home directory:\n\n        cd\n\n2.  List the contents:\n\n        ls -l");
    Ok(())
}
#[test]
fn test_lang_md_prose_wrapalways_format_1_a3796652() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("```js\nconsole.log(\"hello world\");\n```")?;
    assert_eq!(formatted, "```js\nconsole.log(\"hello world\");\n```");
    Ok(())
}
#[test]
fn test_leading_trailing_newlines_md_prose_wrapalways_format_1_a01864cc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("```\n\n\n123\n\n\n```")?;
    assert_eq!(formatted, "```\n\n\n123\n\n\n```");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_4aca1f8e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("```\nhello world\n```")?;
    assert_eq!(formatted, "```\nhello world\n```");
    Ok(())
}
#[test]
fn test_ts_trailing_comma_md_prose_wrapalways_format_1_91ee7e96() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("```ts\nconst test = <T,>(value: T) => {};\n```\n\n```typescript\nconst test = <T,>(value: T) => {};\n```") ? ;
    assert_eq ! (formatted , "```ts\nconst test = <T>(value: T) => {};\n```\n\n```typescript\nconst test = <T>(value: T) => {};\n```");
    Ok(())
}
#[test]
fn test_tsx_trailing_comma_md_prose_wrapalways_format_1_c0fcb7a5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("```tsx\nconst test = <T,>(value: T) => {};\n```")?;
    assert_eq!(formatted, "```tsx\nconst test = <T,>(value: T) => {};\n```");
    Ok(())
}
