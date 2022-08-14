mod constants;

use constants::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use sycamore_router::{
  navigate, HistoryIntegration, Route, Router, RouterProps,
};

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
            .attr("viewBox", BUTTON_SVG_VIEW_BOX)
            .attr("width", "24")
            .attr("xmlns", BUTTON_SVG_XMLNS)
            .c(
              path()
                .attr("alt", "")
                .attr("d", BUTTON_SVG_PREFIX)
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
              .attr("viewBox", BUTTON_SVG_VIEW_BOX)
              .attr("width", "24")
              .attr("xmlns", BUTTON_SVG_XMLNS)
              .c(
                path()
                  .attr("alt", "")
                  .attr("d", BUTTON_SVG_PREFIX)
                  .attr("fill", "none")
                  .attr("role", "presentation"),
              )
              .c(
                path()
                  .attr("alt", "")
                  .attr(
                    "d",
                    SVG_CONTINUE,
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
  let show: &Signal<bool> = create_signal(cx, false);
  div()
    .c(h2().t("Form 1 Title"))
    .c(h3().t("All fields are required"))
    .c(
      form()
        .c(
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
        )
        .c(
          button()
            .class("btn btn-info")
            .attr("type", "button")
            .dyn_if(
              || *show.get(),
              || span().dyn_t(|| "Mask"),
              || span().dyn_t(|| "Show"),
            )
            .on("click", |_| show.set(!*show.get()))
            .dyn_if(
              || *show.get(),
              || {
                span().dyn_c(|| {
                  svg()
                    .attr("alt", "")
                    .attr("fill", "#000")
                    .attr("focusable", "false")
                    .attr("height", "24")
                    .attr("role", "presentation")
                    .attr("viewBox", BUTTON_SVG_VIEW_BOX)
                    .attr("width", "24")
                    .attr("xmlns", BUTTON_SVG_XMLNS)
                    .c(
                      path()
                        .attr("alt", "")
                        .attr("d", BUTTON_SVG_PREFIX)
                        .attr("fill", "none")
                        .attr("role", "presentation"),
                    )
                    .c(
                      path()
                        .attr("alt", "")
                        .attr("d", SVG_MASK)
                        .attr("role", "presentation"),
                    )
                })
              },
              || {
                span().dyn_c(|| {
                  svg()
                    .attr("alt", "")
                    .attr("fill", "#000")
                    .attr("focusable", "false")
                    .attr("height", "24")
                    .attr("role", "presentation")
                    .attr("viewBox", BUTTON_SVG_VIEW_BOX)
                    .attr("width", "24")
                    .attr("xmlns", BUTTON_SVG_XMLNS)
                    .c(
                      path()
                        .attr("alt", "")
                        .attr("d", BUTTON_SVG_PREFIX)
                        .attr("fill", "none")
                        .attr("role", "presentation"),
                    )
                    .c(
                      path()
                        .attr("alt", "")
                        .attr("d", SVG_SHOW)
                        .attr("role", "presentation"),
                    )
                })
              },
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
            .attr("viewBox", BUTTON_SVG_VIEW_BOX)
            .attr("width", "24")
            .attr("xmlns", BUTTON_SVG_XMLNS)
            .c(
              path()
                .attr("alt", "")
                .attr("d", BUTTON_SVG_PREFIX)
                .attr("fill", "none")
                .attr("role", "presentation"),
            )
            .c(
              path()
                .attr("alt", "")
                .attr("d", SVG_LANGUAGE)
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(
      button().class("grid-row-2").attr("type", "button").t("Help").c(
        svg()
          .attr("alt", "")
          .attr("fill", "#000")
          .attr("focusable", "false")
          .attr("height", "24")
          .attr("role", "presentation")
          .attr("viewBox", BUTTON_SVG_VIEW_BOX)
          .attr("width", "24")
          .attr("xmlns", BUTTON_SVG_XMLNS)
          .c(
            path()
              .attr("alt", "")
              .attr("d", BUTTON_SVG_PREFIX)
              .attr("fill", "none")
              .attr("role", "presentation"),
          )
          .c(
            path()
              .attr("alt", "")
              .attr("d", SVG_HELP)
              .attr("role", "presentation"),
          ),
      ),
    )
    .c(
      button().class("grid-row-3").attr("type", "button").t("Accessibility").c(
        svg()
          .attr("alt", "")
          .attr("fill", "#000")
          .attr("focusable", "false")
          .attr("height", "24")
          .attr("role", "presentation")
          .attr("viewBox", BUTTON_SVG_VIEW_BOX)
          .attr("width", "24")
          .attr("xmlns", BUTTON_SVG_XMLNS)
          .c(
            path()
              .attr("alt", "")
              .attr("d", BUTTON_SVG_PREFIX)
              .attr("fill", "none")
              .attr("role", "presentation"),
          )
          .c(
            path()
              .attr("alt", "")
              .attr("d", SVG_ACCESSIBILITY)
              .attr("role", "presentation"),
          ),
      ),
    )
    .c(
      button().class("grid-row-4").attr("type", "button").t("Contact").c(
        svg()
          .attr("alt", "")
          .attr("fill", "#000")
          .attr("focusable", "false")
          .attr("height", "24")
          .attr("role", "presentation")
          .attr("viewBox", BUTTON_SVG_VIEW_BOX)
          .attr("width", "24")
          .attr("xmlns", BUTTON_SVG_XMLNS)
          .c(
            path()
              .attr("alt", "")
              .attr("d", BUTTON_SVG_PREFIX)
              .attr("fill", "none")
              .attr("role", "presentation"),
          )
          .c(
            path()
              .attr("alt", "")
              .attr("d", SVG_CONTACT)
              .attr("role", "presentation"),
          ),
      ),
    )
    .c(
      button().class("grid-row-5").attr("type", "button").t("Privacy").c(
        svg()
          .attr("alt", "")
          .attr("fill", "#000")
          .attr("focusable", "false")
          .attr("height", "24")
          .attr("role", "presentation")
          .attr("viewBox", BUTTON_SVG_VIEW_BOX)
          .attr("width", "24")
          .attr("xmlns", BUTTON_SVG_XMLNS)
          .c(
            path()
              .attr("alt", "")
              .attr("d", BUTTON_SVG_PREFIX)
              .attr("fill", "none")
              .attr("role", "presentation"),
          )
          .c(
            path()
              .attr("alt", "")
              .attr("d", SVG_PRIVACY)
              .attr("role", "presentation"),
          ),
      ),
    )
    .c(
      button()
        .on("click", |_| navigate("/"))
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
            .attr("viewBox", BUTTON_SVG_VIEW_BOX)
            .attr("width", "24")
            .attr("xmlns", BUTTON_SVG_XMLNS)
            .c(
              path()
                .attr("alt", "")
                .attr("d", BUTTON_SVG_PREFIX)
                .attr("fill", "none")
                .attr("role", "presentation"),
            )
            .c(
              path()
                .attr("alt", "")
                .attr("d", SVG_EXIT)
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
