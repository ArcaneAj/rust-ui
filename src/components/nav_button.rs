use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NavButtonComponentProps {
    pub text: AttrValue,
    pub onclick: Callback<MouseEvent>
}

#[function_component(NavButton)]
pub fn nav_button_component(NavButtonComponentProps { text, onclick }: &NavButtonComponentProps) -> Html {
    let on_click = onclick.clone();
    html! { <button class={classes!("nav-button")} onclick={move |e: MouseEvent| on_click.emit(e.clone())}>{ text }</button> }
}
