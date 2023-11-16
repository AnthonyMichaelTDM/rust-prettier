#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_align_md_prose_wrapalways_format_1_7a34fd09() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("|a|b|c|\n|:--|:-:|--:|\n|d|e|f|");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "| a   |  b  |   c |\n| :-- | :-: | --: |\n| d   |  e  |   f |"
    );
}
#[test]
fn test_cjk_md_prose_wrapalways_format_1_22657ffc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("| abc | def | ghi |\n| --- | --- | --- |\n| 第一欄 | 第二欄 | 第三欄 |");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "| abc    | def    | ghi    |\n| ------ | ------ | ------ |\n| 第一欄 | 第二欄 | 第三欄 |"
    );
}
#[test]
fn test_emoji_md_prose_wrapalways_format_1_81115177() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("| abc | def | ghi |\n| --- | --- | --- |\n| 👍👍👍 | 👍👍👍 | 👍👍👍 |");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "| abc    | def    | ghi    |\n| ------ | ------ | ------ |\n| 👍👍👍 | 👍👍👍 | 👍👍👍 |"
    );
}
#[test]
fn test_empty_md_prose_wrapalways_format_1_9a632ba2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("Foo | Bar\n--- | ---\nX   |\nY   |");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "| Foo | Bar |\n| --- | --- |\n| X   |\n| Y   |");
}
#[test]
fn test_escape_md_prose_wrapalways_format_1_28e62ed2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("| a | b | c |\n|:--|:-:|--:|\n| \\\\| | \\\\| | \\\\| |");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "| a   |  b  |   c |\n| :-- | :-: | --: |\n| \\\\|  | \\\\|  |  \\\\| |"
    );
}
#[test]
fn test_html_md_prose_wrapalways_format_1_c4454d7b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("Default | CLI Override | API Override\n--------|--------------|-------------\n\\`\"none\"\\` | <code>--trailing-comma <none&#124;es5&#124;all></code> | <code>trailingComma: \"<none&#124;es5&#124;all>\"</code>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "| Default  | CLI Override                                           | API Override                                           |\n| -------- | ------------------------------------------------------ | ------------------------------------------------------ |\n| \\`\"none\"\\` | <code>--trailing-comma <none&#124;es5&#124;all></code> | <code>trailingComma: \"<none&#124;es5&#124;all>\"</code> |");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_ccc00552() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "| Title A | Title B | Title C |\n|---|---|---|\n| content A | content B | content C |",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "| Title A   | Title B   | Title C   |\n| --------- | --------- | --------- |\n| content A | content B | content C |");
}
#[test]
fn test_table_md_prose_wrapalways_format_1_cd8c5e26() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("- min-table\n\n  | Age | Time | Food | Gold | Requirement |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age | 02:10 | 500 | 0 | Dark Age building x 2 |\n  | Castle Age | 02:40 | 800 | 200 |- |\n  | Imperial Age | 03:30 | 1000 | 800 | Castle Age building x 2 |\n- big-table\n\n  |学号|姓名|分数|\n  |-|-|-|\n  |小明|男|75|\n  |小红|女|79|\n  |小陆|男|92|\n\n| col1 | col2 | col3 |\n|---|--|--|\n| long text | \\`\\` | text |") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "- min-table\n\n  | Age          | Time  | Food | Gold | Requirement             |\n  | ------------ | ----- | ---- | ---- | ----------------------- |\n  | Feudal Age   | 02:10 | 500  | 0    | Dark Age building x 2   |\n  | Castle Age   | 02:40 | 800  | 200  | -                       |\n  | Imperial Age | 03:30 | 1000 | 800  | Castle Age building x 2 |\n\n- big-table\n\n  | 学号 | 姓名 | 分数 |\n  | ---- | ---- | ---- |\n  | 小明 | 男   | 75   |\n  | 小红 | 女   | 79   |\n  | 小陆 | 男   | 92   |\n\n| col1      | col2 | col3 |\n| --------- | ---- | ---- |\n| long text | \\`\\`   | text |");
}
