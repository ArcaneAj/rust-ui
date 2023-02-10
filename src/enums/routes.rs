use yew::prelude::*;
use yew_router::prelude::*;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <div></div> },
        Route::Contact => html! { <h1 class={classes!("mountain-text")}>{"Don't contact me"}</h1> },
        Route::Links => html! { <h1 class={classes!("mountain-text")}><a href="https://youtu.be/dQw4w9WgXcQ">{"Other projects"}</a></h1> },
        Route::NotFound => html! { <h1 class={classes!("mountain-text")}>{ "404" }</h1> },
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/contact")]
    Contact,
    #[at("/links")]
    Links,
    #[not_found]
    #[at("/404")]
    NotFound,
}