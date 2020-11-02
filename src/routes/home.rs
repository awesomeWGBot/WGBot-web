use yew::prelude::*;
use crate::components::calendar::Calendar;
use crate::components::rules::Rules;

/// Home page
pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {

            /*<div class="app">
                <header class="app-header">

                    <a
                        class="app-logo"
                        href="https://yew.rs"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                    </a>
                    <p>
                        { "Edit " } <code>{ "src/routes/home.rs" }</code> { " and saves to reload." }
                    </p>
                    <a
                        id="learn_yew"
                        class="app-link"
                        href="https://yew.rs"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        { "Learn Yew" }
                    </a>
                </header></div>*/
                <>
                <div class="container-fluid mt-1">
                <h3>{ "Wochenübersicht" }</h3>
                <Calendar />
                <h3>{ "Regelmäßige Events" }</h3>
                <Rules />
                </div>
                </>
        }
    }
}
