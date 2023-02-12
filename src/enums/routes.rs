use yew::prelude::*;
use yew_router::prelude::*;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <div></div> },
        Route::Contact => html! { <h1 class={classes!("mountain-text")}>{"Don't contact me"}</h1> },
        Route::Links => html! { <h1 class={classes!("mountain-text")}><a href="https://youtu.be/dQw4w9WgXcQ">{"Other projects"}</a></h1> },
        Route::VirtualManCave => html! {
            <h1 class={classes!("mountain-text")}>
            <title>{"Certified old school time wasters:"}</title>
            <p>{ "Vsauce" }</p>
            <iframe width="560" height="315" src="https://www.youtube.com/embed/jHbyQ_AQP8c" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true></iframe>
            <p> { "Cool math games" } </p>
            <iframe src="https://www.miniplay.com/embed/bloxorz" width="600" height="400" frameborder="0" allowfullscreen=true></iframe>
            // todo: embed this properly without ruining the webpage layout
            // <p> {"park the taxi game"} </p>
            // <iframe src="https://1000webgames.com/games/parkthetaxi/html5/" frameborder="0" width="800" height="450" scrolling="no" allowfullscreen=true></iframe>
            </h1>
        },
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
    #[at("/virtual_man_cave")]
    VirtualManCave,
    #[not_found]
    #[at("/404")]
    NotFound,
}