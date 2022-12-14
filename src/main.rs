mod accessibility;
mod app;
mod button;
mod click_wrap;
mod constants;
mod contact;
mod form1;
mod form2;
mod help;
mod icon;
mod nav_bar;
mod privacy;
mod router;
mod toggle_button;

use app::AppComponent;
use sycamore::builder::prelude::*;

fn main() {
  console_error_panic_hook::set_once();
  console_log::init_with_level(log::Level::Debug).unwrap();
  sycamore::render(|cx| component(|| AppComponent(cx)));
}
