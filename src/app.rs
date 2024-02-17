use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello World Bitches!" }</h1>
            <span class="subtitle">{ "from Yew and DroneBreaker with " }<i class="heart" /></span>

            <footer>
            <p>{ "Hello World Bye!" }</p>
            </footer>
        </main>
    }
}
