use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::menu::Menu;
use crate::enums::routes::{Route, switch};

#[function_component(App)]
pub fn app_component() -> Html {
    html! {
        <main class={classes!("full-height", "full-width")}>
            <img src="yellow_mountain.svg" alt="mountain background" class={classes!("mountain")}/>
            <img src="yellow_moon.svg" alt="moon background" class={classes!("moon")}/>
            <BrowserRouter>
                <Menu title={ "Archon's Design" }/>
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </BrowserRouter>
        </main>
    }
}
