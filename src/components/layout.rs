use yew::{function_component, html, Html, Properties, Children};

mod navbar;
mod footer;

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    html! {
        <div>
            <main>
                { for props.children.iter()}
            </main>
        </div>
    }
}
 
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children, // the field name `children` is important!
}
