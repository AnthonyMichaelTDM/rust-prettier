#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_classes_js_quote_propsas_needed_format_1_3ff08b74() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("as-needed")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  a = \"a\"\n};\n\nclass B {\n  'b' = \"b\"\n};\n\nclass C {\n  c1 = \"c1\"\n  'c2' = \"c2\"\n};\n\nclass D {\n  d1 = \"d1\"\n  'd-2' = \"d2\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  a = \"a\";\n}\n\nclass B {\n  b = \"b\";\n}\n\nclass C {\n  c1 = \"c1\";\n  c2 = \"c2\";\n}\n\nclass D {\n  d1 = \"d1\";\n  \"d-2\" = \"d2\";\n}");
}
#[test]
fn test_classes_js_quote_propsconsistent_format_1_3ff08b74() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("consistent")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  a = \"a\"\n};\n\nclass B {\n  'b' = \"b\"\n};\n\nclass C {\n  c1 = \"c1\"\n  'c2' = \"c2\"\n};\n\nclass D {\n  d1 = \"d1\"\n  'd-2' = \"d2\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  a = \"a\";\n}\n\nclass B {\n  b = \"b\";\n}\n\nclass C {\n  c1 = \"c1\";\n  c2 = \"c2\";\n}\n\nclass D {\n  \"d1\" = \"d1\";\n  \"d-2\" = \"d2\";\n}");
}
#[test]
fn test_classes_js_quote_propspreserve_format_1_3ff08b74() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  a = \"a\"\n};\n\nclass B {\n  'b' = \"b\"\n};\n\nclass C {\n  c1 = \"c1\"\n  'c2' = \"c2\"\n};\n\nclass D {\n  d1 = \"d1\"\n  'd-2' = \"d2\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  a = \"a\";\n}\n\nclass B {\n  \"b\" = \"b\";\n}\n\nclass C {\n  c1 = \"c1\";\n  \"c2\" = \"c2\";\n}\n\nclass D {\n  d1 = \"d1\";\n  \"d-2\" = \"d2\";\n}");
}
#[test]
fn test_objects_js_quote_propsas_needed_format_1_b0145bb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("as-needed")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = {\n  a: \"a\"\n};\n\nconst b = {\n  'b': \"b\"\n};\n\nconst b2 = {\n  // Escapes should stay as escapes and not be unquoted.\n  '\\\\u0062': \"b\",\n  '\\\\u0031': \"1\"\n};\n\nconst c = {\n  c1: \"c1\",\n  'c2': \"c2\"\n};\n\nconst d = {\n  d1: \"d1\",\n  'd-2': \"d2\"\n};\n\n// None of these should become quoted, regardless of the quoteProps value.\nconst e = {\n  NaN: null,\n  1: null,\n  1.5: null,\n  .1: null,\n  1.: null,\n  1.0: null,\n  999999999999999999999: null,\n  0.99999999999999999: null,\n  1E2: null,\n  1e+3: null,\n  1e+100: null,\n  0b10: null,\n  0o10: null,\n  0xf: null,\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: null,\n}\n\nconst f = {\n  // This should be unquoted for quoteProps=as-needed.\n  \"NaN\": null,\n  // Flow does parses number keys, but errors on them during type checking so\n  // don’t unquote them:\n  \"1\": null,\n  \"1.5\": null,\n  // These should never be unquoted. \\`1e+100\\` technically could (it’s the only\n  // one where \\`String(Number(key)) === key\\`), but we came to the conclusion\n  // that it is unexpected.\n  \".1\": null,\n  \"1.\": null,\n  \"1.0\": null,\n  \"999999999999999999999\": null,\n  \"0.99999999999999999\": null,\n  \"1E2\": null,\n  \"1e+3\": null,\n  \"1e+100\": null,\n  \"0b10\": null,\n  \"0o10\": null,\n  \"0xf\": null,\n  \"2n\": null,\n}\n\nObject.entries({\n  // To force quotes for quoteProps=consistent.\n  'a-': 'a-',\n  // These can be quoted:\n  NaN: 'NaN',\n  1: '1',\n  1.5: '1.5',\n  // Prettier will normalize these to \\`0.1\\` and \\`1\\` – then they can be quoted.\n  .1: '.1',\n  1.: '1.',\n  // These should never be quoted. The _actual_ keys are shown as comments.\n  // Copy-paste this into the console to verify. If we were to convert these\n  // numbers into decimal (which completely valid), “information/intent” is\n  // lost. Either way, writing code like this is super confusing.\n  1.0: '1.0', // 1\n  999999999999999999999: '999999999999999999999', // 1e+21\n  0.99999999999999999: '0.99999999999999999', // 1\n  1E2: '1E2', // 100\n  1e+3: '1e+3', // 1000\n  1e+100: '1e+100', // 1e+100 – this one is identical, but would be inconsistent to quote.\n  0b10: '0b10', // 2\n  0o10: '0o10', // 8\n  0xf: '0xf', // 15\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: '2n', // 2\n  0xb_b: '0xb_b', // 187\n  // 0xb_b_bn: '0xb_b_bn', // 3003\n  // 0xan: '0xan', // 10\n  // 0b100000000000_000000000000000011n: '0b100000000000_000000000000000011n' // 536870915\n})\n\n// Negative numbers cannot be unquoted.\n!{\n  \"-1\": null,\n  \"-1.5\": null,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const a = {\n  a: \"a\",\n};\n\nconst b = {\n  b: \"b\",\n};\n\nconst b2 = {\n  // Escapes should stay as escapes and not be unquoted.\n  \"\\\\u0062\": \"b\",\n  \"\\\\u0031\": \"1\",\n};\n\nconst c = {\n  c1: \"c1\",\n  c2: \"c2\",\n};\n\nconst d = {\n  d1: \"d1\",\n  \"d-2\": \"d2\",\n};\n\n// None of these should become quoted, regardless of the quoteProps value.\nconst e = {\n  NaN: null,\n  1: null,\n  1.5: null,\n  0.1: null,\n  1: null,\n  1.0: null,\n  999999999999999999999: null,\n  0.99999999999999999: null,\n  1e2: null,\n  1e3: null,\n  1e100: null,\n  0b10: null,\n  0o10: null,\n  0xf: null,\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: null,\n};\n\nconst f = {\n  // This should be unquoted for quoteProps=as-needed.\n  NaN: null,\n  // Flow does parses number keys, but errors on them during type checking so\n  // don’t unquote them:\n  \"1\": null,\n  \"1.5\": null,\n  // These should never be unquoted. \\`1e+100\\` technically could (it’s the only\n  // one where \\`String(Number(key)) === key\\`), but we came to the conclusion\n  // that it is unexpected.\n  \".1\": null,\n  \"1.\": null,\n  \"1.0\": null,\n  \"999999999999999999999\": null,\n  \"0.99999999999999999\": null,\n  \"1E2\": null,\n  \"1e+3\": null,\n  \"1e+100\": null,\n  \"0b10\": null,\n  \"0o10\": null,\n  \"0xf\": null,\n  \"2n\": null,\n};\n\nObject.entries({\n  // To force quotes for quoteProps=consistent.\n  \"a-\": \"a-\",\n  // These can be quoted:\n  NaN: \"NaN\",\n  1: \"1\",\n  1.5: \"1.5\",\n  // Prettier will normalize these to \\`0.1\\` and \\`1\\` – then they can be quoted.\n  0.1: \".1\",\n  1: \"1.\",\n  // These should never be quoted. The _actual_ keys are shown as comments.\n  // Copy-paste this into the console to verify. If we were to convert these\n  // numbers into decimal (which completely valid), “information/intent” is\n  // lost. Either way, writing code like this is super confusing.\n  1.0: \"1.0\", // 1\n  999999999999999999999: \"999999999999999999999\", // 1e+21\n  0.99999999999999999: \"0.99999999999999999\", // 1\n  1e2: \"1E2\", // 100\n  1e3: \"1e+3\", // 1000\n  1e100: \"1e+100\", // 1e+100 – this one is identical, but would be inconsistent to quote.\n  0b10: \"0b10\", // 2\n  0o10: \"0o10\", // 8\n  0xf: \"0xf\", // 15\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: '2n', // 2\n  0xb_b: \"0xb_b\", // 187\n  // 0xb_b_bn: '0xb_b_bn', // 3003\n  // 0xan: '0xan', // 10\n  // 0b100000000000_000000000000000011n: '0b100000000000_000000000000000011n' // 536870915\n});\n\n// Negative numbers cannot be unquoted.\n!{\n  \"-1\": null,\n  \"-1.5\": null,\n};");
}
#[test]
fn test_objects_js_quote_propsconsistent_format_1_b0145bb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("consistent")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = {\n  a: \"a\"\n};\n\nconst b = {\n  'b': \"b\"\n};\n\nconst b2 = {\n  // Escapes should stay as escapes and not be unquoted.\n  '\\\\u0062': \"b\",\n  '\\\\u0031': \"1\"\n};\n\nconst c = {\n  c1: \"c1\",\n  'c2': \"c2\"\n};\n\nconst d = {\n  d1: \"d1\",\n  'd-2': \"d2\"\n};\n\n// None of these should become quoted, regardless of the quoteProps value.\nconst e = {\n  NaN: null,\n  1: null,\n  1.5: null,\n  .1: null,\n  1.: null,\n  1.0: null,\n  999999999999999999999: null,\n  0.99999999999999999: null,\n  1E2: null,\n  1e+3: null,\n  1e+100: null,\n  0b10: null,\n  0o10: null,\n  0xf: null,\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: null,\n}\n\nconst f = {\n  // This should be unquoted for quoteProps=as-needed.\n  \"NaN\": null,\n  // Flow does parses number keys, but errors on them during type checking so\n  // don’t unquote them:\n  \"1\": null,\n  \"1.5\": null,\n  // These should never be unquoted. \\`1e+100\\` technically could (it’s the only\n  // one where \\`String(Number(key)) === key\\`), but we came to the conclusion\n  // that it is unexpected.\n  \".1\": null,\n  \"1.\": null,\n  \"1.0\": null,\n  \"999999999999999999999\": null,\n  \"0.99999999999999999\": null,\n  \"1E2\": null,\n  \"1e+3\": null,\n  \"1e+100\": null,\n  \"0b10\": null,\n  \"0o10\": null,\n  \"0xf\": null,\n  \"2n\": null,\n}\n\nObject.entries({\n  // To force quotes for quoteProps=consistent.\n  'a-': 'a-',\n  // These can be quoted:\n  NaN: 'NaN',\n  1: '1',\n  1.5: '1.5',\n  // Prettier will normalize these to \\`0.1\\` and \\`1\\` – then they can be quoted.\n  .1: '.1',\n  1.: '1.',\n  // These should never be quoted. The _actual_ keys are shown as comments.\n  // Copy-paste this into the console to verify. If we were to convert these\n  // numbers into decimal (which completely valid), “information/intent” is\n  // lost. Either way, writing code like this is super confusing.\n  1.0: '1.0', // 1\n  999999999999999999999: '999999999999999999999', // 1e+21\n  0.99999999999999999: '0.99999999999999999', // 1\n  1E2: '1E2', // 100\n  1e+3: '1e+3', // 1000\n  1e+100: '1e+100', // 1e+100 – this one is identical, but would be inconsistent to quote.\n  0b10: '0b10', // 2\n  0o10: '0o10', // 8\n  0xf: '0xf', // 15\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: '2n', // 2\n  0xb_b: '0xb_b', // 187\n  // 0xb_b_bn: '0xb_b_bn', // 3003\n  // 0xan: '0xan', // 10\n  // 0b100000000000_000000000000000011n: '0b100000000000_000000000000000011n' // 536870915\n})\n\n// Negative numbers cannot be unquoted.\n!{\n  \"-1\": null,\n  \"-1.5\": null,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const a = {\n  a: \"a\",\n};\n\nconst b = {\n  b: \"b\",\n};\n\nconst b2 = {\n  // Escapes should stay as escapes and not be unquoted.\n  \"\\\\u0062\": \"b\",\n  \"\\\\u0031\": \"1\",\n};\n\nconst c = {\n  c1: \"c1\",\n  c2: \"c2\",\n};\n\nconst d = {\n  \"d1\": \"d1\",\n  \"d-2\": \"d2\",\n};\n\n// None of these should become quoted, regardless of the quoteProps value.\nconst e = {\n  NaN: null,\n  1: null,\n  1.5: null,\n  0.1: null,\n  1: null,\n  1.0: null,\n  999999999999999999999: null,\n  0.99999999999999999: null,\n  1e2: null,\n  1e3: null,\n  1e100: null,\n  0b10: null,\n  0o10: null,\n  0xf: null,\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: null,\n};\n\nconst f = {\n  // This should be unquoted for quoteProps=as-needed.\n  \"NaN\": null,\n  // Flow does parses number keys, but errors on them during type checking so\n  // don’t unquote them:\n  \"1\": null,\n  \"1.5\": null,\n  // These should never be unquoted. \\`1e+100\\` technically could (it’s the only\n  // one where \\`String(Number(key)) === key\\`), but we came to the conclusion\n  // that it is unexpected.\n  \".1\": null,\n  \"1.\": null,\n  \"1.0\": null,\n  \"999999999999999999999\": null,\n  \"0.99999999999999999\": null,\n  \"1E2\": null,\n  \"1e+3\": null,\n  \"1e+100\": null,\n  \"0b10\": null,\n  \"0o10\": null,\n  \"0xf\": null,\n  \"2n\": null,\n};\n\nObject.entries({\n  // To force quotes for quoteProps=consistent.\n  \"a-\": \"a-\",\n  // These can be quoted:\n  \"NaN\": \"NaN\",\n  \"1\": \"1\",\n  \"1.5\": \"1.5\",\n  // Prettier will normalize these to \\`0.1\\` and \\`1\\` – then they can be quoted.\n  \"0.1\": \".1\",\n  \"1\": \"1.\",\n  // These should never be quoted. The _actual_ keys are shown as comments.\n  // Copy-paste this into the console to verify. If we were to convert these\n  // numbers into decimal (which completely valid), “information/intent” is\n  // lost. Either way, writing code like this is super confusing.\n  1.0: \"1.0\", // 1\n  999999999999999999999: \"999999999999999999999\", // 1e+21\n  0.99999999999999999: \"0.99999999999999999\", // 1\n  1e2: \"1E2\", // 100\n  1e3: \"1e+3\", // 1000\n  1e100: \"1e+100\", // 1e+100 – this one is identical, but would be inconsistent to quote.\n  0b10: \"0b10\", // 2\n  0o10: \"0o10\", // 8\n  0xf: \"0xf\", // 15\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: '2n', // 2\n  0xb_b: \"0xb_b\", // 187\n  // 0xb_b_bn: '0xb_b_bn', // 3003\n  // 0xan: '0xan', // 10\n  // 0b100000000000_000000000000000011n: '0b100000000000_000000000000000011n' // 536870915\n});\n\n// Negative numbers cannot be unquoted.\n!{\n  \"-1\": null,\n  \"-1.5\": null,\n};");
}
#[test]
fn test_objects_js_quote_propspreserve_format_1_b0145bb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = {\n  a: \"a\"\n};\n\nconst b = {\n  'b': \"b\"\n};\n\nconst b2 = {\n  // Escapes should stay as escapes and not be unquoted.\n  '\\\\u0062': \"b\",\n  '\\\\u0031': \"1\"\n};\n\nconst c = {\n  c1: \"c1\",\n  'c2': \"c2\"\n};\n\nconst d = {\n  d1: \"d1\",\n  'd-2': \"d2\"\n};\n\n// None of these should become quoted, regardless of the quoteProps value.\nconst e = {\n  NaN: null,\n  1: null,\n  1.5: null,\n  .1: null,\n  1.: null,\n  1.0: null,\n  999999999999999999999: null,\n  0.99999999999999999: null,\n  1E2: null,\n  1e+3: null,\n  1e+100: null,\n  0b10: null,\n  0o10: null,\n  0xf: null,\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: null,\n}\n\nconst f = {\n  // This should be unquoted for quoteProps=as-needed.\n  \"NaN\": null,\n  // Flow does parses number keys, but errors on them during type checking so\n  // don’t unquote them:\n  \"1\": null,\n  \"1.5\": null,\n  // These should never be unquoted. \\`1e+100\\` technically could (it’s the only\n  // one where \\`String(Number(key)) === key\\`), but we came to the conclusion\n  // that it is unexpected.\n  \".1\": null,\n  \"1.\": null,\n  \"1.0\": null,\n  \"999999999999999999999\": null,\n  \"0.99999999999999999\": null,\n  \"1E2\": null,\n  \"1e+3\": null,\n  \"1e+100\": null,\n  \"0b10\": null,\n  \"0o10\": null,\n  \"0xf\": null,\n  \"2n\": null,\n}\n\nObject.entries({\n  // To force quotes for quoteProps=consistent.\n  'a-': 'a-',\n  // These can be quoted:\n  NaN: 'NaN',\n  1: '1',\n  1.5: '1.5',\n  // Prettier will normalize these to \\`0.1\\` and \\`1\\` – then they can be quoted.\n  .1: '.1',\n  1.: '1.',\n  // These should never be quoted. The _actual_ keys are shown as comments.\n  // Copy-paste this into the console to verify. If we were to convert these\n  // numbers into decimal (which completely valid), “information/intent” is\n  // lost. Either way, writing code like this is super confusing.\n  1.0: '1.0', // 1\n  999999999999999999999: '999999999999999999999', // 1e+21\n  0.99999999999999999: '0.99999999999999999', // 1\n  1E2: '1E2', // 100\n  1e+3: '1e+3', // 1000\n  1e+100: '1e+100', // 1e+100 – this one is identical, but would be inconsistent to quote.\n  0b10: '0b10', // 2\n  0o10: '0o10', // 8\n  0xf: '0xf', // 15\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: '2n', // 2\n  0xb_b: '0xb_b', // 187\n  // 0xb_b_bn: '0xb_b_bn', // 3003\n  // 0xan: '0xan', // 10\n  // 0b100000000000_000000000000000011n: '0b100000000000_000000000000000011n' // 536870915\n})\n\n// Negative numbers cannot be unquoted.\n!{\n  \"-1\": null,\n  \"-1.5\": null,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const a = {\n  a: \"a\",\n};\n\nconst b = {\n  \"b\": \"b\",\n};\n\nconst b2 = {\n  // Escapes should stay as escapes and not be unquoted.\n  \"\\\\u0062\": \"b\",\n  \"\\\\u0031\": \"1\",\n};\n\nconst c = {\n  c1: \"c1\",\n  \"c2\": \"c2\",\n};\n\nconst d = {\n  d1: \"d1\",\n  \"d-2\": \"d2\",\n};\n\n// None of these should become quoted, regardless of the quoteProps value.\nconst e = {\n  NaN: null,\n  1: null,\n  1.5: null,\n  0.1: null,\n  1: null,\n  1.0: null,\n  999999999999999999999: null,\n  0.99999999999999999: null,\n  1e2: null,\n  1e3: null,\n  1e100: null,\n  0b10: null,\n  0o10: null,\n  0xf: null,\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: null,\n};\n\nconst f = {\n  // This should be unquoted for quoteProps=as-needed.\n  \"NaN\": null,\n  // Flow does parses number keys, but errors on them during type checking so\n  // don’t unquote them:\n  \"1\": null,\n  \"1.5\": null,\n  // These should never be unquoted. \\`1e+100\\` technically could (it’s the only\n  // one where \\`String(Number(key)) === key\\`), but we came to the conclusion\n  // that it is unexpected.\n  \".1\": null,\n  \"1.\": null,\n  \"1.0\": null,\n  \"999999999999999999999\": null,\n  \"0.99999999999999999\": null,\n  \"1E2\": null,\n  \"1e+3\": null,\n  \"1e+100\": null,\n  \"0b10\": null,\n  \"0o10\": null,\n  \"0xf\": null,\n  \"2n\": null,\n};\n\nObject.entries({\n  // To force quotes for quoteProps=consistent.\n  \"a-\": \"a-\",\n  // These can be quoted:\n  NaN: \"NaN\",\n  1: \"1\",\n  1.5: \"1.5\",\n  // Prettier will normalize these to \\`0.1\\` and \\`1\\` – then they can be quoted.\n  0.1: \".1\",\n  1: \"1.\",\n  // These should never be quoted. The _actual_ keys are shown as comments.\n  // Copy-paste this into the console to verify. If we were to convert these\n  // numbers into decimal (which completely valid), “information/intent” is\n  // lost. Either way, writing code like this is super confusing.\n  1.0: \"1.0\", // 1\n  999999999999999999999: \"999999999999999999999\", // 1e+21\n  0.99999999999999999: \"0.99999999999999999\", // 1\n  1e2: \"1E2\", // 100\n  1e3: \"1e+3\", // 1000\n  1e100: \"1e+100\", // 1e+100 – this one is identical, but would be inconsistent to quote.\n  0b10: \"0b10\", // 2\n  0o10: \"0o10\", // 8\n  0xf: \"0xf\", // 15\n  // Commented out because Flow does not parse BigInt as object keys.\n  // 2n: '2n', // 2\n  0xb_b: \"0xb_b\", // 187\n  // 0xb_b_bn: '0xb_b_bn', // 3003\n  // 0xan: '0xan', // 10\n  // 0b100000000000_000000000000000011n: '0b100000000000_000000000000000011n' // 536870915\n});\n\n// Negative numbers cannot be unquoted.\n!{\n  \"-1\": null,\n  \"-1.5\": null,\n};");
}
#[test]
fn test_with_member_expressions_js_quote_propsas_needed_format_1_ae25f94c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("as-needed")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const obj = {\n  foo: \"\",\n  [foo.bar]: \"\"\n};\n\nclass Foo {\n  foo() {}\n  [foo.bar]() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const obj = {\n  foo: \"\",\n  [foo.bar]: \"\",\n};\n\nclass Foo {\n  foo() {}\n  [foo.bar]() {}\n}");
}
#[test]
fn test_with_member_expressions_js_quote_propsconsistent_format_1_ae25f94c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("consistent")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const obj = {\n  foo: \"\",\n  [foo.bar]: \"\"\n};\n\nclass Foo {\n  foo() {}\n  [foo.bar]() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const obj = {\n  foo: \"\",\n  [foo.bar]: \"\",\n};\n\nclass Foo {\n  foo() {}\n  [foo.bar]() {}\n}");
}
#[test]
fn test_with_member_expressions_js_quote_propspreserve_format_1_ae25f94c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const obj = {\n  foo: \"\",\n  [foo.bar]: \"\"\n};\n\nclass Foo {\n  foo() {}\n  [foo.bar]() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const obj = {\n  foo: \"\",\n  [foo.bar]: \"\",\n};\n\nclass Foo {\n  foo() {}\n  [foo.bar]() {}\n}");
}
#[test]
fn test_with_numbers_js_quote_propsas_needed_format_1_e2fe5220() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("as-needed")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("obj = {\n  foo: \"\",\n  1: \"\"\n};\n\nobj = {\n  \"bar\": \"\",\n  1: \"\"\n};\n\nobj = {\n  \"foo-bar\": \"\",\n  1: \"\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "obj = {\n  foo: \"\",\n  1: \"\",\n};\n\nobj = {\n  bar: \"\",\n  1: \"\",\n};\n\nobj = {\n  \"foo-bar\": \"\",\n  1: \"\",\n};");
}
#[test]
fn test_with_numbers_js_quote_propsconsistent_format_1_e2fe5220() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("consistent")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("obj = {\n  foo: \"\",\n  1: \"\"\n};\n\nobj = {\n  \"bar\": \"\",\n  1: \"\"\n};\n\nobj = {\n  \"foo-bar\": \"\",\n  1: \"\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "obj = {\n  foo: \"\",\n  1: \"\",\n};\n\nobj = {\n  bar: \"\",\n  1: \"\",\n};\n\nobj = {\n  \"foo-bar\": \"\",\n  \"1\": \"\",\n};");
}
#[test]
fn test_with_numbers_js_quote_propspreserve_format_1_e2fe5220() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .quote_props("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("obj = {\n  foo: \"\",\n  1: \"\"\n};\n\nobj = {\n  \"bar\": \"\",\n  1: \"\"\n};\n\nobj = {\n  \"foo-bar\": \"\",\n  1: \"\"\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "obj = {\n  foo: \"\",\n  1: \"\",\n};\n\nobj = {\n  \"bar\": \"\",\n  1: \"\",\n};\n\nobj = {\n  \"foo-bar\": \"\",\n  1: \"\",\n};");
}
