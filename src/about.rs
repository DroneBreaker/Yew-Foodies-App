use yew::prelude::*;

#[function_component(About)]
pub fn AboutPage() -> Html {
    html! {
        <main>
            <h1>{ "Welcome to About Bitches!" }</h1>
            <span class="subtitle">{ "from Yew and DroneBreaker with " }<i class="heart" /></span>

            <footer>
                <p>{ "Hello World Bye!" }</p>
            </footer>
        </main>
    }
}