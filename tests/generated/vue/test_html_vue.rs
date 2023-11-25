use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_elastic_header_html_format_1_2015e17d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- https://github.com/vuejs/vue/blob/dev/examples/elastic-header/index.html -->\n<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    <meta name=\"viewport\" content=\"initial-scale=1, maximum-scale=1, user-scalable=no\">\n    <title>Vue.js elastic header example</title>\n    <!-- Delete \".min\" for console warnings in development -->\n    <script src=\"../../dist/vue.min.js\"></script>\n    <script src=\"http://dynamicsjs.com/lib/dynamics.js\"></script>\n    <link rel=\"stylesheet\" href=\"style.css\">\n    <!-- template for the component -->\n    <script type=\"text/x-template\" id=\"header-view-template\">\n      <div class=\"draggable-header-view\"\n        @mousedown=\"startDrag\" @touchstart=\"startDrag\"\n        @mousemove=\"onDrag\" @touchmove=\"onDrag\"\n        @mouseup=\"stopDrag\" @touchend=\"stopDrag\" @mouseleave=\"stopDrag\">\n        <svg class=\"bg\" width=\"320\" height=\"560\">\n          <path :d=\"headerPath\" fill=\"#3F51B5\"></path>\n        </svg>\n        <div class=\"header\">\n          <slot name=\"header\"></slot>\n        </div>\n        <div class=\"content\" :style=\"contentPosition\">\n          <slot name=\"content\"></slot>\n        </div>\n      </div>\n    </script>\n  </head>\n  <body>\n\n    <div id=\"app\" @touchmove.prevent>\n      <draggable-header-view>\n        <template slot=\"header\">\n          <h1>Elastic Draggable SVG Header</h1>\n          <p>with <a href=\"https://vuejs.org\">Vue.js</a> + <a href=\"http://dynamicsjs.com\">dynamics.js</a></p>\n        </template>\n        <template slot=\"content\">\n          <p>Note this is just an effect demo - there are of course many additional details if you want to use this in production, e.g. handling responsive sizes, reload threshold and content scrolling. Those are out of scope for this quick little hack. However, the idea is that you can hide them as internal details of a Vue.js component and expose a simple Web-Component-like interface.</p>\n        </template>\n      </draggable-header-view>\n    </div>\n\n    <script>\n    Vue.component('draggable-header-view', {\n      template: '#header-view-template',\n      data: function () {\n        return {\n          dragging: false,\n          // quadratic bezier control point\n          c: { x: 160, y: 160 },\n          // record drag start point\n          start: { x: 0, y: 0 }\n        }\n      },\n      computed: {\n        headerPath: function () {\n          return 'M0,0 L320,0 320,160' +\n            'Q' + this.c.x + ',' + this.c.y +\n            ' 0,160'\n        },\n        contentPosition: function () {\n          var dy = this.c.y - 160\n          var dampen = dy > 0 ? 2 : 4\n          return {\n            transform: 'translate3d(0,' + dy / dampen + 'px,0)'\n          }\n        }\n      },\n      methods: {\n        startDrag: function (e) {\n          e = e.changedTouches ? e.changedTouches[0] : e\n          this.dragging = true\n          this.start.x = e.pageX\n          this.start.y = e.pageY\n        },\n        onDrag: function (e) {\n          e = e.changedTouches ? e.changedTouches[0] : e\n          if (this.dragging) {\n            this.c.x = 160 + (e.pageX - this.start.x)\n            // dampen vertical drag by a factor\n            var dy = e.pageY - this.start.y\n            var dampen = dy > 0 ? 1.5 : 4\n            this.c.y = 160 + dy / dampen\n          }\n        },\n        stopDrag: function () {\n          if (this.dragging) {\n            this.dragging = false\n            dynamics.animate(this.c, {\n              x: 160,\n              y: 160\n            }, {\n              type: dynamics.spring,\n              duration: 700,\n              friction: 280\n            })\n          }\n        }\n      }\n    })\n\n    new Vue({ el: '#app' })\n    </script>\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- https://github.com/vuejs/vue/blob/dev/examples/elastic-header/index.html -->\n<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <meta\n      name=\"viewport\"\n      content=\"initial-scale=1, maximum-scale=1, user-scalable=no\"\n    />\n    <title>Vue.js elastic header example</title>\n    <!-- Delete \".min\" for console warnings in development -->\n    <script src=\"../../dist/vue.min.js\"></script>\n    <script src=\"http://dynamicsjs.com/lib/dynamics.js\"></script>\n    <link rel=\"stylesheet\" href=\"style.css\" />\n    <!-- template for the component -->\n    <script type=\"text/x-template\" id=\"header-view-template\">\n      <div class=\"draggable-header-view\"\n        @mousedown=\"startDrag\" @touchstart=\"startDrag\"\n        @mousemove=\"onDrag\" @touchmove=\"onDrag\"\n        @mouseup=\"stopDrag\" @touchend=\"stopDrag\" @mouseleave=\"stopDrag\">\n        <svg class=\"bg\" width=\"320\" height=\"560\">\n          <path :d=\"headerPath\" fill=\"#3F51B5\"></path>\n        </svg>\n        <div class=\"header\">\n          <slot name=\"header\"></slot>\n        </div>\n        <div class=\"content\" :style=\"contentPosition\">\n          <slot name=\"content\"></slot>\n        </div>\n      </div>\n    </script>\n  </head>\n  <body>\n    <div id=\"app\" @touchmove.prevent>\n      <draggable-header-view>\n        <template slot=\"header\">\n          <h1>Elastic Draggable SVG Header</h1>\n          <p>\n            with <a href=\"https://vuejs.org\">Vue.js</a> +\n            <a href=\"http://dynamicsjs.com\">dynamics.js</a>\n          </p>\n        </template>\n        <template slot=\"content\">\n          <p>\n            Note this is just an effect demo - there are of course many\n            additional details if you want to use this in production, e.g.\n            handling responsive sizes, reload threshold and content scrolling.\n            Those are out of scope for this quick little hack. However, the idea\n            is that you can hide them as internal details of a Vue.js component\n            and expose a simple Web-Component-like interface.\n          </p>\n        </template>\n      </draggable-header-view>\n    </div>\n\n    <script>\n      Vue.component(\"draggable-header-view\", {\n        template: \"#header-view-template\",\n        data: function () {\n          return {\n            dragging: false,\n            // quadratic bezier control point\n            c: { x: 160, y: 160 },\n            // record drag start point\n            start: { x: 0, y: 0 },\n          };\n        },\n        computed: {\n          headerPath: function () {\n            return (\n              \"M0,0 L320,0 320,160\" + \"Q\" + this.c.x + \",\" + this.c.y + \" 0,160\"\n            );\n          },\n          contentPosition: function () {\n            var dy = this.c.y - 160;\n            var dampen = dy > 0 ? 2 : 4;\n            return {\n              transform: \"translate3d(0,\" + dy / dampen + \"px,0)\",\n            };\n          },\n        },\n        methods: {\n          startDrag: function (e) {\n            e = e.changedTouches ? e.changedTouches[0] : e;\n            this.dragging = true;\n            this.start.x = e.pageX;\n            this.start.y = e.pageY;\n          },\n          onDrag: function (e) {\n            e = e.changedTouches ? e.changedTouches[0] : e;\n            if (this.dragging) {\n              this.c.x = 160 + (e.pageX - this.start.x);\n              // dampen vertical drag by a factor\n              var dy = e.pageY - this.start.y;\n              var dampen = dy > 0 ? 1.5 : 4;\n              this.c.y = 160 + dy / dampen;\n            }\n          },\n          stopDrag: function () {\n            if (this.dragging) {\n              this.dragging = false;\n              dynamics.animate(\n                this.c,\n                {\n                  x: 160,\n                  y: 160,\n                },\n                {\n                  type: dynamics.spring,\n                  duration: 700,\n                  friction: 280,\n                },\n              );\n            }\n          },\n        },\n      });\n\n      new Vue({ el: \"#app\" });\n    </script>\n  </body>\n</html>");
}
#[test]
fn test_hello_world_html_format_1_458df905() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- Hello World example from https://github.com/vuejs/vuejs.org/blob/master/src/v2/examples/vue-20-hello-world/index.html -->\n<!DOCTYPE html>\n<html>\n<head>\n  <title>My first Vue app</title>\n  <script src=\"https://unpkg.com/vue\"></script>\n</head>\n<body>\n  <div id=\"app\">\n    {{ message }}\n  </div>\n\n  <script>\n    var app = new Vue({\n      el: '#app',\n      data: {\n        message: 'Hello Vue!'\n      }\n    })\n  </script>\n</body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- Hello World example from https://github.com/vuejs/vuejs.org/blob/master/src/v2/examples/vue-20-hello-world/index.html -->\n<!doctype html>\n<html>\n  <head>\n    <title>My first Vue app</title>\n    <script src=\"https://unpkg.com/vue\"></script>\n  </head>\n  <body>\n    <div id=\"app\">\n      {{ message }}\n    </div>\n\n    <script>\n      var app = new Vue({\n        el: \"#app\",\n        data: {\n          message: \"Hello Vue!\",\n        },\n      });\n    </script>\n  </body>\n</html>");
}
#[test]
fn test_upper_case_2_html_format_1_39092d61() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- issue #8162 -->\n\n<!DOCTYPE html><HTML>\n    <body>\n      <div v-if=\"foo ===    'foo'\">\n  \n  </div>\n      <script>\n  new Vue({el: '#app'})\n      </script>\n    </body>\n  </HTML>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- issue #8162 -->\n\n<!doctype html>\n<html>\n  <body>\n    <div v-if=\"foo === 'foo'\"></div>\n    <script>\n      new Vue({ el: \"#app\" });\n    </script>\n  </body>\n</html>");
}
#[test]
fn test_upper_case_html_tag_html_format_1_9a2d9078() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<!doctype html><HTML></HTML>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<!doctype html>\n<html></html>");
}
#[test]
fn test_upper_case_html_tag_2_html_format_1_0a6a2668() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html><HTML>\n  <body>\n    <div v-if=\"foo ===    'foo'\">\n\n</div>\n    <script>\nnew Vue({el: '#app'})\n    </script>\n  </body>\n</HTML>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <body>\n    <div v-if=\"foo === 'foo'\"></div>\n    <script>\n      new Vue({ el: \"#app\" });\n    </script>\n  </body>\n</html>");
}
#[test]
fn test_upper_case_html_tag_3_html_format_1_4a71015a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<HTML><head></head><body></body></HTML>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<html>\n  <head></head>\n  <body></body>\n</html>"
    );
}
#[test]
fn test_upper_case_html_tag_4_html_format_1_b0754683() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<hTml>\n    <body>\n      <div v-if=\"foo ===    'foo'\">\n\n  </div>\n      <script>\n  new Vue({el: '#app'})\n      </script>\n    </body>\n  </hTml>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<html>\n  <body>\n    <div v-if=\"foo === 'foo'\"></div>\n    <script>\n      new Vue({ el: \"#app\" });\n    </script>\n  </body>\n</html>");
}
#[test]
fn test_void_element_html_format_1_dc93e493() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<img>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<img />");
}
