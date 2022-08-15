use crate::constants::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn HelpComponent<G: Html>(cx: Scope) -> View<G> {
  div().c(h2().t("Help")).c(p().t(LOREM_IPSUM)).view(cx)
}
