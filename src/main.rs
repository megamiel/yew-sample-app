mod megalib;
use std::ops::Add;

use gloo::console;
use megalib::utils::*;
use yew::prelude::*;
use yew::{html, Component, Html};
use Msg::*;

pub struct Model {
    todo: String,
    todo_list: Vec<String>,
}

pub enum Msg {
    InputTodo(String),
    AddTodo,
    ComplateTodo(usize),
    Callback(fn() -> ()),
    None,
}

impl Msg {
    fn func(self, model: &mut Model) -> bool {
        match self {
            InputTodo(todo) => {
                model.todo = todo.clone();
                true
            }
            AddTodo => {
                model.todo_list.push(model.todo.clone());
                model.todo = String::new();
                true
            }
            ComplateTodo(index) => {
                model.todo_list.remove(index);
                true
            }
            Callback(callback) => {
                callback();
                false
            }
            None => false,
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            todo: String::from(""),
            todo_list: vec![],
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, func: Self::Message) -> bool {
        func.func(self)
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <>
                <div>
                    <div>{"ToDoリスト"}</div>
                    <input
                        value={self.todo.clone()}
                        placeholder="ToDoを入力"
                        oninput={call(ctx,|e:InputEvent|{
                            // イベントからHtmlInputElement(as Input済み)を取得し、InputTodoを実行している
                            get::<Input>(&e,|element|InputTodo(element.value()))
                        })}
                    />
                    <button
                        onclick={call(ctx,|e:MouseEvent|{
                            // element.inner_text()で要素内の出力文字列を取得
                            get::<Element>(&e,|element|{InputTodo(element.inner_text())})
                        })}
                    >{"おせ！"}</button>
                    <button onclick={call(ctx,|_|Msg::AddTodo)}>{"追加"}</button>
                    <div>
                        <ul>
                            // iterの前にforをつけると展開できる
                            {for self.todo_list.iter().enumerate().map(|(i,todo)|{
                                html!{
                                    // moveによって、所有権を無名関数内の変数に譲渡する
                                    <li>{todo}<button id={i.to_string()} onclick={call(ctx,|e:MouseEvent|{
                                        get::<Element>(&e,|element|{
                                            ComplateTodo(id(element))
                                        })
                                    })}>{"完了"}</button></li>
                                }
                            })}
                        </ul>
                    </div>
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
