use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, \
  consectetur adipiscing elit, \
  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
  Ut enim ad minim veniam, \
  quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo \
  consequat.";

#[derive(Route)]
enum AppRoutes {
  #[to("/")]
  Index,
  #[to("/form1")]
  Form1,
  #[not_found]
  NotFound,
}

fn main() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Debug).unwrap();
  sycamore::render(|cx| component(|| AppComponent(cx)));
}

#[component]
fn Router1<G: Html>(cx: Scope) -> View<G> {
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
              AppRoutes::NotFound => view! { cx, "404 Not Found" },
            })
          }
        }
      }
    }
  }
}

#[component]
fn AppComponent<G: Html>(cx: Scope) -> View<G> {
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
        a().attr("href", "/form1").c(
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
fn Form1Component<G: Html>(cx: Scope) -> View<G> {
  div()
    .c(h2().t("Form 1 Title"))
    .c(h3().t("All fields are required"))
    .c(
      form().c(
        div()
          .class("form-group")
          .c(
            label()
              .class("control-label")
              .attr("for", "idn")
              .t("Identity Number"),
          )
          .c(
            input()
              .class("form-control")
              .id("idn")
              .attr("maxlength", "11")
              .attr("name", "idn")
              .attr("pattern", "^(\\d{9}|(\\d{3}-\\d{2}-\\d{4}))$")
              .attr("placeholder", "[characters masked]")
              .attr("required", "required")
              .attr("type", "password"),
          ),
      ),
    )
    .view(cx)
}

#[component]
fn NavBarComponent<G: Html>(cx: Scope) -> View<G> {
  div()
    .class("btn-group-vertical")
    .attr("role", "group")
    .c(
      button()
        .class("grid-row-1")
        .attr("type", "button")
        .dangerously_set_inner_html("Espa&ntilde;ol")
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
                .attr("d",
    "M11.99 2C6.47 2 2 6.48 2 12s4.47 10 9.99 10C17.52 22 22 17.52 22
    12S17.52 2 11.99 2zm6.93 6h-2.95c-.32-1.25-.78-2.45-1.38-3.56 1.84.63
    3.37 1.91 4.33 3.56zM12 4.04c.83 1.2 1.48 2.53 1.91 3.96h-3.82c.43-1.43
    1.08-2.76 1.91-3.96zM4.26 14C4.1 13.36 4 12.69 4
    12s.1-1.36.26-2h3.38c-.08.66-.14 1.32-.14 2 0 .68.06 1.34.14 2H4.26zm.82
    2h2.95c.32 1.25.78 2.45 1.38
    3.56-1.84-.63-3.37-1.9-4.33-3.56zm2.95-8H5.08c.96-1.66 2.49-2.93
    4.33-3.56C8.81 5.55 8.35 6.75 8.03 8zM12
    19.96c-.83-1.2-1.48-2.53-1.91-3.96h3.82c-.43 1.43-1.08 2.76-1.91
    3.96zM14.34 14H9.66c-.09-.66-.16-1.32-.16-2
    0-.68.07-1.35.16-2h4.68c.09.65.16 1.32.16 2 0 .68-.07 1.34-.16 2zm.25
    5.56c.6-1.11 1.06-2.31 1.38-3.56h2.95c-.96 1.65-2.49 2.93-4.33
    3.56zM16.36 14c.08-.66.14-1.32.14-2 0-.68-.06-1.34-.14-2h3.38c.16.64.26
    1.31.26 2s-.1 1.36-.26 2h-3.38z")
                .attr("role", "presentation"),
            ),
        ),
    )
    .view(cx)
}

#[component]
fn PrivacyComponent<G: Html>(cx: Scope) -> View<G> {
  div().c(h2().t("Privacy")).c(p().t(LOREM_IPSUM)).view(cx)
}
