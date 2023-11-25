#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_jsx_ignore_js_format_1_8602d221() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// this should remain as-is\n<div>\n {/* prettier-ignore */}\n <style jsx global>{ComponentStyles}</style>\n</div>;\n\n// this should remain as-is\n<div>\n {/* prettier-ignore */}\n <span     ugly  format=''   />\n</div>;\n\n// this should remain as-is\nf(\n  <Component>\n    {/*prettier-ignore*/}\n    <span     ugly  format=''   />\n  </Component>\n);\n\n// this be formatted\n<div>\n  {/* prettier-ignore */} foo\n  <Bar   excessive    spaces    />\n</div>;\n\n// this should remain as-is\n<div>\n{\n  /* prettier-ignore */\n  foo ( )\n}\n</div>;\n\n// this should remain as-is\n<div>\n{\n  /* prettier-ignore */\n  x     ?   <Y/> : <Z/>\n}\n</div>;\n\npush(\n  // prettier-ignore\n  <td> :)\n  </td>,\n);\n\nfunction f() {\n  return (\n    // prettier-ignore\n    /* $FlowFixMe(>=0.53.0) */\n    <JSX />\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// this should remain as-is\n<div>\n  {/* prettier-ignore */}\n  <style jsx global>{ComponentStyles}</style>\n</div>;\n\n// this should remain as-is\n<div>\n  {/* prettier-ignore */}\n  <span     ugly  format=''   />\n</div>;\n\n// this should remain as-is\nf(\n  <Component>\n    {/*prettier-ignore*/}\n    <span     ugly  format=''   />\n  </Component>,\n);\n\n// this be formatted\n<div>\n  {/* prettier-ignore */} foo\n  <Bar excessive spaces />\n</div>;\n\n// this should remain as-is\n<div>\n  {\n    /* prettier-ignore */\n    foo ( )\n  }\n</div>;\n\n// this should remain as-is\n<div>\n  {\n    /* prettier-ignore */\n    x     ?   <Y/> : <Z/>\n  }\n</div>;\n\npush(\n  // prettier-ignore\n  <td> :)\n  </td>,\n);\n\nfunction f() {\n  return (\n    // prettier-ignore\n    /* $FlowFixMe(>=0.53.0) */\n    <JSX />\n  );\n}");
}
