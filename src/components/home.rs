use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct HomeComponentProps {
}

#[function_component(Home)]
pub fn home_component(HomeComponentProps {}: &HomeComponentProps) -> Html {
    html! { <div></div> }
}
