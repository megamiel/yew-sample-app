use web_sys::{HtmlElement, HtmlHeadElement, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use gloo::console;
use web_sys::wasm_bindgen::JsCast;

use crate::{Msg, Model};

pub type Element=HtmlElement;
pub type Input=HtmlInputElement;
pub type TextArea=HtmlTextAreaElement;
pub type Head=HtmlHeadElement;

pub fn call<E: JsCast + 'static, F: 'static>(ctx: &yew::Context<Model>, callback: F) -> Callback<E>
where
    F: Fn(E) -> Msg,
{
    ctx.link().callback(move |e: E| callback(e))
}

pub fn get<Element: JsCast>(event: &Event, callback: fn(Element) -> Msg) -> Msg {
    if let Some(target) = event.target() {
        if let Some(element) = target.dyn_into::<Element>().ok() {
            return callback(element);
        }
    }
    Msg::None
}

pub fn id(element: HtmlElement) -> usize {
    element.id().parse::<usize>().expect("Invalid ID")
}
