use sycamore::builder::prelude::*;
use sycamore::prelude::*;

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, \
  consectetur adipiscing elit, \
  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
  Ut enim ad minim veniam, \
  quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo \
  consequat.";

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
        .t("px]")
        .c(
          span()
            .class("time-remaining")
            .dyn_t(|| time_remaining.get().to_string()),
        ),
    )
    .c(div().class("app-content").c(div().class("app-navbar")))
    .c(ClickWrapComponent(cx))
    .c(ContactComponent(cx))
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
fn ClickWrapComponent<G: Html>(cx: Scope) -> View<G> {
  div()
    .class("app-click-wrap-component warning")
    .c(
      h2()
        .c(
          svg()
            .attr("alt", "")
            .attr("fill", "#000000")
            .attr("focusable", "false")
            .attr("height", "24")
            .attr("role", "presentation")
            .attr("viewBox", "0 0 24 24")
            .attr("width", "24")
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .c(
              path()
                .attr("alt", "")
                .attr("d", "M0 0h24v24H0z")
                .attr("fill", "none")
                .attr("role", "presentation"),
            )
            .c(
              path()
                .attr("alt", "")
                .attr("d", "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z")
                .attr("role", "presentation"),
            ),
          // <ng-template
          //   appLang="es">
          // ESTA ES UNA DEMOSTRACI&Oacute;N
          // </ng-template>
        )
        .t("THIS IS A DEMONSTRATION"),
    )
    .c(p().t("[ENGLISH] ").t(LOREM_IPSUM))
    .c(
      div().class("main-buttons").c(
        button()
          .class("btn btn-primary")
          // (click)="accept()"
          // routerLink="/form1"
          .attr("type", "button")
          .t("Continue")
          .c(
            svg()
              .attr("alt", "")
              .attr("fill", "#000")
              .attr("focusable", "false")
              .attr("height", "24")
              .attr("role", "presentation")
              .attr("viewBox", "0 0 24 24")
              .attr("width", "24")
              .attr("xmlns", "http://www.w3.org/2000/svg")
              .c(
                path()
                  .attr("alt", "")
                  .attr("d", "M0 0h24v24H0z")
                  .attr("fill", "none")
                  .attr("role", "presentation"),
              )
              .c(
                path()
                  .attr("alt", "")
                  .attr(
                    "d",
                    "M9 16.2L4.8 12l-1.4 1.4L9 19 21 7l-1.4-1.4L9 16.2z",
                  )
                  .attr("role", "presentation"),
              ),
          ),
      ),
    )
    .view(cx)
}

#[component]
fn ContactComponent<G: Html>(cx: Scope) -> View<G> {
  div()
    .c(h2().t("Contact"))
    .c(h3().t("E-mail"))
    .c(p().t(LOREM_IPSUM))
    .c(h3().t("Mail"))
    .c(p().t(LOREM_IPSUM))
    .c(h3().t("Phone"))
    .c(p().t(LOREM_IPSUM))
    .view(cx)
}

#[component]
fn PrivacyComponent<G: Html>(cx: Scope) -> View<G> {
  div().c(h2().t("Privacy")).c(p().t(LOREM_IPSUM)).view(cx)
}

fn main() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Debug).unwrap();
  sycamore::render(|cx| component(|| App(cx)));
}
