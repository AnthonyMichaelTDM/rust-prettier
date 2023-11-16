#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_break_ts_format_1_b2105a4c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export default class AddAssetHtmlPlugin {\n  apply(compiler: WebpackCompilerType) {\n    compiler.plugin('compilation', (compilation: WebpackCompilationType) => {\n      compilation.plugin('html-webpack-plugin-before-html', (callback: Callback<any>) => {\n        addAllAssetsToCompilation(this.assets, compilation, htmlPluginData, callback);\n      });\n    });\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export default class AddAssetHtmlPlugin {\n  apply(compiler: WebpackCompilerType) {\n    compiler.plugin(\"compilation\", (compilation: WebpackCompilationType) => {\n      compilation.plugin(\n        \"html-webpack-plugin-before-html\",\n        (callback: Callback<any>) => {\n          addAllAssetsToCompilation(\n            this.assets,\n            compilation,\n            htmlPluginData,\n            callback,\n          );\n        },\n      );\n    });\n  }\n}");
}
#[test]
fn test_decorated_function_tsx_format_1_fa933373() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const Counter = decorator(\"my-counter\")(\n  (props: { initialCount?: number; label?: string }) => {\n    const p = useDefault(props, {\n      initialCount: 0,\n      label: \"Counter\",\n    });\n\n    const [s, set] = useState({ count: p.initialCount });\n    const onClick = () => set(\"count\", (it) => it + 1);\n\n    return () => (\n      <button onclick={onClick}>\n        {p.label}: {s.count}\n      </button>\n    );\n  }\n);\n\nconst Counter2 = decorators.decorator(\"my-counter\")(\n  (props: { initialCount?: number; label?: string }) => {\n    return () => (\n      <button onclick={onClick}>\n        {p.label}: {s.count}\n      </button>\n    );\n  }\n);\n\nexport default decorators.decorator(\"my-counter\")(\n  (props: { initialCount?: number; label?: string }) => {\n    return foo;\n  }\n);\n\nexport = decorators.decorator(\"my-counter\")(\n  (props: { initialCount?: number; label?: string }) => {\n    return foo;\n  }\n);\n\nmodule.exports = decorators.decorator(\"my-counter\")(\n  (props: { initialCount?: number; label?: string }) => {\n    return foo;\n  }\n);\n\nconst Counter =\n  decorator(\"foo\")(\n  decorator(\"bar\")(\n  (props: {\n    loremFoo1: Array<Promise<any>>,\n    ipsumBarr2: Promise<number>,\n  }) => {\n    return <div/>;\n  }));") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Counter = decorator(\"my-counter\")((props: {\n  initialCount?: number;\n  label?: string;\n}) => {\n  const p = useDefault(props, {\n    initialCount: 0,\n    label: \"Counter\",\n  });\n\n  const [s, set] = useState({ count: p.initialCount });\n  const onClick = () => set(\"count\", (it) => it + 1);\n\n  return () => (\n    <button onclick={onClick}>\n      {p.label}: {s.count}\n    </button>\n  );\n});\n\nconst Counter2 = decorators.decorator(\"my-counter\")((props: {\n  initialCount?: number;\n  label?: string;\n}) => {\n  return () => (\n    <button onclick={onClick}>\n      {p.label}: {s.count}\n    </button>\n  );\n});\n\nexport default decorators.decorator(\"my-counter\")((props: {\n  initialCount?: number;\n  label?: string;\n}) => {\n  return foo;\n});\n\nexport = decorators.decorator(\"my-counter\")((props: {\n  initialCount?: number;\n  label?: string;\n}) => {\n  return foo;\n});\n\nmodule.exports = decorators.decorator(\"my-counter\")((props: {\n  initialCount?: number;\n  label?: string;\n}) => {\n  return foo;\n});\n\nconst Counter = decorator(\"foo\")(\n  decorator(\"bar\")(\n    (props: {\n      loremFoo1: Array<Promise<any>>;\n      ipsumBarr2: Promise<number>;\n    }) => {\n      return <div />;\n    },\n  ),\n);");
}
#[test]
fn test_edge_case_ts_format_1_438e2ab7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var listener = DOM.listen(\n  introCard,\n  'click',\n  sigil,\n  (event: JavelinEvent): void =>\n    BanzaiLogger.log(\n      config,\n      {...logData, ...DataStore.get(event.getNode(sigil))},\n    ),\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var listener = DOM.listen(\n  introCard,\n  \"click\",\n  sigil,\n  (event: JavelinEvent): void =>\n    BanzaiLogger.log(config, {\n      ...logData,\n      ...DataStore.get(event.getNode(sigil)),\n    }),\n);");
}
#[test]
fn test_forward_ref_tsx_format_1_5a445a7f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export const Link = forwardRef<HTMLAnchorElement, LinkProps>(\n  function Link(props, ref) {\n    return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n  }\n);\n\nexport const LinkWithLongName = forwardRef<HTMLAnchorElement, LinkProps>(\n  function Link(props, ref) {\n    return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n  }\n);\n\nexport const Arrow = forwardRef<HTMLAnchorElement, LinkProps>((props, ref) => {\n  return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n});\n\nexport const ArrowWithLongName = forwardRef<HTMLAnchorElement, LinkProps>(\n  (props, ref) => {\n    return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n  }\n);\n\nconst Link = React.forwardRef<HTMLAnchorElement, LinkProps>(\n  function Link(props, ref) {\n    return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n  },\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const Link = forwardRef<HTMLAnchorElement, LinkProps>(\n  function Link(props, ref) {\n    return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n  },\n);\n\nexport const LinkWithLongName = forwardRef<HTMLAnchorElement, LinkProps>(\n  function Link(props, ref) {\n    return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n  },\n);\n\nexport const Arrow = forwardRef<HTMLAnchorElement, LinkProps>((props, ref) => {\n  return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n});\n\nexport const ArrowWithLongName = forwardRef<HTMLAnchorElement, LinkProps>(\n  (props, ref) => {\n    return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n  },\n);\n\nconst Link = React.forwardRef<HTMLAnchorElement, LinkProps>(\n  function Link(props, ref) {\n    return <ThemeUILink ref={ref} variant=\"default\" {...props} />;\n  },\n);");
}
