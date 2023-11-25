use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_enumerror_js_format_1_b4f469ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/** @flow */\n\nfunction isActive(ad: {state: $Keys<{\n    PAUSED: string;\n    ACTIVE: string;\n    DELETED: string;\n}>}): boolean {\n    return ad.state === 'ACTIVE';\n};\nisActive({state: 'PAUSE'});\n\nvar MyStates = {\n    PAUSED: 'PAUSED',\n    ACTIVE: 'ACTIVE',\n    DELETED: 'DELETED',\n};\nfunction isActive2(ad: {state: $Keys<typeof MyStates>}): boolean {\n    return ad.state === MyStates.ACTIVE;\n};\nisActive2({state: 'PAUSE'});\n\ntype Keys = $Keys<{ x: any, y: any }>;\ntype Union = \"x\" | \"y\"\n\nfunction keys2union(s: Keys): Union { return s; } // ok\nfunction union2keys(s: Union): Keys { return s; } // ok") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/** @flow */\n\nfunction isActive(ad: {\n  state: $Keys<{\n    PAUSED: string,\n    ACTIVE: string,\n    DELETED: string,\n  }>,\n}): boolean {\n  return ad.state === \"ACTIVE\";\n}\nisActive({ state: \"PAUSE\" });\n\nvar MyStates = {\n  PAUSED: \"PAUSED\",\n  ACTIVE: \"ACTIVE\",\n  DELETED: \"DELETED\",\n};\nfunction isActive2(ad: { state: $Keys<typeof MyStates> }): boolean {\n  return ad.state === MyStates.ACTIVE;\n}\nisActive2({ state: \"PAUSE\" });\n\ntype Keys = $Keys<{ x: any, y: any }>;\ntype Union = \"x\" | \"y\";\n\nfunction keys2union(s: Keys): Union {\n  return s;\n} // ok\nfunction union2keys(s: Union): Keys {\n  return s;\n} // ok");
}
