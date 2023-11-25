#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_attributes_vue_semifalse_format_1_b687bddd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<div\n  v-for=\"({ longLongProp, longLongProp }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp=42, longLongProp='Hello, World!' }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp }) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp=42, anotherLongLongProp, yetAnotherLongLongProp='Hello, World!'], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, [longLongProp, longLongProp, [longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, [longLongProp, longLongProp='Hello, Prettier!', [longLongProp, longLongProp, anotherLongLongProp=[longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([{ longLongProp, longLongProp }, { longLongProp, longLongProp }, [{ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, longLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({firstValue, secondValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue={ longLongProp, longLongProp }, secondValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue: {firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue, seventhValue}, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp={ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, yetAnotherLongLongProp }, sixthValue, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"  item  in  items \"\n  v-for=\"  item  of  items \"\n  v-for=\"(    item    , index)    in    items\"\n  v-for=\"value    in     object\"\n  v-for=\"(value,    key)    in    object\"\n  v-for=\"(value,    key)    of    object\"\n  v-for=\"(value  ,   key,   index)    in   object\"\n  v-for=\"   n    in   evenNumbers\"\n  v-for=\" n in even   ( numbers) \"\n  v-for=\" n    in    10\"\n  v-for=\" { a }    in    [0].map(()=>({a:1}))   \"\n  v-for=\" ({ a }, [c  ])    in    [0].map(()=>1)   \"\n  v-for=\" n in items.map(x => { return x }) \"\n  @click=\"  /* hello */   \"\n  @click=\"   /* 1 */ $emit( /* 2 */ 'click' /* 3 */ ) /* 4 */ ; /* 5 */   \"\n  @click=\"   $emit(   'click'   )   \"\n  @click=\"   $emit(   'click'   )  ;\"\n  @click=\"   $emit(   'click'   )  ;if(something){for(let i=j;i<100;i++){}}else{}\"\n  slot-scope=\"     foo\"\n  slot-scope=\"     {row   }\"\n  slot-scope=\"{destructuring:{   a:{b}}}\"\n  #default=\"     foo\"\n  #default=\"     {row   }\"\n  #default=\"{destructuring:{   a:{b}}}\"\n  v-slot=\"     foo\"\n  v-slot=\"     {row   }\"\n  v-slot=\"{destructuring:{   a:{b}}}\"\n  v-slot:name=\"     foo\"\n  v-slot:name=\"     {row   }\"\n  v-slot:name=\"{destructuring:{   a:{b}}}\"\n  :class=\"{ longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: true }\"\n  :class=\"(() => { return 'hello' })()\"\n  :key=\"index /* hello */ \"\n  :key=\"index // hello \"\n  @click=\"() => {console.log(test)}\"\n  @click=\"\n    () => {\n      console.log(test);\n    }\n  \"\n  @click=\"doSomething()\"\n  @click=\"doSomething;\"\n  @click=\"a.b;\"\n  @click=\"a[1];\"\n  @click=\"a['b'];\"\n  @click=\"a[null];\"\n  #default=\"{foo:{bar:{baz}}}\"\n></div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div\n    v-for=\"(\n      { longLongProp, longLongProp }, index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      { longLongProp = 42, longLongProp = 'Hello, World!' }, index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"{\n      longLongProp,\n      longLongProp,\n    } of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      {\n        longLongProp,\n        longLongProp,\n        anotherLongLongProp,\n        yetAnotherLongLongProp,\n      },\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"{\n      longLongProp,\n      longLongProp,\n      anotherLongLongProp,\n      yetAnotherLongLongProp,\n    } of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp = 42,\n        anotherLongLongProp,\n        yetAnotherLongLongProp = 'Hello, World!',\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp,\n        [\n          longLongProp,\n          longLongProp,\n          [\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp,\n          ],\n          yetAnotherLongLongProp,\n        ],\n        yetAnotherLongLongProp,\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp,\n        [\n          longLongProp,\n          longLongProp = 'Hello, Prettier!',\n          [\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp = [\n              longLongProp,\n              longLongProp,\n              anotherLongLongProp,\n              yetAnotherLongLongProp,\n            ],\n            yetAnotherLongLongProp,\n          ],\n          yetAnotherLongLongProp,\n        ],\n        yetAnotherLongLongProp,\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        { longLongProp, longLongProp },\n        { longLongProp, longLongProp },\n        [\n          {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp,\n          },\n          longLongProp,\n        ],\n        yetAnotherLongLongProp,\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp,\n        },\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue = { longLongProp, longLongProp }, secondValue },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue, secondValue, thirdValue, fourthValue }, objectKey, index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue, secondValue, thirdValue, fourthValue }, objectKey, index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp,\n        },\n        sixthValue,\n        seventhValue,\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp,\n        },\n        sixthValue: {\n          firstValue,\n          secondValue,\n          thirdValue,\n          fourthValue,\n          fifthValue: {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp,\n          },\n          sixthValue,\n          seventhValue,\n        },\n        seventhValue,\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp = {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp,\n          },\n          yetAnotherLongLongProp,\n        },\n        sixthValue,\n        seventhValue,\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"item in items\"\n    v-for=\"item of items\"\n    v-for=\"(item, index) in items\"\n    v-for=\"value in object\"\n    v-for=\"(value, key) in object\"\n    v-for=\"(value, key) of object\"\n    v-for=\"(value, key, index) in object\"\n    v-for=\"n in evenNumbers\"\n    v-for=\"n in even(numbers)\"\n    v-for=\"n in 10\"\n    v-for=\"{ a } in [0].map(() => ({ a: 1 }))\"\n    v-for=\"({ a }, [c]) in [0].map(() => 1)\"\n    v-for=\"n in items.map((x) => {\n      return x\n    })\"\n    @click=\"/* hello */\"\n    @click=\"/* 1 */ $emit(/* 2 */ 'click' /* 3 */) /* 4 */ /* 5 */\"\n    @click=\"$emit('click')\"\n    @click=\"$emit('click')\"\n    @click=\"\n      $emit('click')\n      if (something) {\n        for (let i = j; i < 100; i++) {}\n      } else {\n      }\n    \"\n    slot-scope=\"foo\"\n    slot-scope=\"{ row }\"\n    slot-scope=\"{\n      destructuring: {\n        a: { b },\n      },\n    }\"\n    #default=\"foo\"\n    #default=\"{ row }\"\n    #default=\"{\n      destructuring: {\n        a: { b },\n      },\n    }\"\n    v-slot=\"foo\"\n    v-slot=\"{ row }\"\n    v-slot=\"{\n      destructuring: {\n        a: { b },\n      },\n    }\"\n    v-slot:name=\"foo\"\n    v-slot:name=\"{ row }\"\n    v-slot:name=\"{\n      destructuring: {\n        a: { b },\n      },\n    }\"\n    :class=\"{\n      longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: true,\n    }\"\n    :class=\"\n      (() => {\n        return 'hello'\n      })()\n    \"\n    :key=\"index /* hello */\"\n    :key=\"\n      index // hello\n    \"\n    @click=\"\n      () => {\n        console.log(test)\n      }\n    \"\n    @click=\"\n      () => {\n        console.log(test)\n      }\n    \"\n    @click=\"doSomething()\"\n    @click=\"doSomething;\"\n    @click=\"a.b;\"\n    @click=\"a[1];\"\n    @click=\"a['b'];\"\n    @click=\"a[null]\"\n    #default=\"{\n      foo: {\n        bar: { baz },\n      },\n    }\"\n  ></div>\n</template>");
    Ok(())
}
#[test]
fn test_attributes_vue_trailing_commaes_5_format_1_b687bddd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<div\n  v-for=\"({ longLongProp, longLongProp }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp=42, longLongProp='Hello, World!' }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp }) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp=42, anotherLongLongProp, yetAnotherLongLongProp='Hello, World!'], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, [longLongProp, longLongProp, [longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, [longLongProp, longLongProp='Hello, Prettier!', [longLongProp, longLongProp, anotherLongLongProp=[longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([{ longLongProp, longLongProp }, { longLongProp, longLongProp }, [{ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, longLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({firstValue, secondValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue={ longLongProp, longLongProp }, secondValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue: {firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue, seventhValue}, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp={ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, yetAnotherLongLongProp }, sixthValue, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"  item  in  items \"\n  v-for=\"  item  of  items \"\n  v-for=\"(    item    , index)    in    items\"\n  v-for=\"value    in     object\"\n  v-for=\"(value,    key)    in    object\"\n  v-for=\"(value,    key)    of    object\"\n  v-for=\"(value  ,   key,   index)    in   object\"\n  v-for=\"   n    in   evenNumbers\"\n  v-for=\" n in even   ( numbers) \"\n  v-for=\" n    in    10\"\n  v-for=\" { a }    in    [0].map(()=>({a:1}))   \"\n  v-for=\" ({ a }, [c  ])    in    [0].map(()=>1)   \"\n  v-for=\" n in items.map(x => { return x }) \"\n  @click=\"  /* hello */   \"\n  @click=\"   /* 1 */ $emit( /* 2 */ 'click' /* 3 */ ) /* 4 */ ; /* 5 */   \"\n  @click=\"   $emit(   'click'   )   \"\n  @click=\"   $emit(   'click'   )  ;\"\n  @click=\"   $emit(   'click'   )  ;if(something){for(let i=j;i<100;i++){}}else{}\"\n  slot-scope=\"     foo\"\n  slot-scope=\"     {row   }\"\n  slot-scope=\"{destructuring:{   a:{b}}}\"\n  #default=\"     foo\"\n  #default=\"     {row   }\"\n  #default=\"{destructuring:{   a:{b}}}\"\n  v-slot=\"     foo\"\n  v-slot=\"     {row   }\"\n  v-slot=\"{destructuring:{   a:{b}}}\"\n  v-slot:name=\"     foo\"\n  v-slot:name=\"     {row   }\"\n  v-slot:name=\"{destructuring:{   a:{b}}}\"\n  :class=\"{ longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: true }\"\n  :class=\"(() => { return 'hello' })()\"\n  :key=\"index /* hello */ \"\n  :key=\"index // hello \"\n  @click=\"() => {console.log(test)}\"\n  @click=\"\n    () => {\n      console.log(test);\n    }\n  \"\n  @click=\"doSomething()\"\n  @click=\"doSomething;\"\n  @click=\"a.b;\"\n  @click=\"a[1];\"\n  @click=\"a['b'];\"\n  @click=\"a[null];\"\n  #default=\"{foo:{bar:{baz}}}\"\n></div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div\n    v-for=\"(\n      { longLongProp, longLongProp }, index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      { longLongProp = 42, longLongProp = 'Hello, World!' }, index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"{\n      longLongProp,\n      longLongProp,\n    } of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      {\n        longLongProp,\n        longLongProp,\n        anotherLongLongProp,\n        yetAnotherLongLongProp,\n      },\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"{\n      longLongProp,\n      longLongProp,\n      anotherLongLongProp,\n      yetAnotherLongLongProp,\n    } of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp = 42,\n        anotherLongLongProp,\n        yetAnotherLongLongProp = 'Hello, World!',\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp,\n        [\n          longLongProp,\n          longLongProp,\n          [\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp,\n          ],\n          yetAnotherLongLongProp,\n        ],\n        yetAnotherLongLongProp,\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp,\n        [\n          longLongProp,\n          longLongProp = 'Hello, Prettier!',\n          [\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp = [\n              longLongProp,\n              longLongProp,\n              anotherLongLongProp,\n              yetAnotherLongLongProp,\n            ],\n            yetAnotherLongLongProp,\n          ],\n          yetAnotherLongLongProp,\n        ],\n        yetAnotherLongLongProp,\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        { longLongProp, longLongProp },\n        { longLongProp, longLongProp },\n        [\n          {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp,\n          },\n          longLongProp,\n        ],\n        yetAnotherLongLongProp,\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp,\n        },\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue = { longLongProp, longLongProp }, secondValue },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue, secondValue, thirdValue, fourthValue }, objectKey, index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue, secondValue, thirdValue, fourthValue }, objectKey, index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp,\n        },\n        sixthValue,\n        seventhValue,\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp,\n        },\n        sixthValue: {\n          firstValue,\n          secondValue,\n          thirdValue,\n          fourthValue,\n          fifthValue: {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp,\n          },\n          sixthValue,\n          seventhValue,\n        },\n        seventhValue,\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp = {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp,\n          },\n          yetAnotherLongLongProp,\n        },\n        sixthValue,\n        seventhValue,\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"item in items\"\n    v-for=\"item of items\"\n    v-for=\"(item, index) in items\"\n    v-for=\"value in object\"\n    v-for=\"(value, key) in object\"\n    v-for=\"(value, key) of object\"\n    v-for=\"(value, key, index) in object\"\n    v-for=\"n in evenNumbers\"\n    v-for=\"n in even(numbers)\"\n    v-for=\"n in 10\"\n    v-for=\"{ a } in [0].map(() => ({ a: 1 }))\"\n    v-for=\"({ a }, [c]) in [0].map(() => 1)\"\n    v-for=\"n in items.map((x) => {\n      return x;\n    })\"\n    @click=\"/* hello */\"\n    @click=\"/* 1 */ $emit(/* 2 */ 'click' /* 3 */) /* 4 */ /* 5 */\"\n    @click=\"$emit('click')\"\n    @click=\"$emit('click')\"\n    @click=\"\n      $emit('click');\n      if (something) {\n        for (let i = j; i < 100; i++) {}\n      } else {\n      }\n    \"\n    slot-scope=\"foo\"\n    slot-scope=\"{ row }\"\n    slot-scope=\"{\n      destructuring: {\n        a: { b },\n      },\n    }\"\n    #default=\"foo\"\n    #default=\"{ row }\"\n    #default=\"{\n      destructuring: {\n        a: { b },\n      },\n    }\"\n    v-slot=\"foo\"\n    v-slot=\"{ row }\"\n    v-slot=\"{\n      destructuring: {\n        a: { b },\n      },\n    }\"\n    v-slot:name=\"foo\"\n    v-slot:name=\"{ row }\"\n    v-slot:name=\"{\n      destructuring: {\n        a: { b },\n      },\n    }\"\n    :class=\"{\n      longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: true,\n    }\"\n    :class=\"\n      (() => {\n        return 'hello';\n      })()\n    \"\n    :key=\"index /* hello */\"\n    :key=\"\n      index // hello\n    \"\n    @click=\"\n      () => {\n        console.log(test);\n      }\n    \"\n    @click=\"\n      () => {\n        console.log(test);\n      }\n    \"\n    @click=\"doSomething()\"\n    @click=\"doSomething;\"\n    @click=\"a.b;\"\n    @click=\"a[1];\"\n    @click=\"a['b'];\"\n    @click=\"a[null]\"\n    #default=\"{\n      foo: {\n        bar: { baz },\n      },\n    }\"\n  ></div>\n</template>");
    Ok(())
}
#[test]
fn test_attributes_vue_trailing_commanone_format_1_b687bddd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<div\n  v-for=\"({ longLongProp, longLongProp }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp=42, longLongProp='Hello, World!' }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp }) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp=42, anotherLongLongProp, yetAnotherLongLongProp='Hello, World!'], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, [longLongProp, longLongProp, [longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([longLongProp, longLongProp, [longLongProp, longLongProp='Hello, Prettier!', [longLongProp, longLongProp, anotherLongLongProp=[longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"([{ longLongProp, longLongProp }, { longLongProp, longLongProp }, [{ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, longLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n  v-for=\"({firstValue, secondValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue={ longLongProp, longLongProp }, secondValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue: {firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, sixthValue, seventhValue}, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"({firstValue, secondValue, thirdValue, fourthValue, fifthValue: { longLongProp, longLongProp, anotherLongLongProp={ longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp }, yetAnotherLongLongProp }, sixthValue, seventhValue}, objectKey, index) in objectWithAVeryVeryVeryVeryLongName\"\n  v-for=\"  item  in  items \"\n  v-for=\"  item  of  items \"\n  v-for=\"(    item    , index)    in    items\"\n  v-for=\"value    in     object\"\n  v-for=\"(value,    key)    in    object\"\n  v-for=\"(value,    key)    of    object\"\n  v-for=\"(value  ,   key,   index)    in   object\"\n  v-for=\"   n    in   evenNumbers\"\n  v-for=\" n in even   ( numbers) \"\n  v-for=\" n    in    10\"\n  v-for=\" { a }    in    [0].map(()=>({a:1}))   \"\n  v-for=\" ({ a }, [c  ])    in    [0].map(()=>1)   \"\n  v-for=\" n in items.map(x => { return x }) \"\n  @click=\"  /* hello */   \"\n  @click=\"   /* 1 */ $emit( /* 2 */ 'click' /* 3 */ ) /* 4 */ ; /* 5 */   \"\n  @click=\"   $emit(   'click'   )   \"\n  @click=\"   $emit(   'click'   )  ;\"\n  @click=\"   $emit(   'click'   )  ;if(something){for(let i=j;i<100;i++){}}else{}\"\n  slot-scope=\"     foo\"\n  slot-scope=\"     {row   }\"\n  slot-scope=\"{destructuring:{   a:{b}}}\"\n  #default=\"     foo\"\n  #default=\"     {row   }\"\n  #default=\"{destructuring:{   a:{b}}}\"\n  v-slot=\"     foo\"\n  v-slot=\"     {row   }\"\n  v-slot=\"{destructuring:{   a:{b}}}\"\n  v-slot:name=\"     foo\"\n  v-slot:name=\"     {row   }\"\n  v-slot:name=\"{destructuring:{   a:{b}}}\"\n  :class=\"{ longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: true }\"\n  :class=\"(() => { return 'hello' })()\"\n  :key=\"index /* hello */ \"\n  :key=\"index // hello \"\n  @click=\"() => {console.log(test)}\"\n  @click=\"\n    () => {\n      console.log(test);\n    }\n  \"\n  @click=\"doSomething()\"\n  @click=\"doSomething;\"\n  @click=\"a.b;\"\n  @click=\"a[1];\"\n  @click=\"a['b'];\"\n  @click=\"a[null];\"\n  #default=\"{foo:{bar:{baz}}}\"\n></div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div\n    v-for=\"(\n      { longLongProp, longLongProp }, index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      { longLongProp = 42, longLongProp = 'Hello, World!' }, index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"{\n      longLongProp,\n      longLongProp\n    } of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      {\n        longLongProp,\n        longLongProp,\n        anotherLongLongProp,\n        yetAnotherLongLongProp\n      },\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"{\n      longLongProp,\n      longLongProp,\n      anotherLongLongProp,\n      yetAnotherLongLongProp\n    } of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp = 42,\n        anotherLongLongProp,\n        yetAnotherLongLongProp = 'Hello, World!'\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp,\n        [\n          longLongProp,\n          longLongProp,\n          [\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp\n          ],\n          yetAnotherLongLongProp\n        ],\n        yetAnotherLongLongProp\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp,\n        [\n          longLongProp,\n          longLongProp = 'Hello, Prettier!',\n          [\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp = [\n              longLongProp,\n              longLongProp,\n              anotherLongLongProp,\n              yetAnotherLongLongProp\n            ],\n            yetAnotherLongLongProp\n          ],\n          yetAnotherLongLongProp\n        ],\n        yetAnotherLongLongProp\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      [\n        { longLongProp, longLongProp },\n        { longLongProp, longLongProp },\n        [\n          {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp\n          },\n          longLongProp\n        ],\n        yetAnotherLongLongProp\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp\n        }\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue = { longLongProp, longLongProp }, secondValue },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue, secondValue, thirdValue, fourthValue }, objectKey, index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      { firstValue, secondValue, thirdValue, fourthValue }, objectKey, index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp\n        },\n        sixthValue,\n        seventhValue\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp,\n          yetAnotherLongLongProp\n        },\n        sixthValue: {\n          firstValue,\n          secondValue,\n          thirdValue,\n          fourthValue,\n          fifthValue: {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp\n          },\n          sixthValue,\n          seventhValue\n        },\n        seventhValue\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"(\n      {\n        firstValue,\n        secondValue,\n        thirdValue,\n        fourthValue,\n        fifthValue: {\n          longLongProp,\n          longLongProp,\n          anotherLongLongProp = {\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp,\n            yetAnotherLongLongProp\n          },\n          yetAnotherLongLongProp\n        },\n        sixthValue,\n        seventhValue\n      },\n      objectKey,\n      index\n    ) in objectWithAVeryVeryVeryVeryLongName\"\n    v-for=\"item in items\"\n    v-for=\"item of items\"\n    v-for=\"(item, index) in items\"\n    v-for=\"value in object\"\n    v-for=\"(value, key) in object\"\n    v-for=\"(value, key) of object\"\n    v-for=\"(value, key, index) in object\"\n    v-for=\"n in evenNumbers\"\n    v-for=\"n in even(numbers)\"\n    v-for=\"n in 10\"\n    v-for=\"{ a } in [0].map(() => ({ a: 1 }))\"\n    v-for=\"({ a }, [c]) in [0].map(() => 1)\"\n    v-for=\"n in items.map((x) => {\n      return x;\n    })\"\n    @click=\"/* hello */\"\n    @click=\"/* 1 */ $emit(/* 2 */ 'click' /* 3 */) /* 4 */ /* 5 */\"\n    @click=\"$emit('click')\"\n    @click=\"$emit('click')\"\n    @click=\"\n      $emit('click');\n      if (something) {\n        for (let i = j; i < 100; i++) {}\n      } else {\n      }\n    \"\n    slot-scope=\"foo\"\n    slot-scope=\"{ row }\"\n    slot-scope=\"{\n      destructuring: {\n        a: { b }\n      }\n    }\"\n    #default=\"foo\"\n    #default=\"{ row }\"\n    #default=\"{\n      destructuring: {\n        a: { b }\n      }\n    }\"\n    v-slot=\"foo\"\n    v-slot=\"{ row }\"\n    v-slot=\"{\n      destructuring: {\n        a: { b }\n      }\n    }\"\n    v-slot:name=\"foo\"\n    v-slot:name=\"{ row }\"\n    v-slot:name=\"{\n      destructuring: {\n        a: { b }\n      }\n    }\"\n    :class=\"{\n      longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: true\n    }\"\n    :class=\"\n      (() => {\n        return 'hello';\n      })()\n    \"\n    :key=\"index /* hello */\"\n    :key=\"\n      index // hello\n    \"\n    @click=\"\n      () => {\n        console.log(test);\n      }\n    \"\n    @click=\"\n      () => {\n        console.log(test);\n      }\n    \"\n    @click=\"doSomething()\"\n    @click=\"doSomething;\"\n    @click=\"a.b;\"\n    @click=\"a[1];\"\n    @click=\"a['b'];\"\n    @click=\"a[null]\"\n    #default=\"{\n      foo: {\n        bar: { baz }\n      }\n    }\"\n  ></div>\n</template>");
    Ok(())
}
#[test]
fn test_board_card_vue_semifalse_format_1_7b682582() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\nimport './issue_card_inner';\nimport eventHub from '../eventhub';\n\nconst Store = gl.issueBoards.BoardsStore;\n\nexport default {\n  name: 'BoardsIssueCard',\n  components: {\n    'issue-card-inner': gl.issueBoards.IssueCardInner,\n  },\n  props: {\n    list: Object,\n    issue: Object,\n    issueLinkBase: String,\n    disabled: Boolean,\n    index: Number,\n    rootPath: String,\n  },\n  data() {\n    return {\n      showDetail: false,\n      detailIssue: Store.detail,\n    };\n  },\n  computed: {\n    issueDetailVisible() {\n      return (\n        this.detailIssue.issue && this.detailIssue.issue.id === this.issue.id\n      );\n    },\n  },\n  methods: {\n    mouseDown() {\n      this.showDetail = true;\n    },\n    mouseMove() {\n      this.showDetail = false;\n    },\n    showIssue(e) {\n      if (e.target.classList.contains('js-no-trigger')) return;\n\n      if (this.showDetail) {\n        this.showDetail = false;\n\n        if (Store.detail.issue && Store.detail.issue.id === this.issue.id) {\n          eventHub.$emit('clearDetailIssue');\n        } else {\n          eventHub.$emit('newDetailIssue', this.issue);\n          Store.detail.list = this.list;\n        }\n      }\n    },\n  },\n};\n</script>\n\n<template>\n  <li class=\"card\"\n    :class=\"{ 'user-can-drag': !disabled && issue.id, 'is-disabled': disabled || !issue.id, 'is-active': issueDetailVisible }\"\n    :index=\"index\"\n    :data-issue-id=\"issue.id\"\n    @mousedown=\"mouseDown\"\n    @mousemove=\"mouseMove\"\n    @mouseup=\"showIssue($event)\">\n    <issue-card-inner\n      :list=\"list\"\n      :issue=\"issue\"\n      :issue-link-base=\"issueLinkBase\"\n      :root-path=\"rootPath\"\n      :update-filters=\"true\" />\n  </li>\n</template>") ? ;
    assert_eq ! (formatted , "<script>\nimport \"./issue_card_inner\"\nimport eventHub from \"../eventhub\"\n\nconst Store = gl.issueBoards.BoardsStore\n\nexport default {\n  name: \"BoardsIssueCard\",\n  components: {\n    \"issue-card-inner\": gl.issueBoards.IssueCardInner,\n  },\n  props: {\n    list: Object,\n    issue: Object,\n    issueLinkBase: String,\n    disabled: Boolean,\n    index: Number,\n    rootPath: String,\n  },\n  data() {\n    return {\n      showDetail: false,\n      detailIssue: Store.detail,\n    }\n  },\n  computed: {\n    issueDetailVisible() {\n      return (\n        this.detailIssue.issue && this.detailIssue.issue.id === this.issue.id\n      )\n    },\n  },\n  methods: {\n    mouseDown() {\n      this.showDetail = true\n    },\n    mouseMove() {\n      this.showDetail = false\n    },\n    showIssue(e) {\n      if (e.target.classList.contains(\"js-no-trigger\")) return\n\n      if (this.showDetail) {\n        this.showDetail = false\n\n        if (Store.detail.issue && Store.detail.issue.id === this.issue.id) {\n          eventHub.$emit(\"clearDetailIssue\")\n        } else {\n          eventHub.$emit(\"newDetailIssue\", this.issue)\n          Store.detail.list = this.list\n        }\n      }\n    },\n  },\n}\n</script>\n\n<template>\n  <li\n    class=\"card\"\n    :class=\"{\n      'user-can-drag': !disabled && issue.id,\n      'is-disabled': disabled || !issue.id,\n      'is-active': issueDetailVisible,\n    }\"\n    :index=\"index\"\n    :data-issue-id=\"issue.id\"\n    @mousedown=\"mouseDown\"\n    @mousemove=\"mouseMove\"\n    @mouseup=\"showIssue($event)\"\n  >\n    <issue-card-inner\n      :list=\"list\"\n      :issue=\"issue\"\n      :issue-link-base=\"issueLinkBase\"\n      :root-path=\"rootPath\"\n      :update-filters=\"true\"\n    />\n  </li>\n</template>");
    Ok(())
}
#[test]
fn test_board_card_vue_trailing_commaes_5_format_1_7b682582() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\nimport './issue_card_inner';\nimport eventHub from '../eventhub';\n\nconst Store = gl.issueBoards.BoardsStore;\n\nexport default {\n  name: 'BoardsIssueCard',\n  components: {\n    'issue-card-inner': gl.issueBoards.IssueCardInner,\n  },\n  props: {\n    list: Object,\n    issue: Object,\n    issueLinkBase: String,\n    disabled: Boolean,\n    index: Number,\n    rootPath: String,\n  },\n  data() {\n    return {\n      showDetail: false,\n      detailIssue: Store.detail,\n    };\n  },\n  computed: {\n    issueDetailVisible() {\n      return (\n        this.detailIssue.issue && this.detailIssue.issue.id === this.issue.id\n      );\n    },\n  },\n  methods: {\n    mouseDown() {\n      this.showDetail = true;\n    },\n    mouseMove() {\n      this.showDetail = false;\n    },\n    showIssue(e) {\n      if (e.target.classList.contains('js-no-trigger')) return;\n\n      if (this.showDetail) {\n        this.showDetail = false;\n\n        if (Store.detail.issue && Store.detail.issue.id === this.issue.id) {\n          eventHub.$emit('clearDetailIssue');\n        } else {\n          eventHub.$emit('newDetailIssue', this.issue);\n          Store.detail.list = this.list;\n        }\n      }\n    },\n  },\n};\n</script>\n\n<template>\n  <li class=\"card\"\n    :class=\"{ 'user-can-drag': !disabled && issue.id, 'is-disabled': disabled || !issue.id, 'is-active': issueDetailVisible }\"\n    :index=\"index\"\n    :data-issue-id=\"issue.id\"\n    @mousedown=\"mouseDown\"\n    @mousemove=\"mouseMove\"\n    @mouseup=\"showIssue($event)\">\n    <issue-card-inner\n      :list=\"list\"\n      :issue=\"issue\"\n      :issue-link-base=\"issueLinkBase\"\n      :root-path=\"rootPath\"\n      :update-filters=\"true\" />\n  </li>\n</template>") ? ;
    assert_eq ! (formatted , "<script>\nimport \"./issue_card_inner\";\nimport eventHub from \"../eventhub\";\n\nconst Store = gl.issueBoards.BoardsStore;\n\nexport default {\n  name: \"BoardsIssueCard\",\n  components: {\n    \"issue-card-inner\": gl.issueBoards.IssueCardInner,\n  },\n  props: {\n    list: Object,\n    issue: Object,\n    issueLinkBase: String,\n    disabled: Boolean,\n    index: Number,\n    rootPath: String,\n  },\n  data() {\n    return {\n      showDetail: false,\n      detailIssue: Store.detail,\n    };\n  },\n  computed: {\n    issueDetailVisible() {\n      return (\n        this.detailIssue.issue && this.detailIssue.issue.id === this.issue.id\n      );\n    },\n  },\n  methods: {\n    mouseDown() {\n      this.showDetail = true;\n    },\n    mouseMove() {\n      this.showDetail = false;\n    },\n    showIssue(e) {\n      if (e.target.classList.contains(\"js-no-trigger\")) return;\n\n      if (this.showDetail) {\n        this.showDetail = false;\n\n        if (Store.detail.issue && Store.detail.issue.id === this.issue.id) {\n          eventHub.$emit(\"clearDetailIssue\");\n        } else {\n          eventHub.$emit(\"newDetailIssue\", this.issue);\n          Store.detail.list = this.list;\n        }\n      }\n    },\n  },\n};\n</script>\n\n<template>\n  <li\n    class=\"card\"\n    :class=\"{\n      'user-can-drag': !disabled && issue.id,\n      'is-disabled': disabled || !issue.id,\n      'is-active': issueDetailVisible,\n    }\"\n    :index=\"index\"\n    :data-issue-id=\"issue.id\"\n    @mousedown=\"mouseDown\"\n    @mousemove=\"mouseMove\"\n    @mouseup=\"showIssue($event)\"\n  >\n    <issue-card-inner\n      :list=\"list\"\n      :issue=\"issue\"\n      :issue-link-base=\"issueLinkBase\"\n      :root-path=\"rootPath\"\n      :update-filters=\"true\"\n    />\n  </li>\n</template>");
    Ok(())
}
#[test]
fn test_board_card_vue_trailing_commanone_format_1_7b682582() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\nimport './issue_card_inner';\nimport eventHub from '../eventhub';\n\nconst Store = gl.issueBoards.BoardsStore;\n\nexport default {\n  name: 'BoardsIssueCard',\n  components: {\n    'issue-card-inner': gl.issueBoards.IssueCardInner,\n  },\n  props: {\n    list: Object,\n    issue: Object,\n    issueLinkBase: String,\n    disabled: Boolean,\n    index: Number,\n    rootPath: String,\n  },\n  data() {\n    return {\n      showDetail: false,\n      detailIssue: Store.detail,\n    };\n  },\n  computed: {\n    issueDetailVisible() {\n      return (\n        this.detailIssue.issue && this.detailIssue.issue.id === this.issue.id\n      );\n    },\n  },\n  methods: {\n    mouseDown() {\n      this.showDetail = true;\n    },\n    mouseMove() {\n      this.showDetail = false;\n    },\n    showIssue(e) {\n      if (e.target.classList.contains('js-no-trigger')) return;\n\n      if (this.showDetail) {\n        this.showDetail = false;\n\n        if (Store.detail.issue && Store.detail.issue.id === this.issue.id) {\n          eventHub.$emit('clearDetailIssue');\n        } else {\n          eventHub.$emit('newDetailIssue', this.issue);\n          Store.detail.list = this.list;\n        }\n      }\n    },\n  },\n};\n</script>\n\n<template>\n  <li class=\"card\"\n    :class=\"{ 'user-can-drag': !disabled && issue.id, 'is-disabled': disabled || !issue.id, 'is-active': issueDetailVisible }\"\n    :index=\"index\"\n    :data-issue-id=\"issue.id\"\n    @mousedown=\"mouseDown\"\n    @mousemove=\"mouseMove\"\n    @mouseup=\"showIssue($event)\">\n    <issue-card-inner\n      :list=\"list\"\n      :issue=\"issue\"\n      :issue-link-base=\"issueLinkBase\"\n      :root-path=\"rootPath\"\n      :update-filters=\"true\" />\n  </li>\n</template>") ? ;
    assert_eq ! (formatted , "<script>\nimport \"./issue_card_inner\";\nimport eventHub from \"../eventhub\";\n\nconst Store = gl.issueBoards.BoardsStore;\n\nexport default {\n  name: \"BoardsIssueCard\",\n  components: {\n    \"issue-card-inner\": gl.issueBoards.IssueCardInner\n  },\n  props: {\n    list: Object,\n    issue: Object,\n    issueLinkBase: String,\n    disabled: Boolean,\n    index: Number,\n    rootPath: String\n  },\n  data() {\n    return {\n      showDetail: false,\n      detailIssue: Store.detail\n    };\n  },\n  computed: {\n    issueDetailVisible() {\n      return (\n        this.detailIssue.issue && this.detailIssue.issue.id === this.issue.id\n      );\n    }\n  },\n  methods: {\n    mouseDown() {\n      this.showDetail = true;\n    },\n    mouseMove() {\n      this.showDetail = false;\n    },\n    showIssue(e) {\n      if (e.target.classList.contains(\"js-no-trigger\")) return;\n\n      if (this.showDetail) {\n        this.showDetail = false;\n\n        if (Store.detail.issue && Store.detail.issue.id === this.issue.id) {\n          eventHub.$emit(\"clearDetailIssue\");\n        } else {\n          eventHub.$emit(\"newDetailIssue\", this.issue);\n          Store.detail.list = this.list;\n        }\n      }\n    }\n  }\n};\n</script>\n\n<template>\n  <li\n    class=\"card\"\n    :class=\"{\n      'user-can-drag': !disabled && issue.id,\n      'is-disabled': disabled || !issue.id,\n      'is-active': issueDetailVisible\n    }\"\n    :index=\"index\"\n    :data-issue-id=\"issue.id\"\n    @mousedown=\"mouseDown\"\n    @mousemove=\"mouseMove\"\n    @mouseup=\"showIssue($event)\"\n  >\n    <issue-card-inner\n      :list=\"list\"\n      :issue=\"issue\"\n      :issue-link-base=\"issueLinkBase\"\n      :root-path=\"rootPath\"\n      :update-filters=\"true\"\n    />\n  </li>\n</template>");
    Ok(())
}
#[test]
fn test_case_sensitive_tags_vue_semifalse_format_1_6ab445a7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template>\n\t<Input></Input>\n</template>")?;
    assert_eq!(formatted, "<template>\n  <Input></Input>\n</template>");
    Ok(())
}
#[test]
fn test_case_sensitive_tags_vue_trailing_commaes_5_format_1_6ab445a7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template>\n\t<Input></Input>\n</template>")?;
    assert_eq!(formatted, "<template>\n  <Input></Input>\n</template>");
    Ok(())
}
#[test]
fn test_case_sensitive_tags_vue_trailing_commanone_format_1_6ab445a7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template>\n\t<Input></Input>\n</template>")?;
    assert_eq!(formatted, "<template>\n  <Input></Input>\n</template>");
    Ok(())
}
#[test]
fn test_custom_block_vue_semifalse_format_1_96430b86() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n>\n  en:\n    one: One\n    two: Two\n</i18n>\n\n<html><head></head><body></body></html>") ? ;
    assert_eq ! (formatted , "<i18n>\n  en:\n    one: One\n    two: Two\n</i18n>\n\n<html>\n  <head></head>\n  <body></body>\n</html>");
    Ok(())
}
#[test]
fn test_custom_block_vue_trailing_commaes_5_format_1_96430b86() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n>\n  en:\n    one: One\n    two: Two\n</i18n>\n\n<html><head></head><body></body></html>") ? ;
    assert_eq ! (formatted , "<i18n>\n  en:\n    one: One\n    two: Two\n</i18n>\n\n<html>\n  <head></head>\n  <body></body>\n</html>");
    Ok(())
}
#[test]
fn test_custom_block_vue_trailing_commanone_format_1_96430b86() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<i18n>\n  en:\n    one: One\n    two: Two\n</i18n>\n\n<html><head></head><body></body></html>") ? ;
    assert_eq ! (formatted , "<i18n>\n  en:\n    one: One\n    two: Two\n</i18n>\n\n<html>\n  <head></head>\n  <body></body>\n</html>");
    Ok(())
}
#[test]
fn test_expression_binding_vue_semifalse_format_1_1096e839() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- #7396 -->\n<template>\n<MyComponent\n:foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n:bar=\"`${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n+`${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\"\n:baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo +               bar} template literal value`\"\n:src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n:add=\"'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' + 'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\"\n:num=\"100000000000000000000000000000000000000000000000000000000000000000000000000\"\n/>\n</template>") ? ;
    assert_eq ! (formatted , "<!-- #7396 -->\n<template>\n  <MyComponent\n    :foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n    :bar=\"\n      `${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}` +\n      `${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n    \"\n    :baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${\n      foo + bar\n    } template literal value`\"\n    :src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n    :add=\"\n      'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' +\n      'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\n    \"\n    :num=\"\n      100000000000000000000000000000000000000000000000000000000000000000000000000\n    \"\n  />\n</template>");
    Ok(())
}
#[test]
fn test_expression_binding_vue_trailing_commaes_5_format_1_1096e839() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- #7396 -->\n<template>\n<MyComponent\n:foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n:bar=\"`${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n+`${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\"\n:baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo +               bar} template literal value`\"\n:src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n:add=\"'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' + 'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\"\n:num=\"100000000000000000000000000000000000000000000000000000000000000000000000000\"\n/>\n</template>") ? ;
    assert_eq ! (formatted , "<!-- #7396 -->\n<template>\n  <MyComponent\n    :foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n    :bar=\"\n      `${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}` +\n      `${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n    \"\n    :baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${\n      foo + bar\n    } template literal value`\"\n    :src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n    :add=\"\n      'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' +\n      'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\n    \"\n    :num=\"\n      100000000000000000000000000000000000000000000000000000000000000000000000000\n    \"\n  />\n</template>");
    Ok(())
}
#[test]
fn test_expression_binding_vue_trailing_commanone_format_1_1096e839() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- #7396 -->\n<template>\n<MyComponent\n:foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n:bar=\"`${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n+`${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\"\n:baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo +               bar} template literal value`\"\n:src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n:add=\"'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' + 'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\"\n:num=\"100000000000000000000000000000000000000000000000000000000000000000000000000\"\n/>\n</template>") ? ;
    assert_eq ! (formatted , "<!-- #7396 -->\n<template>\n  <MyComponent\n    :foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n    :bar=\"\n      `${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}` +\n      `${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n    \"\n    :baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${\n      foo + bar\n    } template literal value`\"\n    :src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n    :add=\"\n      'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' +\n      'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\n    \"\n    :num=\"\n      100000000000000000000000000000000000000000000000000000000000000000000000000\n    \"\n  />\n</template>");
    Ok(())
}
#[test]
fn test_expression_binding_ts_vue_semifalse_format_1_f3bac74c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- #7396 -->\n<template>\n<MyComponent\n:foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n:bar=\"`${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n+`${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\"\n:baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo +               bar} template literal value`\"\n:src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n:add=\"'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' + 'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\"\n:num=\"100000000000000000000000000000000000000000000000000000000000000000000000000\"\n/>\n</template>\n<script lang=\"ts\"></script>") ? ;
    assert_eq ! (formatted , "<!-- #7396 -->\n<template>\n  <MyComponent\n    :foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n    :bar=\"\n      `${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}` +\n      `${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n    \"\n    :baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${\n      foo + bar\n    } template literal value`\"\n    :src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n    :add=\"\n      'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' +\n      'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\n    \"\n    :num=\"\n      100000000000000000000000000000000000000000000000000000000000000000000000000\n    \"\n  />\n</template>\n<script lang=\"ts\"></script>");
    Ok(())
}
#[test]
fn test_expression_binding_ts_vue_trailing_commaes_5_format_1_f3bac74c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- #7396 -->\n<template>\n<MyComponent\n:foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n:bar=\"`${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n+`${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\"\n:baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo +               bar} template literal value`\"\n:src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n:add=\"'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' + 'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\"\n:num=\"100000000000000000000000000000000000000000000000000000000000000000000000000\"\n/>\n</template>\n<script lang=\"ts\"></script>") ? ;
    assert_eq ! (formatted , "<!-- #7396 -->\n<template>\n  <MyComponent\n    :foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n    :bar=\"\n      `${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}` +\n      `${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n    \"\n    :baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${\n      foo + bar\n    } template literal value`\"\n    :src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n    :add=\"\n      'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' +\n      'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\n    \"\n    :num=\"\n      100000000000000000000000000000000000000000000000000000000000000000000000000\n    \"\n  />\n</template>\n<script lang=\"ts\"></script>");
    Ok(())
}
#[test]
fn test_expression_binding_ts_vue_trailing_commanone_format_1_f3bac74c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- #7396 -->\n<template>\n<MyComponent\n:foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n:bar=\"`${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n+`${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\"\n:baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo +               bar} template literal value`\"\n:src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n:add=\"'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' + 'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\"\n:num=\"100000000000000000000000000000000000000000000000000000000000000000000000000\"\n/>\n</template>\n<script lang=\"ts\"></script>") ? ;
    assert_eq ! (formatted , "<!-- #7396 -->\n<template>\n  <MyComponent\n    :foo=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${foo} template literal value`\"\n    :bar=\"\n      `${first} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}` +\n      `${second} loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${template} ${literal}`\n    \"\n    :baz=\"`loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog ${\n      foo + bar\n    } template literal value`\"\n    :src=\"'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog literal string value'\"\n    :add=\"\n      'first loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog' +\n      'second loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooog'\n    \"\n    :num=\"\n      100000000000000000000000000000000000000000000000000000000000000000000000000\n    \"\n  />\n</template>\n<script lang=\"ts\"></script>");
    Ok(())
}
#[test]
fn test_filter_vue_semifalse_format_1_b28caafd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- vue filters are only allowed in v-bind and interpolation -->\n<template>\n  <div class=\"allowed\">{{value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird}}</div>\n  <div class=\"allowed\" v-bind:something='value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird'></div>\n  <div class=\"allowed\" :class='value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird'></div>\n  <div class=\"not-allowed\" v-if='value | thisIsARealSuperLongBitwiseOr(\"arg1\", arg2) | anotherBitwiseOrLongJustForFun | bitwiseOrTheThird'></div>\n</template>") ? ;
    assert_eq ! (formatted , "<!-- vue filters are only allowed in v-bind and interpolation -->\n<template>\n  <div class=\"allowed\">\n    {{\n      value\n        | thisIsARealSuperLongFilterPipe(\"arg1\", arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    }}\n  </div>\n  <div\n    class=\"allowed\"\n    v-bind:something=\"\n      value\n        | thisIsARealSuperLongFilterPipe('arg1', arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    \"\n  ></div>\n  <div\n    class=\"allowed\"\n    :class=\"\n      value\n        | thisIsARealSuperLongFilterPipe('arg1', arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    \"\n  ></div>\n  <div\n    class=\"not-allowed\"\n    v-if=\"\n      value |\n        thisIsARealSuperLongBitwiseOr('arg1', arg2) |\n        anotherBitwiseOrLongJustForFun |\n        bitwiseOrTheThird\n    \"\n  ></div>\n</template>");
    Ok(())
}
#[test]
fn test_filter_vue_trailing_commaes_5_format_1_b28caafd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- vue filters are only allowed in v-bind and interpolation -->\n<template>\n  <div class=\"allowed\">{{value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird}}</div>\n  <div class=\"allowed\" v-bind:something='value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird'></div>\n  <div class=\"allowed\" :class='value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird'></div>\n  <div class=\"not-allowed\" v-if='value | thisIsARealSuperLongBitwiseOr(\"arg1\", arg2) | anotherBitwiseOrLongJustForFun | bitwiseOrTheThird'></div>\n</template>") ? ;
    assert_eq ! (formatted , "<!-- vue filters are only allowed in v-bind and interpolation -->\n<template>\n  <div class=\"allowed\">\n    {{\n      value\n        | thisIsARealSuperLongFilterPipe(\"arg1\", arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    }}\n  </div>\n  <div\n    class=\"allowed\"\n    v-bind:something=\"\n      value\n        | thisIsARealSuperLongFilterPipe('arg1', arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    \"\n  ></div>\n  <div\n    class=\"allowed\"\n    :class=\"\n      value\n        | thisIsARealSuperLongFilterPipe('arg1', arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    \"\n  ></div>\n  <div\n    class=\"not-allowed\"\n    v-if=\"\n      value |\n        thisIsARealSuperLongBitwiseOr('arg1', arg2) |\n        anotherBitwiseOrLongJustForFun |\n        bitwiseOrTheThird\n    \"\n  ></div>\n</template>");
    Ok(())
}
#[test]
fn test_filter_vue_trailing_commanone_format_1_b28caafd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- vue filters are only allowed in v-bind and interpolation -->\n<template>\n  <div class=\"allowed\">{{value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird}}</div>\n  <div class=\"allowed\" v-bind:something='value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird'></div>\n  <div class=\"allowed\" :class='value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2) | anotherPipeLongJustForFun | pipeTheThird'></div>\n  <div class=\"not-allowed\" v-if='value | thisIsARealSuperLongBitwiseOr(\"arg1\", arg2) | anotherBitwiseOrLongJustForFun | bitwiseOrTheThird'></div>\n</template>") ? ;
    assert_eq ! (formatted , "<!-- vue filters are only allowed in v-bind and interpolation -->\n<template>\n  <div class=\"allowed\">\n    {{\n      value\n        | thisIsARealSuperLongFilterPipe(\"arg1\", arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    }}\n  </div>\n  <div\n    class=\"allowed\"\n    v-bind:something=\"\n      value\n        | thisIsARealSuperLongFilterPipe('arg1', arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    \"\n  ></div>\n  <div\n    class=\"allowed\"\n    :class=\"\n      value\n        | thisIsARealSuperLongFilterPipe('arg1', arg2)\n        | anotherPipeLongJustForFun\n        | pipeTheThird\n    \"\n  ></div>\n  <div\n    class=\"not-allowed\"\n    v-if=\"\n      value |\n        thisIsARealSuperLongBitwiseOr('arg1', arg2) |\n        anotherBitwiseOrLongJustForFun |\n        bitwiseOrTheThird\n    \"\n  ></div>\n</template>");
    Ok(())
}
#[test]
fn test_interpolations_vue_semifalse_format_1_c2882a66() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<div>Fuga magnam facilis. Voluptatem quaerat porro.{{\n\n\nx => {\n    const hello = 'world'\n    return hello;\n}\n\n\n\n}} Magni consectetur in et molestias neque esse voluptatibus voluptas. {{  \n    \n    \n    some_variable \n\n\n\n}} Eum quia nihil nulla esse. Dolorem asperiores vero est error {{\n\n                    preserve\n\n                    invalid\n                    \n                    interpolation\n\n}} reprehenderit voluptates minus {{console.log(  short_interpolation )}} nemo.</div>\n\n<script type=\"text/jsx\">\n  export default {\n    render (h) {\n      return (\n        <ul\n          class={{\n            'a': b,\n            'c': d,\n            \"e\": f\n          }}\n        >\n          { this.xyz }\n        </ul>\n    )\n  };\n</script>\n\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890{{ something }}1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890 {{ something }}1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890{{ something }} 1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890 {{ something }} 1234567890\n</div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div>\n    Fuga magnam facilis. Voluptatem quaerat porro.{{\n      (x) => {\n        const hello = \"world\"\n        return hello\n      }\n    }}\n    Magni consectetur in et molestias neque esse voluptatibus voluptas.\n    {{ some_variable }} Eum quia nihil nulla esse. Dolorem asperiores vero est\n    error\n    {{\n\n                    preserve\n\n                    invalid\n                    \n                    interpolation\n\n    }}\n    reprehenderit voluptates minus {{ console.log(short_interpolation) }} nemo.\n  </div>\n\n  <script type=\"text/jsx\">\n    export default {\n      render (h) {\n        return (\n          <ul\n            class={{\n              'a': b,\n              'c': d,\n              \"e\": f\n            }}\n          >\n            { this.xyz }\n          </ul>\n      )\n    };\n  </script>\n\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890{{\n      something\n    }}1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890\n    {{ something }}1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890{{\n      something\n    }}\n    1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890\n    {{ something }} 1234567890\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_interpolations_vue_trailing_commaes_5_format_1_c2882a66() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<div>Fuga magnam facilis. Voluptatem quaerat porro.{{\n\n\nx => {\n    const hello = 'world'\n    return hello;\n}\n\n\n\n}} Magni consectetur in et molestias neque esse voluptatibus voluptas. {{  \n    \n    \n    some_variable \n\n\n\n}} Eum quia nihil nulla esse. Dolorem asperiores vero est error {{\n\n                    preserve\n\n                    invalid\n                    \n                    interpolation\n\n}} reprehenderit voluptates minus {{console.log(  short_interpolation )}} nemo.</div>\n\n<script type=\"text/jsx\">\n  export default {\n    render (h) {\n      return (\n        <ul\n          class={{\n            'a': b,\n            'c': d,\n            \"e\": f\n          }}\n        >\n          { this.xyz }\n        </ul>\n    )\n  };\n</script>\n\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890{{ something }}1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890 {{ something }}1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890{{ something }} 1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890 {{ something }} 1234567890\n</div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div>\n    Fuga magnam facilis. Voluptatem quaerat porro.{{\n      (x) => {\n        const hello = \"world\";\n        return hello;\n      }\n    }}\n    Magni consectetur in et molestias neque esse voluptatibus voluptas.\n    {{ some_variable }} Eum quia nihil nulla esse. Dolorem asperiores vero est\n    error\n    {{\n\n                    preserve\n\n                    invalid\n                    \n                    interpolation\n\n    }}\n    reprehenderit voluptates minus {{ console.log(short_interpolation) }} nemo.\n  </div>\n\n  <script type=\"text/jsx\">\n    export default {\n      render (h) {\n        return (\n          <ul\n            class={{\n              'a': b,\n              'c': d,\n              \"e\": f\n            }}\n          >\n            { this.xyz }\n          </ul>\n      )\n    };\n  </script>\n\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890{{\n      something\n    }}1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890\n    {{ something }}1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890{{\n      something\n    }}\n    1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890\n    {{ something }} 1234567890\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_interpolations_vue_trailing_commanone_format_1_c2882a66() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<div>Fuga magnam facilis. Voluptatem quaerat porro.{{\n\n\nx => {\n    const hello = 'world'\n    return hello;\n}\n\n\n\n}} Magni consectetur in et molestias neque esse voluptatibus voluptas. {{  \n    \n    \n    some_variable \n\n\n\n}} Eum quia nihil nulla esse. Dolorem asperiores vero est error {{\n\n                    preserve\n\n                    invalid\n                    \n                    interpolation\n\n}} reprehenderit voluptates minus {{console.log(  short_interpolation )}} nemo.</div>\n\n<script type=\"text/jsx\">\n  export default {\n    render (h) {\n      return (\n        <ul\n          class={{\n            'a': b,\n            'c': d,\n            \"e\": f\n          }}\n        >\n          { this.xyz }\n        </ul>\n    )\n  };\n</script>\n\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890{{ something }}1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890 {{ something }}1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890{{ something }} 1234567890\n</div>\n<div>\n  1234567890123456789012345678901234567890123456789012345678901234567890 {{ something }} 1234567890\n</div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div>\n    Fuga magnam facilis. Voluptatem quaerat porro.{{\n      (x) => {\n        const hello = \"world\";\n        return hello;\n      }\n    }}\n    Magni consectetur in et molestias neque esse voluptatibus voluptas.\n    {{ some_variable }} Eum quia nihil nulla esse. Dolorem asperiores vero est\n    error\n    {{\n\n                    preserve\n\n                    invalid\n                    \n                    interpolation\n\n    }}\n    reprehenderit voluptates minus {{ console.log(short_interpolation) }} nemo.\n  </div>\n\n  <script type=\"text/jsx\">\n    export default {\n      render (h) {\n        return (\n          <ul\n            class={{\n              'a': b,\n              'c': d,\n              \"e\": f\n            }}\n          >\n            { this.xyz }\n          </ul>\n      )\n    };\n  </script>\n\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890{{\n      something\n    }}1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890\n    {{ something }}1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890{{\n      something\n    }}\n    1234567890\n  </div>\n  <div>\n    1234567890123456789012345678901234567890123456789012345678901234567890\n    {{ something }} 1234567890\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_multiple_template_1_vue_semifalse_format_1_bf438e5a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<template></template>\n<template lang=unknown>\n<a>\n</template>\ncloned.")?;
    assert_eq!(
        formatted,
        "<template></template>\n<template lang=\"unknown\">\n<a>\n</template>\ncloned."
    );
    Ok(())
}
#[test]
fn test_multiple_template_1_vue_trailing_commaes_5_format_1_bf438e5a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<template></template>\n<template lang=unknown>\n<a>\n</template>\ncloned.")?;
    assert_eq!(
        formatted,
        "<template></template>\n<template lang=\"unknown\">\n<a>\n</template>\ncloned."
    );
    Ok(())
}
#[test]
fn test_multiple_template_1_vue_trailing_commanone_format_1_bf438e5a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<template></template>\n<template lang=unknown>\n<a>\n</template>\ncloned.")?;
    assert_eq!(
        formatted,
        "<template></template>\n<template lang=\"unknown\">\n<a>\n</template>\ncloned."
    );
    Ok(())
}
#[test]
fn test_multiple_template_2_vue_semifalse_format_1_30916f11() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<template></template>\n<template lang=unknown>\n<a>\n</template>\n<template></template >",
    )?;
    assert_eq ! (formatted , "<template></template>\n<template lang=\"unknown\">\n<a>\n</template>\n<template></template>");
    Ok(())
}
#[test]
fn test_multiple_template_2_vue_trailing_commaes_5_format_1_30916f11() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<template></template>\n<template lang=unknown>\n<a>\n</template>\n<template></template >",
    )?;
    assert_eq ! (formatted , "<template></template>\n<template lang=\"unknown\">\n<a>\n</template>\n<template></template>");
    Ok(())
}
#[test]
fn test_multiple_template_2_vue_trailing_commanone_format_1_30916f11() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<template></template>\n<template lang=unknown>\n<a>\n</template>\n<template></template >",
    )?;
    assert_eq ! (formatted , "<template></template>\n<template lang=\"unknown\">\n<a>\n</template>\n<template></template>");
    Ok(())
}
#[test]
fn test_nested_template_vue_semifalse_format_1_885d9209() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>\n      <template>\n        <div></div>\n      </template>\n      <div>\n               Do not go out, you'll got yourself cloned by bad bad people.\n      </div>\n  </div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div>\n    <template>\n      <div></div>\n    </template>\n    <div>Do not go out, you'll got yourself cloned by bad bad people.</div>\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_nested_template_vue_trailing_commaes_5_format_1_885d9209() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>\n      <template>\n        <div></div>\n      </template>\n      <div>\n               Do not go out, you'll got yourself cloned by bad bad people.\n      </div>\n  </div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div>\n    <template>\n      <div></div>\n    </template>\n    <div>Do not go out, you'll got yourself cloned by bad bad people.</div>\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_nested_template_vue_trailing_commanone_format_1_885d9209() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>\n      <template>\n        <div></div>\n      </template>\n      <div>\n               Do not go out, you'll got yourself cloned by bad bad people.\n      </div>\n  </div>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div>\n    <template>\n      <div></div>\n    </template>\n    <div>Do not go out, you'll got yourself cloned by bad bad people.</div>\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_one_line_template_1_vue_semifalse_format_1_71e8ec8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template><p>foo</p><div>foo</div></template>")?;
    assert_eq!(
        formatted,
        "<template>\n  <p>foo</p>\n  <div>foo</div>\n</template>"
    );
    Ok(())
}
#[test]
fn test_one_line_template_1_vue_trailing_commaes_5_format_1_71e8ec8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template><p>foo</p><div>foo</div></template>")?;
    assert_eq!(
        formatted,
        "<template>\n  <p>foo</p>\n  <div>foo</div>\n</template>"
    );
    Ok(())
}
#[test]
fn test_one_line_template_1_vue_trailing_commanone_format_1_71e8ec8b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template><p>foo</p><div>foo</div></template>")?;
    assert_eq!(
        formatted,
        "<template>\n  <p>foo</p>\n  <div>foo</div>\n</template>"
    );
    Ok(())
}
#[test]
fn test_one_line_template_2_vue_semifalse_format_1_15b37f58() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<template><div><p>foo</p><div>bar</div></div></template>")?;
    assert_eq!(
        formatted,
        "<template>\n  <div>\n    <p>foo</p>\n    <div>bar</div>\n  </div>\n</template>"
    );
    Ok(())
}
#[test]
fn test_one_line_template_2_vue_trailing_commaes_5_format_1_15b37f58() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<template><div><p>foo</p><div>bar</div></div></template>")?;
    assert_eq!(
        formatted,
        "<template>\n  <div>\n    <p>foo</p>\n    <div>bar</div>\n  </div>\n</template>"
    );
    Ok(())
}
#[test]
fn test_one_line_template_2_vue_trailing_commanone_format_1_15b37f58() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<template><div><p>foo</p><div>bar</div></div></template>")?;
    assert_eq!(
        formatted,
        "<template>\n  <div>\n    <p>foo</p>\n    <div>bar</div>\n  </div>\n</template>"
    );
    Ok(())
}
#[test]
fn test_pre_child_vue_semifalse_format_1_fbb92ce4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/ide/components/jobs/detail.vue -->\n<pre\n  ref=\"buildTrace\"\n  class=\"build-trace mb-0 h-100\"\n  @scroll=\"scrollBuildLog\"\n>\n  <code\n    v-show=\"!detailJob.isLoading\"\n    class=\"bash\"\n    v-html=\"jobOutput\"\n  >\n  </code>\n  <div\n    v-show=\"detailJob.isLoading\"\n    class=\"build-loader-animation\"\n  >\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n  </div>\n</pre>\n</template>\n\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/code_block.vue -->\n<template>\n  <pre class=\"code-block rounded\">\n    <code class=\"d-block\">{{ code }}</code>\n  </pre>\n</template>\n\n<template>\n<pre class='woot'>\n  {{ stuff }}\n  </pre>\n</template>\n\n<template>\n<pre class='woot'>\n  123{{ wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq }}123\n  123{{ wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq }}123\n    </pre>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/ide/components/jobs/detail.vue -->\n  <pre ref=\"buildTrace\" class=\"build-trace mb-0 h-100\" @scroll=\"scrollBuildLog\">\n  <code\n    v-show=\"!detailJob.isLoading\"\n    class=\"bash\"\n    v-html=\"jobOutput\"\n  >\n  </code>\n  <div\n    v-show=\"detailJob.isLoading\"\n    class=\"build-loader-animation\"\n  >\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n  </div>\n</pre>\n</template>\n\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/code_block.vue -->\n<template>\n  <pre class=\"code-block rounded\">\n    <code class=\"d-block\">{{ code }}</code>\n  </pre>\n</template>\n\n<template>\n  <pre class=\"woot\">\n  {{ stuff }}\n  </pre>\n</template>\n\n<template>\n  <pre class=\"woot\">\n  123{{\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq\n    }}123\n  123{{\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq\n    }}123\n    </pre\n  >\n</template>");
    Ok(())
}
#[test]
fn test_pre_child_vue_trailing_commaes_5_format_1_fbb92ce4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/ide/components/jobs/detail.vue -->\n<pre\n  ref=\"buildTrace\"\n  class=\"build-trace mb-0 h-100\"\n  @scroll=\"scrollBuildLog\"\n>\n  <code\n    v-show=\"!detailJob.isLoading\"\n    class=\"bash\"\n    v-html=\"jobOutput\"\n  >\n  </code>\n  <div\n    v-show=\"detailJob.isLoading\"\n    class=\"build-loader-animation\"\n  >\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n  </div>\n</pre>\n</template>\n\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/code_block.vue -->\n<template>\n  <pre class=\"code-block rounded\">\n    <code class=\"d-block\">{{ code }}</code>\n  </pre>\n</template>\n\n<template>\n<pre class='woot'>\n  {{ stuff }}\n  </pre>\n</template>\n\n<template>\n<pre class='woot'>\n  123{{ wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq }}123\n  123{{ wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq }}123\n    </pre>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/ide/components/jobs/detail.vue -->\n  <pre ref=\"buildTrace\" class=\"build-trace mb-0 h-100\" @scroll=\"scrollBuildLog\">\n  <code\n    v-show=\"!detailJob.isLoading\"\n    class=\"bash\"\n    v-html=\"jobOutput\"\n  >\n  </code>\n  <div\n    v-show=\"detailJob.isLoading\"\n    class=\"build-loader-animation\"\n  >\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n  </div>\n</pre>\n</template>\n\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/code_block.vue -->\n<template>\n  <pre class=\"code-block rounded\">\n    <code class=\"d-block\">{{ code }}</code>\n  </pre>\n</template>\n\n<template>\n  <pre class=\"woot\">\n  {{ stuff }}\n  </pre>\n</template>\n\n<template>\n  <pre class=\"woot\">\n  123{{\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq\n    }}123\n  123{{\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq\n    }}123\n    </pre\n  >\n</template>");
    Ok(())
}
#[test]
fn test_pre_child_vue_trailing_commanone_format_1_fbb92ce4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/ide/components/jobs/detail.vue -->\n<pre\n  ref=\"buildTrace\"\n  class=\"build-trace mb-0 h-100\"\n  @scroll=\"scrollBuildLog\"\n>\n  <code\n    v-show=\"!detailJob.isLoading\"\n    class=\"bash\"\n    v-html=\"jobOutput\"\n  >\n  </code>\n  <div\n    v-show=\"detailJob.isLoading\"\n    class=\"build-loader-animation\"\n  >\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n  </div>\n</pre>\n</template>\n\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/code_block.vue -->\n<template>\n  <pre class=\"code-block rounded\">\n    <code class=\"d-block\">{{ code }}</code>\n  </pre>\n</template>\n\n<template>\n<pre class='woot'>\n  {{ stuff }}\n  </pre>\n</template>\n\n<template>\n<pre class='woot'>\n  123{{ wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq }}123\n  123{{ wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq || wqeqwwqwqweqweqwewwq }}123\n    </pre>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/ide/components/jobs/detail.vue -->\n  <pre ref=\"buildTrace\" class=\"build-trace mb-0 h-100\" @scroll=\"scrollBuildLog\">\n  <code\n    v-show=\"!detailJob.isLoading\"\n    class=\"bash\"\n    v-html=\"jobOutput\"\n  >\n  </code>\n  <div\n    v-show=\"detailJob.isLoading\"\n    class=\"build-loader-animation\"\n  >\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n    <div class=\"dot\"></div>\n  </div>\n</pre>\n</template>\n\n<!-- copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/code_block.vue -->\n<template>\n  <pre class=\"code-block rounded\">\n    <code class=\"d-block\">{{ code }}</code>\n  </pre>\n</template>\n\n<template>\n  <pre class=\"woot\">\n  {{ stuff }}\n  </pre>\n</template>\n\n<template>\n  <pre class=\"woot\">\n  123{{\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq\n    }}123\n  123{{\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq ||\n      wqeqwwqwqweqweqwewwq\n    }}123\n    </pre\n  >\n</template>");
    Ok(())
}
#[test]
fn test_script_src_vue_semifalse_format_1_72ce1b9d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase.js\"></script>\n<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase-firestore.js\"></script>") ? ;
    assert_eq ! (formatted , "<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase.js\"></script>\n<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase-firestore.js\"></script>");
    Ok(())
}
#[test]
fn test_script_src_vue_trailing_commaes_5_format_1_72ce1b9d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase.js\"></script>\n<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase-firestore.js\"></script>") ? ;
    assert_eq ! (formatted , "<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase.js\"></script>\n<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase-firestore.js\"></script>");
    Ok(())
}
#[test]
fn test_script_src_vue_trailing_commanone_format_1_72ce1b9d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase.js\"></script>\n<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase-firestore.js\"></script>") ? ;
    assert_eq ! (formatted , "<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase.js\"></script>\n<script src=\"https://www.gstatic.com/firebasejs/4.10.1/firebase-firestore.js\"></script>");
    Ok(())
}
#[test]
fn test_self_closing_vue_semifalse_format_1_fb2b536a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div />\n</template>\n\n<script>\nfoo( )\n</script>\n\n<template>\n<div class=\"container\">\n  <HomeH />\n  <HomeA />\n  <HomeX />\n  <HomeY />\n</div>\n</template>\n\n<template>\n  <Foo><Bar\n         attr\n                                     /></Foo>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div />\n</template>\n\n<script>\nfoo()\n</script>\n\n<template>\n  <div class=\"container\">\n    <HomeH />\n    <HomeA />\n    <HomeX />\n    <HomeY />\n  </div>\n</template>\n\n<template>\n  <Foo><Bar attr /></Foo>\n</template>");
    Ok(())
}
#[test]
fn test_self_closing_vue_trailing_commaes_5_format_1_fb2b536a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div />\n</template>\n\n<script>\nfoo( )\n</script>\n\n<template>\n<div class=\"container\">\n  <HomeH />\n  <HomeA />\n  <HomeX />\n  <HomeY />\n</div>\n</template>\n\n<template>\n  <Foo><Bar\n         attr\n                                     /></Foo>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div />\n</template>\n\n<script>\nfoo();\n</script>\n\n<template>\n  <div class=\"container\">\n    <HomeH />\n    <HomeA />\n    <HomeX />\n    <HomeY />\n  </div>\n</template>\n\n<template>\n  <Foo><Bar attr /></Foo>\n</template>");
    Ok(())
}
#[test]
fn test_self_closing_vue_trailing_commanone_format_1_fb2b536a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div />\n</template>\n\n<script>\nfoo( )\n</script>\n\n<template>\n<div class=\"container\">\n  <HomeH />\n  <HomeA />\n  <HomeX />\n  <HomeY />\n</div>\n</template>\n\n<template>\n  <Foo><Bar\n         attr\n                                     /></Foo>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <div />\n</template>\n\n<script>\nfoo();\n</script>\n\n<template>\n  <div class=\"container\">\n    <HomeH />\n    <HomeA />\n    <HomeX />\n    <HomeY />\n  </div>\n</template>\n\n<template>\n  <Foo><Bar attr /></Foo>\n</template>");
    Ok(())
}
#[test]
fn test_self_closing_style_vue_semifalse_format_1_2a01d682() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <span :class=\"$style.root\"><slot /></span>\n</template>\n\n<style src=\"./style.css\" module />") ? ;
    assert_eq ! (formatted , "<template>\n  <span :class=\"$style.root\"><slot /></span>\n</template>\n\n<style src=\"./style.css\" module />");
    Ok(())
}
#[test]
fn test_self_closing_style_vue_trailing_commaes_5_format_1_2a01d682() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <span :class=\"$style.root\"><slot /></span>\n</template>\n\n<style src=\"./style.css\" module />") ? ;
    assert_eq ! (formatted , "<template>\n  <span :class=\"$style.root\"><slot /></span>\n</template>\n\n<style src=\"./style.css\" module />");
    Ok(())
}
#[test]
fn test_self_closing_style_vue_trailing_commanone_format_1_2a01d682() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <span :class=\"$style.root\"><slot /></span>\n</template>\n\n<style src=\"./style.css\" module />") ? ;
    assert_eq ! (formatted , "<template>\n  <span :class=\"$style.root\"><slot /></span>\n</template>\n\n<style src=\"./style.css\" module />");
    Ok(())
}
#[test]
fn test_slot_ts_vue_semifalse_format_1_cdba1545() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"ts\"></script>\n<template>\n  <comp>\n    <template\n    v-slot=\"x:    unknown   []\"\n    v-slot:named=\"   y  :     never\"\n    slot-scope=\" { des: {truc: tured  } } : any \"\n    #shorthand=\" abc   : string  \"\n    >\n      {{ x[1] }}\n    </template>\n  </comp>\n</template>") ? ;
    assert_eq ! (formatted , "<script lang=\"ts\"></script>\n<template>\n  <comp>\n    <template\n      v-slot=\"x: unknown[]\"\n      v-slot:named=\"y: never\"\n      slot-scope=\"{ des: { truc: tured } }: any\"\n      #shorthand=\"abc: string\"\n    >\n      {{ x[1] }}\n    </template>\n  </comp>\n</template>");
    Ok(())
}
#[test]
fn test_slot_ts_vue_trailing_commaes_5_format_1_cdba1545() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"ts\"></script>\n<template>\n  <comp>\n    <template\n    v-slot=\"x:    unknown   []\"\n    v-slot:named=\"   y  :     never\"\n    slot-scope=\" { des: {truc: tured  } } : any \"\n    #shorthand=\" abc   : string  \"\n    >\n      {{ x[1] }}\n    </template>\n  </comp>\n</template>") ? ;
    assert_eq ! (formatted , "<script lang=\"ts\"></script>\n<template>\n  <comp>\n    <template\n      v-slot=\"x: unknown[]\"\n      v-slot:named=\"y: never\"\n      slot-scope=\"{ des: { truc: tured } }: any\"\n      #shorthand=\"abc: string\"\n    >\n      {{ x[1] }}\n    </template>\n  </comp>\n</template>");
    Ok(())
}
#[test]
fn test_slot_ts_vue_trailing_commanone_format_1_cdba1545() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"ts\"></script>\n<template>\n  <comp>\n    <template\n    v-slot=\"x:    unknown   []\"\n    v-slot:named=\"   y  :     never\"\n    slot-scope=\" { des: {truc: tured  } } : any \"\n    #shorthand=\" abc   : string  \"\n    >\n      {{ x[1] }}\n    </template>\n  </comp>\n</template>") ? ;
    assert_eq ! (formatted , "<script lang=\"ts\"></script>\n<template>\n  <comp>\n    <template\n      v-slot=\"x: unknown[]\"\n      v-slot:named=\"y: never\"\n      slot-scope=\"{ des: { truc: tured } }: any\"\n      #shorthand=\"abc: string\"\n    >\n      {{ x[1] }}\n    </template>\n  </comp>\n</template>");
    Ok(())
}
#[test]
fn test_style_vue_semifalse_format_1_a038f778() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\n</script>\n\n<style>\n#foo1{ color: #f00;\n}\n</style>\n<style scoped>\n#foo2{ color: #f00;\n}\n</style>\n\n<style lang=\"css\">\n#foo3{ color: #f00;\n}\n</style>\n<style lang=\"css\" scoped>\n#foo4{ color: #f00;\n}\n</style>\n\n<style lang=\"less\">\n#foo5{ color: #f00;\n}\n</style>\n<style lang=\"less\" scoped>\n#foo6{ \n         @color: #f00;\n  color: @color;\n}\n</style>\n\n\n<style lang=\"scss\">\n#foo8{ \n         $color: #f00;\n  color: $color;\n}\n</style>\n<style lang=\"scss\" scoped>\n#foo8{ \n         $color: #f00;\n  color: $color;\n}\n</style>\n") ? ;
    assert_eq ! (formatted , "<script></script>\n\n<style>\n#foo1 {\n  color: #f00;\n}\n</style>\n<style scoped>\n#foo2 {\n  color: #f00;\n}\n</style>\n\n<style lang=\"css\">\n#foo3 {\n  color: #f00;\n}\n</style>\n<style lang=\"css\" scoped>\n#foo4 {\n  color: #f00;\n}\n</style>\n\n<style lang=\"less\">\n#foo5 {\n  color: #f00;\n}\n</style>\n<style lang=\"less\" scoped>\n#foo6 {\n  @color: #f00;\n  color: @color;\n}\n</style>\n\n<style lang=\"scss\">\n#foo8 {\n  $color: #f00;\n  color: $color;\n}\n</style>\n<style lang=\"scss\" scoped>\n#foo8 {\n  $color: #f00;\n  color: $color;\n}\n</style>");
    Ok(())
}
#[test]
fn test_style_vue_trailing_commaes_5_format_1_a038f778() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\n</script>\n\n<style>\n#foo1{ color: #f00;\n}\n</style>\n<style scoped>\n#foo2{ color: #f00;\n}\n</style>\n\n<style lang=\"css\">\n#foo3{ color: #f00;\n}\n</style>\n<style lang=\"css\" scoped>\n#foo4{ color: #f00;\n}\n</style>\n\n<style lang=\"less\">\n#foo5{ color: #f00;\n}\n</style>\n<style lang=\"less\" scoped>\n#foo6{ \n         @color: #f00;\n  color: @color;\n}\n</style>\n\n\n<style lang=\"scss\">\n#foo8{ \n         $color: #f00;\n  color: $color;\n}\n</style>\n<style lang=\"scss\" scoped>\n#foo8{ \n         $color: #f00;\n  color: $color;\n}\n</style>\n") ? ;
    assert_eq ! (formatted , "<script></script>\n\n<style>\n#foo1 {\n  color: #f00;\n}\n</style>\n<style scoped>\n#foo2 {\n  color: #f00;\n}\n</style>\n\n<style lang=\"css\">\n#foo3 {\n  color: #f00;\n}\n</style>\n<style lang=\"css\" scoped>\n#foo4 {\n  color: #f00;\n}\n</style>\n\n<style lang=\"less\">\n#foo5 {\n  color: #f00;\n}\n</style>\n<style lang=\"less\" scoped>\n#foo6 {\n  @color: #f00;\n  color: @color;\n}\n</style>\n\n<style lang=\"scss\">\n#foo8 {\n  $color: #f00;\n  color: $color;\n}\n</style>\n<style lang=\"scss\" scoped>\n#foo8 {\n  $color: #f00;\n  color: $color;\n}\n</style>");
    Ok(())
}
#[test]
fn test_style_vue_trailing_commanone_format_1_a038f778() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\n</script>\n\n<style>\n#foo1{ color: #f00;\n}\n</style>\n<style scoped>\n#foo2{ color: #f00;\n}\n</style>\n\n<style lang=\"css\">\n#foo3{ color: #f00;\n}\n</style>\n<style lang=\"css\" scoped>\n#foo4{ color: #f00;\n}\n</style>\n\n<style lang=\"less\">\n#foo5{ color: #f00;\n}\n</style>\n<style lang=\"less\" scoped>\n#foo6{ \n         @color: #f00;\n  color: @color;\n}\n</style>\n\n\n<style lang=\"scss\">\n#foo8{ \n         $color: #f00;\n  color: $color;\n}\n</style>\n<style lang=\"scss\" scoped>\n#foo8{ \n         $color: #f00;\n  color: $color;\n}\n</style>\n") ? ;
    assert_eq ! (formatted , "<script></script>\n\n<style>\n#foo1 {\n  color: #f00;\n}\n</style>\n<style scoped>\n#foo2 {\n  color: #f00;\n}\n</style>\n\n<style lang=\"css\">\n#foo3 {\n  color: #f00;\n}\n</style>\n<style lang=\"css\" scoped>\n#foo4 {\n  color: #f00;\n}\n</style>\n\n<style lang=\"less\">\n#foo5 {\n  color: #f00;\n}\n</style>\n<style lang=\"less\" scoped>\n#foo6 {\n  @color: #f00;\n  color: @color;\n}\n</style>\n\n<style lang=\"scss\">\n#foo8 {\n  $color: #f00;\n  color: $color;\n}\n</style>\n<style lang=\"scss\" scoped>\n#foo8 {\n  $color: #f00;\n  color: $color;\n}\n</style>");
    Ok(())
}
#[test]
fn test_tag_name_vue_semifalse_format_1_54a88e99() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template>\n  <Table></Table>\n</template>")?;
    assert_eq!(formatted, "<template>\n  <Table></Table>\n</template>");
    Ok(())
}
#[test]
fn test_tag_name_vue_trailing_commaes_5_format_1_54a88e99() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template>\n  <Table></Table>\n</template>")?;
    assert_eq!(formatted, "<template>\n  <Table></Table>\n</template>");
    Ok(())
}
#[test]
fn test_tag_name_vue_trailing_commanone_format_1_54a88e99() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<template>\n  <Table></Table>\n</template>")?;
    assert_eq!(formatted, "<template>\n  <Table></Table>\n</template>");
    Ok(())
}
#[test]
fn test_template_vue_semifalse_format_1_a1b5b80e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!--copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/content_viewer/viewers/image_viewer.vue-->\n<template>\n  <div class=\"file-container\">\n    <div class=\"file-content image_file\">\n      <img\n        ref=\"contentImg\"\n        :class=\"{ 'is-zoomable': isZoomable, 'is-zoomed': isZoomed }\"\n        :src=\"path\"\n        :alt=\"path\"\n        @load=\"onImgLoad\"\n        @click=\"onImgClick\"/>\n      <p\n        v-if=\"renderInfo\"\n        class=\"file-info prepend-top-10\">\n        <template v-if=\"fileSize>0\">\n          {{ fileSizeReadable }}\n        </template>\n        <template v-if=\"fileSize>0 && width && height\">\n          |\n        </template>\n        <template v-if=\"width && height\">\n          W: {{ width }} | H: {{ height }}\n        </template>\n      </p>\n    </div>\n  </div>\n</template>") ? ;
    assert_eq ! (formatted , "<!--copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/content_viewer/viewers/image_viewer.vue-->\n<template>\n  <div class=\"file-container\">\n    <div class=\"file-content image_file\">\n      <img\n        ref=\"contentImg\"\n        :class=\"{ 'is-zoomable': isZoomable, 'is-zoomed': isZoomed }\"\n        :src=\"path\"\n        :alt=\"path\"\n        @load=\"onImgLoad\"\n        @click=\"onImgClick\"\n      />\n      <p v-if=\"renderInfo\" class=\"file-info prepend-top-10\">\n        <template v-if=\"fileSize > 0\">\n          {{ fileSizeReadable }}\n        </template>\n        <template v-if=\"fileSize > 0 && width && height\"> | </template>\n        <template v-if=\"width && height\">\n          W: {{ width }} | H: {{ height }}\n        </template>\n      </p>\n    </div>\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_template_vue_trailing_commaes_5_format_1_a1b5b80e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!--copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/content_viewer/viewers/image_viewer.vue-->\n<template>\n  <div class=\"file-container\">\n    <div class=\"file-content image_file\">\n      <img\n        ref=\"contentImg\"\n        :class=\"{ 'is-zoomable': isZoomable, 'is-zoomed': isZoomed }\"\n        :src=\"path\"\n        :alt=\"path\"\n        @load=\"onImgLoad\"\n        @click=\"onImgClick\"/>\n      <p\n        v-if=\"renderInfo\"\n        class=\"file-info prepend-top-10\">\n        <template v-if=\"fileSize>0\">\n          {{ fileSizeReadable }}\n        </template>\n        <template v-if=\"fileSize>0 && width && height\">\n          |\n        </template>\n        <template v-if=\"width && height\">\n          W: {{ width }} | H: {{ height }}\n        </template>\n      </p>\n    </div>\n  </div>\n</template>") ? ;
    assert_eq ! (formatted , "<!--copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/content_viewer/viewers/image_viewer.vue-->\n<template>\n  <div class=\"file-container\">\n    <div class=\"file-content image_file\">\n      <img\n        ref=\"contentImg\"\n        :class=\"{ 'is-zoomable': isZoomable, 'is-zoomed': isZoomed }\"\n        :src=\"path\"\n        :alt=\"path\"\n        @load=\"onImgLoad\"\n        @click=\"onImgClick\"\n      />\n      <p v-if=\"renderInfo\" class=\"file-info prepend-top-10\">\n        <template v-if=\"fileSize > 0\">\n          {{ fileSizeReadable }}\n        </template>\n        <template v-if=\"fileSize > 0 && width && height\"> | </template>\n        <template v-if=\"width && height\">\n          W: {{ width }} | H: {{ height }}\n        </template>\n      </p>\n    </div>\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_template_vue_trailing_commanone_format_1_a1b5b80e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!--copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/content_viewer/viewers/image_viewer.vue-->\n<template>\n  <div class=\"file-container\">\n    <div class=\"file-content image_file\">\n      <img\n        ref=\"contentImg\"\n        :class=\"{ 'is-zoomable': isZoomable, 'is-zoomed': isZoomed }\"\n        :src=\"path\"\n        :alt=\"path\"\n        @load=\"onImgLoad\"\n        @click=\"onImgClick\"/>\n      <p\n        v-if=\"renderInfo\"\n        class=\"file-info prepend-top-10\">\n        <template v-if=\"fileSize>0\">\n          {{ fileSizeReadable }}\n        </template>\n        <template v-if=\"fileSize>0 && width && height\">\n          |\n        </template>\n        <template v-if=\"width && height\">\n          W: {{ width }} | H: {{ height }}\n        </template>\n      </p>\n    </div>\n  </div>\n</template>") ? ;
    assert_eq ! (formatted , "<!--copied from https://github.com/gitlabhq/gitlabhq/blob/master/app/assets/javascripts/vue_shared/components/content_viewer/viewers/image_viewer.vue-->\n<template>\n  <div class=\"file-container\">\n    <div class=\"file-content image_file\">\n      <img\n        ref=\"contentImg\"\n        :class=\"{ 'is-zoomable': isZoomable, 'is-zoomed': isZoomed }\"\n        :src=\"path\"\n        :alt=\"path\"\n        @load=\"onImgLoad\"\n        @click=\"onImgClick\"\n      />\n      <p v-if=\"renderInfo\" class=\"file-info prepend-top-10\">\n        <template v-if=\"fileSize > 0\">\n          {{ fileSizeReadable }}\n        </template>\n        <template v-if=\"fileSize > 0 && width && height\"> | </template>\n        <template v-if=\"width && height\">\n          W: {{ width }} | H: {{ height }}\n        </template>\n      </p>\n    </div>\n  </div>\n</template>");
    Ok(())
}
#[test]
fn test_template_dom_html_semifalse_format_1_6ef2e38a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html><html>\n  <body>\n    <div v-if=\"foo ===    'foo'\">\n\n</div>\n    <script>\nnew Vue({el: '#app'})\n    </script>\n  </body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <div v-if=\"foo === 'foo'\"></div>\n    <script>\n      new Vue({ el: \"#app\" })\n    </script>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_template_dom_html_trailing_commaes_5_format_1_6ef2e38a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html><html>\n  <body>\n    <div v-if=\"foo ===    'foo'\">\n\n</div>\n    <script>\nnew Vue({el: '#app'})\n    </script>\n  </body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <div v-if=\"foo === 'foo'\"></div>\n    <script>\n      new Vue({ el: \"#app\" });\n    </script>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_template_dom_html_trailing_commanone_format_1_6ef2e38a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html><html>\n  <body>\n    <div v-if=\"foo ===    'foo'\">\n\n</div>\n    <script>\nnew Vue({el: '#app'})\n    </script>\n  </body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <div v-if=\"foo === 'foo'\"></div>\n    <script>\n      new Vue({ el: \"#app\" });\n    </script>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_template_lang_vue_semifalse_format_1_1c55fca2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template \n\n\n   \n   lang='pug'>\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"unknown\">\n  #container\n    some-component(tag='<some-tag>')\n</template>\n\n<template lang=\"\">\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>\n\n<template lang>\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>") ? ;
    assert_eq ! (formatted , "<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"unknown\">\n  #container\n    some-component(tag='<some-tag>')\n</template>\n\n<template lang=\"\">\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>\n\n<template lang>\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>");
    Ok(())
}
#[test]
fn test_template_lang_vue_trailing_commaes_5_format_1_1c55fca2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template \n\n\n   \n   lang='pug'>\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"unknown\">\n  #container\n    some-component(tag='<some-tag>')\n</template>\n\n<template lang=\"\">\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>\n\n<template lang>\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>") ? ;
    assert_eq ! (formatted , "<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"unknown\">\n  #container\n    some-component(tag='<some-tag>')\n</template>\n\n<template lang=\"\">\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>\n\n<template lang>\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>");
    Ok(())
}
#[test]
fn test_template_lang_vue_trailing_commanone_format_1_1c55fca2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template \n\n\n   \n   lang='pug'>\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"unknown\">\n  #container\n    some-component(tag='<some-tag>')\n</template>\n\n<template lang=\"\">\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>\n\n<template lang>\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>") ? ;
    assert_eq ! (formatted , "<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"pug\">\n  .test\n    #foo\n  .bla\n</template>\n\n<template lang=\"unknown\">\n  #container\n    some-component(tag='<some-tag>')\n</template>\n\n<template lang=\"\">\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>\n\n<template lang>\n  <v-app-bar>\n    <v-menu offset-y>\n      <template></template>\n    </v-menu>\n  </v-app-bar>\n</template>");
    Ok(())
}
#[test]
fn test_test_vue_semifalse_format_1_a1e4ec14() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\n</script>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br/>\n  </footer>\n</template>") ? ;
    assert_eq ! (formatted , "<script></script>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br />\n  </footer>\n</template>");
    Ok(())
}
#[test]
fn test_test_vue_trailing_commaes_5_format_1_a1e4ec14() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\n</script>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br/>\n  </footer>\n</template>") ? ;
    assert_eq ! (formatted , "<script></script>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br />\n  </footer>\n</template>");
    Ok(())
}
#[test]
fn test_test_vue_trailing_commanone_format_1_a1e4ec14() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>\n</script>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br/>\n  </footer>\n</template>") ? ;
    assert_eq ! (formatted , "<script></script>\n\n<template>\n  <br />\n  <footer>\n    foo\n    <br />\n  </footer>\n</template>");
    Ok(())
}
#[test]
fn test_v_if_vue_semifalse_format_1_57152ca2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<root>\n  <and v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 && short_1 && short_2 &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4\n\"></and>\n  <and v-if=\"\n(long_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 && (short_1 && short_2) ) &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4\n\"></and>\n  <or v-if=\"\nlong_long_long_long_long_long_long_condition_1 || long_long_long_long_long_long_long_condition_2 ||short_1 || short_2 ||\nlong_long_long_long_long_long_long_condition_3 ||\nlong_long_long_long_long_long_long_condition_4\n\"></or>\n  <or v-if=\"\n(long_long_long_long_long_long_long_condition_1 || long_long_long_long_long_long_long_condition_2 || (short_1 || short_2) ||\nlong_long_long_long_long_long_long_condition_3) ||\nlong_long_long_long_long_long_long_condition_4\n\"></or>\n  <mixed v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 ||  ((short_1 && short_2) &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4)\n\"></mixed>\n  <mixed v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 || short_1 && short_2 &&\nlong_long_long_long_long_long_long_condition_3 ||\nlong_long_long_long_long_long_long_condition_4\n\"></mixed>\n</root>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <root>\n    <and\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 &&\n        long_long_long_long_long_long_long_condition_2 &&\n        short_1 &&\n        short_2 &&\n        long_long_long_long_long_long_long_condition_3 &&\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></and>\n    <and\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 &&\n        long_long_long_long_long_long_long_condition_2 &&\n        short_1 &&\n        short_2 &&\n        long_long_long_long_long_long_long_condition_3 &&\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></and>\n    <or\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 ||\n        long_long_long_long_long_long_long_condition_2 ||\n        short_1 ||\n        short_2 ||\n        long_long_long_long_long_long_long_condition_3 ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></or>\n    <or\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 ||\n        long_long_long_long_long_long_long_condition_2 ||\n        short_1 ||\n        short_2 ||\n        long_long_long_long_long_long_long_condition_3 ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></or>\n    <mixed\n      v-if=\"\n        (long_long_long_long_long_long_long_condition_1 &&\n          long_long_long_long_long_long_long_condition_2) ||\n        (short_1 &&\n          short_2 &&\n          long_long_long_long_long_long_long_condition_3 &&\n          long_long_long_long_long_long_long_condition_4)\n      \"\n    ></mixed>\n    <mixed\n      v-if=\"\n        (long_long_long_long_long_long_long_condition_1 &&\n          long_long_long_long_long_long_long_condition_2) ||\n        (short_1 &&\n          short_2 &&\n          long_long_long_long_long_long_long_condition_3) ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></mixed>\n  </root>\n</template>");
    Ok(())
}
#[test]
fn test_v_if_vue_trailing_commaes_5_format_1_57152ca2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<root>\n  <and v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 && short_1 && short_2 &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4\n\"></and>\n  <and v-if=\"\n(long_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 && (short_1 && short_2) ) &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4\n\"></and>\n  <or v-if=\"\nlong_long_long_long_long_long_long_condition_1 || long_long_long_long_long_long_long_condition_2 ||short_1 || short_2 ||\nlong_long_long_long_long_long_long_condition_3 ||\nlong_long_long_long_long_long_long_condition_4\n\"></or>\n  <or v-if=\"\n(long_long_long_long_long_long_long_condition_1 || long_long_long_long_long_long_long_condition_2 || (short_1 || short_2) ||\nlong_long_long_long_long_long_long_condition_3) ||\nlong_long_long_long_long_long_long_condition_4\n\"></or>\n  <mixed v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 ||  ((short_1 && short_2) &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4)\n\"></mixed>\n  <mixed v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 || short_1 && short_2 &&\nlong_long_long_long_long_long_long_condition_3 ||\nlong_long_long_long_long_long_long_condition_4\n\"></mixed>\n</root>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <root>\n    <and\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 &&\n        long_long_long_long_long_long_long_condition_2 &&\n        short_1 &&\n        short_2 &&\n        long_long_long_long_long_long_long_condition_3 &&\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></and>\n    <and\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 &&\n        long_long_long_long_long_long_long_condition_2 &&\n        short_1 &&\n        short_2 &&\n        long_long_long_long_long_long_long_condition_3 &&\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></and>\n    <or\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 ||\n        long_long_long_long_long_long_long_condition_2 ||\n        short_1 ||\n        short_2 ||\n        long_long_long_long_long_long_long_condition_3 ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></or>\n    <or\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 ||\n        long_long_long_long_long_long_long_condition_2 ||\n        short_1 ||\n        short_2 ||\n        long_long_long_long_long_long_long_condition_3 ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></or>\n    <mixed\n      v-if=\"\n        (long_long_long_long_long_long_long_condition_1 &&\n          long_long_long_long_long_long_long_condition_2) ||\n        (short_1 &&\n          short_2 &&\n          long_long_long_long_long_long_long_condition_3 &&\n          long_long_long_long_long_long_long_condition_4)\n      \"\n    ></mixed>\n    <mixed\n      v-if=\"\n        (long_long_long_long_long_long_long_condition_1 &&\n          long_long_long_long_long_long_long_condition_2) ||\n        (short_1 &&\n          short_2 &&\n          long_long_long_long_long_long_long_condition_3) ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></mixed>\n  </root>\n</template>");
    Ok(())
}
#[test]
fn test_v_if_vue_trailing_commanone_format_1_57152ca2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n<root>\n  <and v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 && short_1 && short_2 &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4\n\"></and>\n  <and v-if=\"\n(long_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 && (short_1 && short_2) ) &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4\n\"></and>\n  <or v-if=\"\nlong_long_long_long_long_long_long_condition_1 || long_long_long_long_long_long_long_condition_2 ||short_1 || short_2 ||\nlong_long_long_long_long_long_long_condition_3 ||\nlong_long_long_long_long_long_long_condition_4\n\"></or>\n  <or v-if=\"\n(long_long_long_long_long_long_long_condition_1 || long_long_long_long_long_long_long_condition_2 || (short_1 || short_2) ||\nlong_long_long_long_long_long_long_condition_3) ||\nlong_long_long_long_long_long_long_condition_4\n\"></or>\n  <mixed v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 ||  ((short_1 && short_2) &&\nlong_long_long_long_long_long_long_condition_3 &&\nlong_long_long_long_long_long_long_condition_4)\n\"></mixed>\n  <mixed v-if=\"\nlong_long_long_long_long_long_long_condition_1 && long_long_long_long_long_long_long_condition_2 || short_1 && short_2 &&\nlong_long_long_long_long_long_long_condition_3 ||\nlong_long_long_long_long_long_long_condition_4\n\"></mixed>\n</root>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <root>\n    <and\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 &&\n        long_long_long_long_long_long_long_condition_2 &&\n        short_1 &&\n        short_2 &&\n        long_long_long_long_long_long_long_condition_3 &&\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></and>\n    <and\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 &&\n        long_long_long_long_long_long_long_condition_2 &&\n        short_1 &&\n        short_2 &&\n        long_long_long_long_long_long_long_condition_3 &&\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></and>\n    <or\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 ||\n        long_long_long_long_long_long_long_condition_2 ||\n        short_1 ||\n        short_2 ||\n        long_long_long_long_long_long_long_condition_3 ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></or>\n    <or\n      v-if=\"\n        long_long_long_long_long_long_long_condition_1 ||\n        long_long_long_long_long_long_long_condition_2 ||\n        short_1 ||\n        short_2 ||\n        long_long_long_long_long_long_long_condition_3 ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></or>\n    <mixed\n      v-if=\"\n        (long_long_long_long_long_long_long_condition_1 &&\n          long_long_long_long_long_long_long_condition_2) ||\n        (short_1 &&\n          short_2 &&\n          long_long_long_long_long_long_long_condition_3 &&\n          long_long_long_long_long_long_long_condition_4)\n      \"\n    ></mixed>\n    <mixed\n      v-if=\"\n        (long_long_long_long_long_long_long_condition_1 &&\n          long_long_long_long_long_long_long_condition_2) ||\n        (short_1 &&\n          short_2 &&\n          long_long_long_long_long_long_long_condition_3) ||\n        long_long_long_long_long_long_long_condition_4\n      \"\n    ></mixed>\n  </root>\n</template>");
    Ok(())
}
