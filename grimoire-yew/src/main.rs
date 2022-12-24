use wasm_bindgen::JsValue;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[wasm_bindgen(module="/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name=invoke_open_grimoire_dialog, catch)]
    pub async fn open_grimoire_dialog() -> Result<JsValue, JsValue>;
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    // let name = use_state_eq(|| "World".to_string());
    // let n2 = name.clone();
    //
    // use_effect_with_deps (
    //     move |name| {
    //         grimoire_dialog();
    //         || ()
    //     },
    //     (n2)
    // );

    let onclick = Callback::from(
        move |_| {
            grimoire_dialog();
        }
    );

    html! {
        //<h1>{ "Hello world" }</h1>
        <button {onclick}>{ "Open Grimoire" }</button>
    }
}

fn grimoire_dialog() {
    spawn_local(
        async move {
            open_grimoire_dialog().await;
        }
    )
}