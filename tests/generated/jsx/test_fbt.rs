#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_0193a4c7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("x =\n  <fbt>\n    <fbt:param>\n      First\n    </fbt:param>,\n    <fbt:param>\n      Second\n    </fbt:param>\n  </fbt>\n\nx =\n  <fbt>\n    <fbt:param>\n      First\n    </fbt:param>\n    ,\n    <fbt:param>\n      Second\n    </fbt:param>\n  </fbt>\n\nx =\n  <fbt>\n    <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>\n  </fbt>\n\nx =\n  <fbt>\n    <fbt:param>\n      First\n    </fbt:param>,<fbt:param>\n      Second\n    </fbt:param>\n  </fbt>\n\nx =\n  <fbt desc=\"example 1\">\n    Prefix comes before\n    <fbt:param>\n      <b>\n        suffix\n      </b>\n    </fbt:param>\n  </fbt>\n\nx =\n  <fbt desc=\"example 2\">\n    Prefix comes before\n    <fbt:param name=\"bold stuff\">\n      <b>\n      suffix\n      </b>\n    </fbt:param>\n    <fbt:param name=\"a link\">\n      <link href=\"#\">\n        suffix\n      </link>\n    </fbt:param>\n  </fbt>\n\nx =\n  <fbt desc=\"example 3\">\n    Count Chocula knows the the number\n    <fbt:enum enum-range={['one', 'two', 'three']} value={getValue()} />\n    is awesome\n  </fbt>\n\nx = (\n  <fbt>\n    {hour}:{minute}:{second}\n  </fbt>\n);\n\nx = (\n  <fbt>\n    {hour}\n    :\n    {minute}\n    :\n    {second}\n  </fbt>\n);\n\nx = (\n  <fbt>\n    {hour}:\n    {minute}:\n    {second}\n  </fbt>\n);\n\nfirst = (\n  <fbt>\n    Text<br />\n    More text<br />\n    And more<br />\n  </fbt>\n);\n\nsecond = (\n  <fbt>\n    Text<br />More text<br />And more<br />\n  </fbt>\n);\n\nthird = (\n  <fbt>\n    Text\n    <br />\n    More text\n    <br />\n    And more\n    <br />\n  </fbt>\n);") ? ;
    assert_eq ! (formatted , "x = (\n  <fbt>\n    <fbt:param>First</fbt:param>,\n    <fbt:param>Second</fbt:param>\n  </fbt>\n);\n\nx = (\n  <fbt>\n    <fbt:param>First</fbt:param>\n    ,\n    <fbt:param>Second</fbt:param>\n  </fbt>\n);\n\nx = (\n  <fbt>\n    <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>\n  </fbt>\n);\n\nx = (\n  <fbt>\n    <fbt:param>First</fbt:param>,<fbt:param>Second</fbt:param>\n  </fbt>\n);\n\nx = (\n  <fbt desc=\"example 1\">\n    Prefix comes before\n    <fbt:param>\n      <b>suffix</b>\n    </fbt:param>\n  </fbt>\n);\n\nx = (\n  <fbt desc=\"example 2\">\n    Prefix comes before\n    <fbt:param name=\"bold stuff\">\n      <b>suffix</b>\n    </fbt:param>\n    <fbt:param name=\"a link\">\n      <link href=\"#\">suffix</link>\n    </fbt:param>\n  </fbt>\n);\n\nx = (\n  <fbt desc=\"example 3\">\n    Count Chocula knows the the number\n    <fbt:enum enum-range={[\"one\", \"two\", \"three\"]} value={getValue()} />\n    is awesome\n  </fbt>\n);\n\nx = (\n  <fbt>\n    {hour}:{minute}:{second}\n  </fbt>\n);\n\nx = (\n  <fbt>\n    {hour}\n    :\n    {minute}\n    :\n    {second}\n  </fbt>\n);\n\nx = (\n  <fbt>\n    {hour}:\n    {minute}:\n    {second}\n  </fbt>\n);\n\nfirst = (\n  <fbt>\n    Text<br />\n    More text<br />\n    And more<br />\n  </fbt>\n);\n\nsecond = (\n  <fbt>\n    Text<br />More text<br />And more<br />\n  </fbt>\n);\n\nthird = (\n  <fbt>\n    Text\n    <br />\n    More text\n    <br />\n    And more\n    <br />\n  </fbt>\n);");
    Ok(())
}
