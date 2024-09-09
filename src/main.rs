use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100vh; margin: 0;">
            <button style="font-size: 2rem; padding: 1rem 2rem;" {onclick}>{ "+1" }</button>
            <p style="font-size: 2rem; margin: 1rem 0;">{ *counter }</p>

            <div style="display: flex; justify-content: center; align-items: center; height: 100vh;">
                <h1 style="font-size: 5rem;">{ "Barni" }</h1>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
