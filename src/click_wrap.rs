use crate::constants::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn ClickWrapComponent<G: Html>(cx: Scope) -> View<G> {
  let language_toggle = use_context::<Signal<bool>>(cx);
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
        )
        .dyn_if(
          || *language_toggle.get(),
          || span().dyn_t(|| "THIS IS A DEMONSTRATION"),
          || {
            span().dyn_dangerously_set_inner_html(|| {
              "ESTA ES UNA DEMOSTRACI&Oacute;N"
            })
          },
        ),
    )
    .c(
      p()
        .dyn_if(
          || *language_toggle.get(),
          || span().dyn_t(|| "[English] "),
          || span().dyn_dangerously_set_inner_html(|| "[Espa&ntilde;ol] "),
        )
        .t(LOREM_IPSUM),
    )
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
