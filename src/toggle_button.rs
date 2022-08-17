use crate::icon::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ToggleButtonProp<'a> {
  pub svg_false: &'a str,
  pub svg_true: &'a str,
  pub text_false: &'a str,
  pub text_true: &'a str,
}

#[component]
pub fn ToggleButtonComponent<'a, G: Html>(
  cx: Scope<'a>,
  prop: ToggleButtonProp<'a>,
) -> View<G> {
  let show = use_context::<Signal<bool>>(cx);
  button()
    .class("btn btn-info")
    .attr("type", "button")
    .dyn_t(move || {
      if *show.get() {
        prop.text_true
      } else {
        prop.text_false
      }
    })
    .on("click", |_| show.set(!*show.get()))
    .dyn_c(move || {
      if *show.get() {
        IconComponent(
          cx,
          &IconProp {
            svg: prop.svg_true,
          },
        )
      } else {
        IconComponent(
          cx,
          &IconProp {
            svg: prop.svg_false,
          },
        )
      }
    })
    .view(cx)
}
