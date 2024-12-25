mod megalib;
use megalib::utils::*;
use gloo::console;
use web_sys::{HtmlElement as Element, HtmlInputElement as Input};
use yew::prelude::*;
use yew::{html, Component, Html};

pub struct Model {
    todo: String,
    todo_list: Vec<String>,
}

pub enum Func {
    InputTodo(String),
    AddTodo,
    ComplateTodo(usize),
    Callback(fn() -> ()),
    None,
}

impl Component for Model {
    type Message = Func;
    type Properties = ();

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            todo: String::from(""),
            todo_list: vec![],
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, func: Self::Message) -> bool {
        match func {
            Func::InputTodo(todo) => {
                self.todo = todo;
                true
            }
            Func::AddTodo => {
                self.todo_list.push(self.todo.clone());
                self.todo = String::new();
                true
            }
            Func::ComplateTodo(index) => {
                self.todo_list.remove(index);
                true
            }
            Func::Callback(callback) => {
                callback();
                false
            }
            Func::None => false,
        }
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
                            get::<Input>(&e,|element|Func::InputTodo(element.value()))
                        })}
                    />
                    <button
                        onclick={call(ctx,|e:MouseEvent|{
                            get::<Element>(&e,|element|{Func::InputTodo(element.text_content().unwrap())})
                        })}
                    >{"おせ！"}</button>
                    <button onclick={call(ctx,|_|Func::AddTodo)}>{"追加"}</button>
                    <div>
                        <ul>
                            // iterの前にforをつけると展開できる
                            {for self.todo_list.iter().enumerate().map(|(i,todo)|{
                                html!{
                                    // moveによって、所有権を無名関数内の変数に譲渡する
                                    <li>{todo}<button id={i.to_string()} onclick={call(ctx,|e:MouseEvent|{
                                        get::<Element>(&e,|element|{
                                            Func::ComplateTodo(id(element))
                                        })
                                        // Func::ComplateTodo(0)
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

// fn call<E: JsCast + 'static>(ctx: &yew::Context<Model>, callback: fn(E) -> Func) -> Callback<E> {
//     ctx.link().callback(move |e: E| callback(e))
// }

// fn id(element: Element) -> usize {
//     element.id().parse::<usize>().expect("")
// }

// // インプットイベントから要素を抽出しコールバックを実行
// fn get<Element: JsCast>(event: &Event, callback: fn(Element) -> Func) -> Func {
//     if let Some(target) = event.target() {
//         if let Some(element) = target.dyn_into::<Element>().ok() {
//             return callback(element);
//         }
//     }
//     Func::None
// }

fn main() {
    yew::start_app::<Model>();
}
