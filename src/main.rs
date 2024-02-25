mod app;
mod about;

use app::App;
use about::About;

fn main() {
    yew::Renderer::<App>::new().render();
}