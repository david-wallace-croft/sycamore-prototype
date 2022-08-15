use crate::nav_bar::NavBarComponent;
use crate::privacy::PrivacyComponent;
use crate::router::Router1;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn AppComponent<G: Html>(cx: Scope) -> View<G> {
  let banner_width: &Signal<u64> = create_signal(cx, 123);
  let time_remaining: &Signal<u64> = create_signal(cx, 456);
  div()
    .c(
      h1()
        .class("app-header")
        .t("Application Title [")
        .dyn_t(|| banner_width.get().to_string())
        .t("px]")
        .c(
          span()
            .class("time-remaining")
            .dyn_t(|| time_remaining.get().to_string()),
        ),
    )
    .c(
      div().class("app-content")
        .c(div().class("app-navbar").c(NavBarComponent(cx)))
        .c(div().class("app-main")
          .c(Router1(cx)))
        .c(div().class("app-info")
          .c(PrivacyComponent(cx))))
    // .c(Form1Component(cx))
    // .c(ContactComponent(cx))
    // .c(PrivacyComponent(cx))
    .view(cx)
}
