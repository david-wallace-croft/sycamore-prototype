use crate::click_wrap::ClickWrapComponent;
use crate::form1::Form1Component;
use crate::form2::Form2Component;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

// #[derive(Route)]
// enum Form1Nested {
//   #[to("/contact")]
//   Contact,
//   #[to("/privacy")]
//   Privacy,
//   #[not_found]
//   NotFound,
// }

#[derive(Route)]
enum AppRoutes {
  #[to("/")]
  Index,
  #[to("/form1")]
  Form1,
  #[to("/form2")]
  Form2,
  //   #[to("/form1/<_..>")]
  //   Form1Nested(Form1Nested),
  #[not_found]
  NotFound,
}

#[component]
pub fn Router1<G: Html>(cx: Scope) -> View<G> {
  view! {
    cx,
    Router {
      integration: HistoryIntegration::new(),
      view: |cx, route: &ReadSignal<AppRoutes>| {
        view! {
          cx,
          div(class="app") {
            (match route.get().as_ref() {
              AppRoutes::Index => ClickWrapComponent(cx),
              AppRoutes::Form1 => Form1Component(cx),
              AppRoutes::Form2 => Form2Component(cx),
            //   AppRoutes::Form1Nested(nested) => match nested {
            //     Form1Nested::Contact => ContactComponent(cx),
            //     Form1Nested::Privacy => PrivacyComponent(cx),
            //     Form1Nested::NotFound => view! { cx, "404 Not Found" },
            //   },
              AppRoutes::NotFound => view! { cx, "404 Not Found" },
            })
          }
        }
      }
    }
  }
}
