use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NotFoundComponentProps {
}

#[function_component(NotFound)]
pub fn not_found_component(NotFoundComponentProps {}: &NotFoundComponentProps) -> Html {
    html! { <h1 class={classes!("mountain-text")}>{ "404" }</h1> }
}
