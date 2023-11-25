#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_break_js_format_1_b2105a4c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export default class AddAssetHtmlPlugin {\n  apply(compiler: WebpackCompilerType) {\n    compiler.plugin('compilation', (compilation: WebpackCompilationType) => {\n      compilation.plugin('html-webpack-plugin-before-html', (callback: Callback<any>) => {\n        addAllAssetsToCompilation(this.assets, compilation, htmlPluginData, callback);\n      });\n    });\n  }\n}") ? ;
    assert_eq ! (formatted , "export default class AddAssetHtmlPlugin {\n  apply(compiler: WebpackCompilerType) {\n    compiler.plugin(\"compilation\", (compilation: WebpackCompilationType) => {\n      compilation.plugin(\n        \"html-webpack-plugin-before-html\",\n        (callback: Callback<any>) => {\n          addAllAssetsToCompilation(\n            this.assets,\n            compilation,\n            htmlPluginData,\n            callback,\n          );\n        },\n      );\n    });\n  }\n}");
    Ok(())
}
#[test]
fn test_decorated_function_js_format_1_fa53c3db() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const Counter = decorator(\"my-counter\")(\n  (props: { initialCount?: number; label?: string }) => {\n    const p = useDefault(props, {\n      initialCount: 0,\n      label: \"Counter\",\n    });\n\n    const [s, set] = useState({ count: p.initialCount });\n    const onClick = () => set(\"count\", (it) => it + 1);\n\n    return () => (\n      <button onclick={onClick}>\n        {p.label}: {s.count}\n      </button>\n    );\n  }\n);\n\nconst Counter2 = decorators.decorator(\"my-counter\")(\n  (props: { initialCount?: number; label?: string }) => {\n    return () => (\n      <button onclick={onClick}>\n        {p.label}: {s.count}\n      </button>\n    );\n  }\n);\n\nexport default decorators.decorator(\"my-counter\")(\n  (props: { initialCount?: number; label?: string }) => {\n    return foo;\n  }\n);") ? ;
    assert_eq ! (formatted , "const Counter = decorator(\"my-counter\")((props: {\n  initialCount?: number,\n  label?: string,\n}) => {\n  const p = useDefault(props, {\n    initialCount: 0,\n    label: \"Counter\",\n  });\n\n  const [s, set] = useState({ count: p.initialCount });\n  const onClick = () => set(\"count\", (it) => it + 1);\n\n  return () => (\n    <button onclick={onClick}>\n      {p.label}: {s.count}\n    </button>\n  );\n});\n\nconst Counter2 = decorators.decorator(\"my-counter\")((props: {\n  initialCount?: number,\n  label?: string,\n}) => {\n  return () => (\n    <button onclick={onClick}>\n      {p.label}: {s.count}\n    </button>\n  );\n});\n\nexport default decorators.decorator(\"my-counter\")((props: {\n  initialCount?: number,\n  label?: string,\n}) => {\n  return foo;\n});");
    Ok(())
}
#[test]
fn test_edge_case_js_format_1_438e2ab7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var listener = DOM.listen(\n  introCard,\n  'click',\n  sigil,\n  (event: JavelinEvent): void =>\n    BanzaiLogger.log(\n      config,\n      {...logData, ...DataStore.get(event.getNode(sigil))},\n    ),\n);") ? ;
    assert_eq ! (formatted , "var listener = DOM.listen(\n  introCard,\n  \"click\",\n  sigil,\n  (event: JavelinEvent): void =>\n    BanzaiLogger.log(config, {\n      ...logData,\n      ...DataStore.get(event.getNode(sigil)),\n    }),\n);");
    Ok(())
}
