#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_cf40e678() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction filterOutVoids<T> (arr: Array<?T>): Array<T> {\n  return arr.filter(Boolean)\n}\n\nfunction filterOutSmall (arr: Array<?number>): Array<?number> {\n  return arr.filter(num => num && num > 10)\n}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction filterOutVoids<T>(arr: Array<?T>): Array<T> {\n  return arr.filter(Boolean);\n}\n\nfunction filterOutSmall(arr: Array<?number>): Array<?number> {\n  return arr.filter((num) => num && num > 10);\n}");
    Ok(())
}
#[test]
fn test_test_2_js_format_1_b3204634() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction filterItems(items: Array<string|number>): Array<string|number> {\n  return items.map(item => {\n    if (typeof item === 'string') {\n      return item.length > 2 ? item : null;\n    } else {\n      return item*10;\n    }\n  }).filter(Boolean);\n}\n\nconst filteredItems = filterItems(['foo', 'b', 1, 2]);\n\nconsole.log(filteredItems);") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction filterItems(items: Array<string | number>): Array<string | number> {\n  return items\n    .map((item) => {\n      if (typeof item === \"string\") {\n        return item.length > 2 ? item : null;\n      } else {\n        return item * 10;\n      }\n    })\n    .filter(Boolean);\n}\n\nconst filteredItems = filterItems([\"foo\", \"b\", 1, 2]);\n\nconsole.log(filteredItems);");
    Ok(())
}
