use crate::constants::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use sycamore_router::navigate;

#[component]
pub fn NavBarComponent<G: Html>(cx: Scope) -> View<G> {
  div()
    .class("btn-group-vertical")
    .attr("role", "group")
    .c(
      button()
        .class("grid-row-1")
        .attr("type", "button")
        .dangerously_set_inner_html("Espa&ntilde;ol")
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
      button().class("grid-row-2").attr("type", "button").t("Help").c(
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
      button().class("grid-row-3").attr("type", "button").t("Accessibility").c(
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
      button().class("grid-row-4").attr("type", "button").t("Contact").c(
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
      button().class("grid-row-5").attr("type", "button").t("Privacy").c(
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
        .on("click", |_| navigate("/"))
        .class("grid-row-6")
        .attr("type", "button")
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
