use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::{home::Home, contact::Contact, links::Links, virtual_man_cave::VirtualManCave, not_found::NotFound};

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Contact => html! { <Contact/> },
        Route::Links => html! { <Links/> },
        Route::VirtualManCave => html! { <VirtualManCave/> },
        Route::NotFound => html! { <NotFound/> },
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
    #[at("/virtual_man_cave")]
    VirtualManCave,
    #[not_found]
    #[at("/404")]
    NotFound,
}