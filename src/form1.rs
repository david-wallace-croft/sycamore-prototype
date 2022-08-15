use crate::constants::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Form1Component<G: Html>(cx: Scope) -> View<G> {
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
                .attr("required", "required")
                .dyn_attr("type", || {
                  if *show.get() {
                    Some("text")
                  } else {
                    Some("password")
                  }
                })
                .dyn_attr("placeholder", || {
                  if *show.get() {
                    Some("[characters shown]")
                  } else {
                    Some("[characters masked]")
                  }
                }),
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
