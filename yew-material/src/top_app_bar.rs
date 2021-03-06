pub mod action_items;
pub mod navigation_icon;
pub mod title;

pub use action_items::MatTopAppBarActionItems;
pub use navigation_icon::MatTopAppBarNavigationIcon;
pub use title::MatTopAppBarTitle;

use crate::{add_event_listener, to_option};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/../build/mwc-top-app-bar.js")]
extern "C" {
    #[derive(Debug)]
    type TopAppBar;

    #[wasm_bindgen(getter, static_method_of = TopAppBar)]
    fn _dummy_loader() -> JsValue;
}

loader_hack!(TopAppBar);

/// The `mwc-top-app-bar` component
///
/// [MWC Documentation](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar)
pub struct MatTopAppBar {
    props: Props,
    node_ref: NodeRef,
    closure: Option<Closure<dyn FnMut()>>,
}

/// Props for [`MatTopAppBar`]
///
/// MWC Documentation:
///
/// - [Properties](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar#propertiesattributes)
/// - [Events](https://github.com/material-components/material-components-web-components/tree/master/packages/top-app-bar#events)
#[derive(Debug, Properties, Clone)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub center_title: bool,
    #[prop_or_default]
    pub dense: bool,
    #[prop_or_default]
    pub prominent: bool,
    /// Binds to `MDCTopAppBar:nav`
    ///
    /// See events docs to learn more.
    #[prop_or_default]
    pub onnavigationiconclick: Callback<()>,
}

impl Component for MatTopAppBar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        TopAppBar::ensure_loaded();
        Self {
            props,
            node_ref: NodeRef::default(),
            closure: None,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <mwc-top-app-bar
                centerTitle?=to_option(self.props.center_title)
                dense?=to_option(self.props.dense)
                prominent?=to_option(self.props.prominent)
                ref=self.node_ref.clone()
            >
                { self.props.children.clone() }
            </mwc-top-app-bar>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let callback = self.props.onnavigationiconclick.clone();
            add_event_listener(
                &self.node_ref,
                "MDCTopAppBar:nav",
                move || {
                    callback.emit(());
                },
                &mut self.closure,
            );
        }
    }
}
