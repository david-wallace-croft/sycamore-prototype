use crate::button::*;
use crate::constants::*;
use crate::icon::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn Form1Component<G: Html>(cx: Scope) -> View<G> {
  let show: &Signal<bool> = create_signal(cx, false);
  div()
    .class("app-form1")
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
        .c(ButtonComponent(
          cx,
          ButtonProp {
            attr_type: "reset",
            svg: SVG_CLEAR,
            text: "Clear",
          },
        ))
        .c(
          button()
            .class("btn btn-info")
            .attr("type", "button")
            .dyn_t(|| {
              if *show.get() {
                "Mask"
              } else {
                "Show"
              }
            })
            .on("click", |_| show.set(!*show.get()))
            .dyn_c(move || {
              if *show.get() {
                IconComponent(
                  cx,
                  IconProp {
                    svg: SVG_MASK,
                  },
                )
              } else {
                IconComponent(
                  cx,
                  IconProp {
                    svg: SVG_SHOW,
                  },
                )
              }
            }),
        ),
    )
    .view(cx)
}
