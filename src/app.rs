use crate::accessibility::AccessibilityComponent;
use crate::contact::ContactComponent;
use crate::help::HelpComponent;
use crate::nav_bar::NavBarComponent;
use crate::privacy::PrivacyComponent;
use crate::router::Router1;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

pub enum InfoEnum {
  Accessibility,
  Contact,
  Empty,
  Help,
  Privacy,
}

#[component]
pub fn AppComponent<G: Html>(cx: Scope) -> View<G> {
  let banner_width: &Signal<u64> = create_signal(cx, 123);
  let time_remaining: &Signal<u64> = create_signal(cx, 456);
  let info_component: &Signal<InfoEnum> = create_signal(cx, InfoEnum::Empty);
  let language_toggle: &Signal<bool> = create_signal(cx, false);
  provide_context_ref(cx, info_component);
  provide_context_ref(cx, language_toggle);
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
            InfoEnum::Accessibility => AccessibilityComponent(cx),
            InfoEnum::Contact => ContactComponent(cx),
            InfoEnum::Empty => View::empty(),
            InfoEnum::Help => HelpComponent(cx),
            InfoEnum::Privacy => PrivacyComponent(cx),
          },
        )),
    )
    .view(cx)
}
