use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct LinksComponentProps {
}

#[function_component(Links)]
pub fn links_component(LinksComponentProps {}: &LinksComponentProps) -> Html {
    html! { <h1 class={classes!("mountain-text")}><a href="https://youtu.be/dQw4w9WgXcQ">{"Other projects"}</a></h1> }
}
