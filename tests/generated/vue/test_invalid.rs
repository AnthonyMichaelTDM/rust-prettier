#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_v_for_vue_format_1_0a10f628() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>\n    <div\n      v-for=\"    _       in          \"\n      v-for=\"            in _        \"\n      v-for=\"            in          \"\n      v-for=\"    _,      in a        \"\n      v-for=\"    ,_      in a        \"\n\n      v-for=\"    a, b,      in a        \"\n      v-for=\"    a,  , c     in a        \"\n\n      v-for=\"     , b, c     in a        \"\n      v-for=\"    a, b,       in a        \"\n\n      v-for=\"     , b, c     in a        \"\n      v-for=\"    a,  , c     in a        \"\n      v-for=\"    (,a,b) of 'abcd'        \"\n    ></div>\n  </div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div>\n    <div\n      v-for=\"    _       in          \"\n      v-for=\"            in _        \"\n      v-for=\"            in          \"\n      v-for=\"_ in a\"\n      v-for=\"    ,_      in a        \"\n      v-for=\"(a, b) in a\"\n      v-for=\"    a,  , c     in a        \"\n      v-for=\"     , b, c     in a        \"\n      v-for=\"(a, b) in a\"\n      v-for=\"     , b, c     in a        \"\n      v-for=\"    a,  , c     in a        \"\n      v-for=\"    (,a,b) of 'abcd'        \"\n    ></div>\n  </div>\n</template>");
    Ok(())
}
