use crate::icon::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ButtonProp<'a> {
  pub attr_type: &'a str,
  pub svg: &'a str,
  pub text: &'a str,
}

#[component]
pub fn ButtonComponent<'a, G: Html>(
  cx: Scope<'a>,
  prop: ButtonProp<'a>,
) -> View<G> {
  button()
    .class("btn btn-info")
    .attr("type", prop.attr_type)
    .t(prop.text)
    .c(IconComponent(
      cx,
      IconProp {
        svg: prop.svg,
      },
    ))
    .view(cx)
}
