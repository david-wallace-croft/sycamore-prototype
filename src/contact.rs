use crate::constants::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn ContactComponent<G: Html>(cx: Scope) -> View<G> {
  div()
    .c(h2().t("Contact"))
    .c(h3().t("E-mail"))
    .c(p().t(LOREM_IPSUM))
    .c(h3().t("Mail"))
    .c(p().t(LOREM_IPSUM))
    .c(h3().t("Phone"))
    .c(p().t(LOREM_IPSUM))
    .view(cx)
}
