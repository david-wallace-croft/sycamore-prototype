use crate::button::*;
use crate::constants::*;
use crate::icon::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn ClickWrapComponent<G: Html>(cx: Scope) -> View<G> {
  let language_toggle = use_context::<Signal<bool>>(cx);
  div()
    .class("app-click-wrap-component warning")
    .c(
      h2()
        .c(IconComponent(
          cx,
          &IconProp {
            svg: SVG_DEMONSTRATION,
          },
        ))
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
    .c(div().class("main-buttons").c(a().attr("href", "/form1").c(
      ButtonComponent(
        cx,
        &ButtonProp {
          attr_type: "button",
          text: "Continue",
          svg: SVG_CONTINUE,
        },
      ),
    )))
    .view(cx)
}
