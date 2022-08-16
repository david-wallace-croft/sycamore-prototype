use crate::constants::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[derive(Prop)]
pub struct IconProp<'a> {
  pub svg: &'a str,
}

#[component]
pub fn IconComponent<'a, G: Html>(
  cx: Scope<'a>,
  prop: IconProp<'a>,
) -> View<G> {
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
    .c(path().attr("alt", "").attr("d", prop.svg).attr("role", "presentation"))
    .view(cx)
}
