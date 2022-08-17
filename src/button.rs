use crate::icon::*;
use sycamore::builder::{prelude::*, ElementBuilder};
use sycamore::prelude::*;

#[derive(Prop)]
pub struct ButtonProp<'a> {
  // TODO: change attr_type to an enum
  pub attr_type: &'a str,
  pub svg: &'a str,
  pub text: &'a str,
}

pub fn button_builder<'a, G>(
  prop: &ButtonProp<'a>
) -> ElementBuilder<'a, G, impl FnOnce(Scope<'a>) -> G + 'a>
where
  G: GenericNode,
{
  button().class("btn btn-info").attr("type", prop.attr_type).t(prop.text).c(
    icon_builder(&IconProp {
      svg: prop.svg,
    }),
  )
}

#[component]
pub fn ButtonComponent<'a, G: Html>(
  cx: Scope<'a>,
  prop: &ButtonProp<'a>,
) -> View<G> {
  button_builder(prop).view(cx)
}
