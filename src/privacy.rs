use crate::constants::*;
use sycamore::builder::prelude::*;
use sycamore::prelude::*;

#[component]
pub fn PrivacyComponent<G: Html>(cx: Scope) -> View<G> {
  div().c(h2().t("Privacy")).c(p().t(LOREM_IPSUM)).view(cx)
}
