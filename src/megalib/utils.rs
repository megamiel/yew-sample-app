use web_sys::{HtmlElement as Element, HtmlInputElement as Input};
use yew::prelude::*;
use gloo::console;
use web_sys::wasm_bindgen::JsCast;

use crate::{Func, Model};

pub fn call<E: JsCast + 'static, F: 'static>(ctx: &yew::Context<Model>, callback: F) -> Callback<E>
where
    F: Fn(E) -> Func,
{
    ctx.link().callback(move |e: E| callback(e))
}

pub fn get<Element: JsCast>(event: &Event, callback: fn(Element) -> Func) -> Func {
    if let Some(target) = event.target() {
        if let Some(element) = target.dyn_into::<Element>().ok() {
            return callback(element);
        }
    }
    Func::None
}

pub fn id(element: Element) -> usize {
    element.id().parse::<usize>().expect("Invalid ID")
}
