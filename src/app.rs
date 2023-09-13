use yew::prelude::*;

#[function_component]
fn Nav() -> Html {
    html! {
        <nav> 
            <ul>
                <li>{ "Nav Here" }</li>
            </ul>
        </nav>
    }
}

#[function_component]
pub fn App() -> Html {
    
    html! {
        <main>
            <Nav />
            <header>{ "RestedWicked" }</header>
            <p>{ "gaming" }</p>
        </main>
    }
}
