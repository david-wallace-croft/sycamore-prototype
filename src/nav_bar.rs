use crate::app::InfoEnum;
use crate::constants::*;
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
      button()
        .attr("type", "button")
        .class("grid-row-2")
        .on("click", |_| {
          info_component.set(InfoEnum::Help);
        })
        .t("Help")
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
                .attr("d", SVG_HELP)
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-3")
        .on("click", |_| {
          info_component.set(InfoEnum::Accessibility);
        })
        .t("Accessibility")
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
                .attr("d", SVG_ACCESSIBILITY)
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-4")
        .on("click", |_| {
          info_component.set(InfoEnum::Contact);
        })
        .t("Contact")
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
                .attr("d", SVG_CONTACT)
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-5")
        .on("click", |_| {
          info_component.set(InfoEnum::Privacy);
        })
        .t("Privacy")
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
                .attr("d", SVG_PRIVACY)
                .attr("role", "presentation"),
            ),
        ),
    )
    .c(
      button()
        .attr("type", "button")
        .class("grid-row-6")
        .on("click", |_| navigate("/"))
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
