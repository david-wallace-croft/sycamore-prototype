use crate::app::InfoEnum;
use crate::constants::*;
use crate::icon::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use sycamore_router::navigate;

#[component]
pub fn NavBarComponent<G: Html>(cx: Scope) -> View<G> {
  let info_component = use_context::<Signal<InfoEnum>>(cx);
  let language_toggle = use_context::<Signal<bool>>(cx);
  div()
    .class("btn-group-vertical")
    .attr("role", "group")
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-1")
        .dyn_if(
          || *language_toggle.get(),
          || span().dyn_dangerously_set_inner_html(|| "Espa&ntilde;ol"),
          || span().dyn_t(|| "English"),
        )
        .on("click", |_| {
          language_toggle.set(!*language_toggle.get());
        })
        .c(IconComponent(
          cx,
          &IconProp {
            svg: SVG_LANGUAGE,
          },
        )),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-2")
        .on("click", |_| {
          info_component.set(InfoEnum::Help);
        })
        .t("Help")
        .c(IconComponent(
          cx,
          &IconProp {
            svg: SVG_HELP,
          },
        )),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-3")
        .on("click", |_| {
          info_component.set(InfoEnum::Accessibility);
        })
        .t("Accessibility")
        .c(IconComponent(
          cx,
          &IconProp {
            svg: SVG_ACCESSIBILITY,
          },
        )),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-4")
        .on("click", |_| {
          info_component.set(InfoEnum::Contact);
        })
        .t("Contact")
        .c(IconComponent(
          cx,
          &IconProp {
            svg: SVG_CONTACT,
          },
        )),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-5")
        .on("click", |_| {
          info_component.set(InfoEnum::Privacy);
        })
        .t("Privacy")
        .c(IconComponent(
          cx,
          &IconProp {
            svg: SVG_PRIVACY,
          },
        )),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-6")
        .on("click", |_| navigate("/"))
        .t("Exit")
        .c(IconComponent(
          cx,
          &IconProp {
            svg: SVG_EXIT,
          },
        )),
    )
    .view(cx)
}
