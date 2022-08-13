use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, \
  consectetur adipiscing elit, \
  sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
  Ut enim ad minim veniam, \
  quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo \
  consequat.";
const SVG_FINGERPRINT: &str = "\
  M17.81 4.47c-.08 0-.16-.02-.23-.06C15.66 3.42 14 3 12.01 3c-1.98 0-3.86.47-5.\
  57 1.41-.24.13-.54.04-.68-.2-.13-.24-.04-.55.2-.68C7.82 2.52 9.86 2 12.01 2c2\
  .13 0 3.99.47 6.03 1.52.25.13.34.43.21.67-.09.18-.26.28-.44.28zM3.5 9.72c-.1 \
  0-.2-.03-.29-.09-.23-.16-.28-.47-.12-.7.99-1.4 2.25-2.5 3.75-3.27C9.98 4.04 1\
  4 4.03 17.15 5.65c1.5.77 2.76 1.86 3.75 3.25.16.22.11.54-.12.7-.23.16-.54.11-\
  .7-.12-.9-1.26-2.04-2.25-3.39-2.94-2.87-1.47-6.54-1.47-9.4.01-1.36.7-2.5 1.7-\
  3.4 2.96-.08.14-.23.21-.39.21zm6.25 12.07c-.13 0-.26-.05-.35-.15-.87-.87-1.34\
  -1.43-2.01-2.64-.69-1.23-1.05-2.73-1.05-4.34 0-2.97 2.54-5.39 5.66-5.39s5.66 \
  2.42 5.66 5.39c0 .28-.22.5-.5.5s-.5-.22-.5-.5c0-2.42-2.09-4.39-4.66-4.39-2.5\
  7 0-4.66 1.97-4.66 4.39 0 1.44.32 2.77.93 3.85.64 1.15 1.08 1.64 1.85 2.42.19\
  .2.19.51 0 .71-.11.1-.24.15-.37.15zm7.17-1.85c-1.19 0-2.24-.3-3.1-.89-1.49-1.\
  01-2.38-2.65-2.38-4.39 0-.28.22-.5.5-.5s.5.22.5.5c0 1.41.72 2.74 1.94 3.56.71\
  .48 1.54.71 2.54.71.24 0 .64-.03 1.04-.1.27-.05.53.13.58.41.05.27-.13.53-.41.\
  58-.57.11-1.07.12-1.21.12zM14.91 22c-.04 0-.09-.01-.13-.02-1.59-.44-2.63-1.03\
  -3.72-2.1-1.4-1.39-2.17-3.24-2.17-5.22 0-1.62 1.38-2.94 3.08-2.94 1.7 0 3.08 \
  1.32 3.08 2.94 0 1.07.93 1.94 2.08 1.94s2.08-.87 2.08-1.94c0-3.77-3.25-6.83-7\
  .25-6.83-2.84 0-5.44 1.58-6.61 4.03-.39.81-.59 1.76-.59 2.8 0 .78.07 2.01.67 \
  3.61.1.26-.03.55-.29.64-.26.1-.55-.04-.64-.29-.49-1.31-.73-2.61-.73-3.96 0-1.\
  2.23-2.29.68-3.24 1.33-2.79 4.28-4.6 7.51-4.6 4.55 0 8.25 3.51 8.25 7.83 0 1.\
  62-1.38 2.94-3.08 2.94s-3.08-1.32-3.08-2.94c0-1.07-.93-1.94-2.08-1.94s-2.08.8\
  7-2.08 1.94c0 1.71.66 3.31 1.87 4.51.95.94 1.86 1.46 3.27 1.85.27.07.42.35.35\
  .61-.05.23-.26.38-.47.38z";

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
  #[to("/clickwrap")]
  ClickWrap,
  #[to("/form1")]
  Form1,
  //   #[to("/form1/<_..>")]
  //   Form1Nested(Form1Nested),
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
              AppRoutes::ClickWrap => ClickWrapComponent(cx),
              AppRoutes::Form1 => Form1Component(cx),
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
    .c(
      button()
        .class("grid-row-2")
        .attr("type", "button")
        .t("Help")
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
    "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12
    2zm1 17h-2v-2h2v2zm2.07-7.75l-.9.92C13.45 12.9 13 13.5 13
    15h-2v-.5c0-1.1.45-2.1 1.17-2.83l1.24-1.26c.37-.36.59-.86.59-1.41
    0-1.1-.9-2-2-2s-2 .9-2 2H8c0-2.21 1.79-4 4-4s4 1.79 4 4c0 .88-.36
    1.68-.93 2.25z")
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(
      button()
        .class("grid-row-3")
        .attr("type", "button")
        .t("Accessibility")
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
    "M12 2c1.1 0 2 .9 2 2s-.9 2-2 2-2-.9-2-2 .9-2 2-2zm9
    7h-6v13h-2v-6h-2v6H9V9H3V7h18v2z")
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(
      button()
        .class("grid-row-4")
        .attr("type", "button")
        .t("Contact")
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
    "M21 8V7l-3 2-3-2v1l3 2 3-2zm1-5H2C.9 3 0 3.9 0 5v14c0 1.1.9
    2 2 2h20c1.1 0 1.99-.9 1.99-2L24 5c0-1.1-.9-2-2-2zM8 6c1.66
    0 3 1.34 3 3s-1.34 3-3 3-3-1.34-3-3 1.34-3 3-3zm6 12H2v-1c0-2
    4-3.1 6-3.1s6 1.1 6 3.1v1zm8-6h-8V6h8v6z")
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(
      button()
        .class("grid-row-5")
        .attr("type", "button")
        .t("Privacy")
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
                .attr("d", SVG_FINGERPRINT)
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(a().attr("href", "/")
      .c(
        button()
          .class("grid-row-6")
          .attr("type", "button")
          .t("Exit")
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
                  .attr("d", "M5 17h14v2H5zm7-12L5.33 15h13.34z")
                  .attr("role", "presentation"),
              ),
          ),
      )
    ).view(cx)
}

#[component]
fn PrivacyComponent<G: Html>(cx: Scope) -> View<G> {
  div().c(h2().t("Privacy")).c(p().t(LOREM_IPSUM)).view(cx)
}
