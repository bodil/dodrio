use dodrio::bumpalo::Bump;
use dodrio::{Node, Render, Vdom};
use dodrio_js_api::JsRender;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Import the JS `Greeting` class.
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    type Greeting;

    // And the `Greeting` class's constructor.
    #[wasm_bindgen(constructor)]
    fn new(who: &str) -> Greeting;
}

/// This is our Rust rendering component that wraps the JS rendering component.
pub struct GreetingViaJs {
    js: JsRender,
}

impl GreetingViaJs {
    /// Create a new `GreetingViaJs`, which will internally create a new JS
    /// `Greeting`.
    pub fn new(who: &str) -> GreetingViaJs {
        let js = JsRender::new(Greeting::new(who));
        GreetingViaJs { js }
    }
}

/// Here's the `Render` implementation! This adds a `<p>` element and some text
/// around whatever the inner JS `Greeting` component renders.
impl Render for GreetingViaJs {
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        use dodrio::builder::*;
        p(bump)
            .children([text("JavaScript says: "), self.js.render(bump)])
            .finish()
    }
}

/// Finally our main initialization function that kicks everything off!
#[wasm_bindgen(start)]
pub fn run() {
    // Set up the panic hook for debugging when things go wrong.
    console_error_panic_hook::set_once();

    // Grab the document's `<body>`.
    let window = web_sys::window().unwrap_throw();
    let document = window.document().unwrap_throw();
    let body = document.body().unwrap_throw();

    // Create a new `GreetingViaJs` render component.
    let component = GreetingViaJs::new("World");

    // Create a virtual DOM and mount it and the `Hello` render component to the
    // `<body>`.
    let vdom = Vdom::new(body.as_ref(), component);

    // Run the virtual DOM forever and don't unmount it.
    vdom.forget();
}
