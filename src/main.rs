use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let plusbutton = {
        let onclick = {
            let counter = counter.clone();
            move |_| {
                let value = *counter + 1;
                counter.set(value);
            }
        };
        html! {
            <button {onclick}> { "+1" } </button>
        }
    };
    let minusbutton = {
        let onclick = {
            let counter = counter.clone();
            move |_| {
                let value = *counter - 1;
                counter.set(value);
            }
        };
        html! {
            <button {onclick}> { "-1" } </button>
        }
    };

    let mut two = AttrValue::from("I am not two!");
    if *counter == 2 {
        two = AttrValue::from("I am two!");
    };

    html! {
        <div>
            { minusbutton }
            { plusbutton }
            <p>{ *counter }</p>
            <p>{ two }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
