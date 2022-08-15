use crate::contact::ContactComponent;
use crate::nav_bar::NavBarComponent;
use crate::privacy::PrivacyComponent;
use crate::router::Router1;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

pub enum InfoEnum {
  Contact,
  Empty,
  Privacy,
}

#[component]
pub fn AppComponent<G: Html>(cx: Scope) -> View<G> {
  let banner_width: &Signal<u64> = create_signal(cx, 123);
  let time_remaining: &Signal<u64> = create_signal(cx, 456);
  let info_component: &Signal<InfoEnum> = create_signal(cx, InfoEnum::Empty);
  provide_context_ref(cx, info_component);
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
      div()
        .class("app-content")
        .c(div().class("app-navbar").c(NavBarComponent(cx)))
        .c(div().class("app-main").c(Router1(cx)))
        .c(div().class("app-info").dyn_c(
          move || match *info_component.get() {
            InfoEnum::Contact => ContactComponent(cx),
            InfoEnum::Empty => View::empty(),
            InfoEnum::Privacy => PrivacyComponent(cx),
          },
        )),
    )
    .view(cx)
}
