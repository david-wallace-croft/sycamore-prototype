use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
  let banner_width: &Signal<u64> = create_signal(cx, 123);
  let time_remaining: &Signal<u64> = create_signal(cx, 456);
  div()
    .c(
      h1()
        .class("app-header")
        .t("Application Title [")
        .dyn_t(|| banner_width.get().to_string())
        .t("px]"),
    )
    .c(
      span().class("time-remaining").dyn_t(|| time_remaining.get().to_string()),
    )
    .c(div().class("app-content").c(div().class("app-navbar")))
    .c(PrivacyComponent(cx))
    .view(cx)
  //   let name = create_signal(cx, String::new());
  //   div()
  //     .c(
  //       h1()
  //         .t("Hello, ")
  //         .dyn_if(
  //           || !name.get().is_empty(),
  //           || span().dyn_t(|| name.get().to_string()),
  //           || span().t("World"),
  //         )
  //         .t("!"),
  //     )
  //     .c(input().bind_value(name))
  //     .view(cx)
}

#[component]
fn PrivacyComponent<G: Html>(cx: Scope) -> View<G> {
  div()
    .c(h2().t("Privacy"))
    .c(p().t(
      "Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
      sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
      Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi \
      ut aliquip ex ea commodo consequat.",
    ))
    .view(cx)
}

fn main() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Debug).unwrap();
  sycamore::render(|cx| component(|| App(cx)));
}
