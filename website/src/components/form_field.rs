use crate::components::Codeblock;
use crate::with_raw_code;
use yew::prelude::*;
use yew_material::{MatCheckbox, MatFormfield, MatRadio, MatSwitch};

pub struct FormField {}

impl Component for FormField {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let checkbox = with_raw_code!(checkbox { html! {
        <section>
            <div class="demo">
                <h3>{"Standard"}</h3>
                <MatFormfield label="This is a checkbox">
                    <MatCheckbox />
                </MatFormfield>
            </div>

            <div class="demo">
                <h3>{"Align End"}</h3>
                <MatFormfield label="This is another checkbox" align_end=true>
                    <MatCheckbox />
                </MatFormfield>
            </div>
        </section> }});

        let switch = with_raw_code!(switch { html! {
        <section class="demo">
            <MatFormfield label="This is a switch">
                <MatSwitch />
            </MatFormfield>
        </section>
        }});

        let radio = with_raw_code!(radio { html! {
        <section class="demo">
            <MatFormfield label="This is a radio button">
                <MatRadio name="radio-name" />
            </MatFormfield>
        </section>
        }});

        html! {<>
            <Codeblock title="Checkbox" code_and_html=checkbox />

            <Codeblock title="Switch" code_and_html=switch />

            <Codeblock title="Radio" code_and_html=radio />
        </>}
    }
}
