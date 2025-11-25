use dioxus::prelude::*;

/// Input component
#[component]
pub fn Input(props: InputProps) -> Element {
    let mut response = use_signal(|| String::new());
    // let oninput = match props.oninput {
    //     Some(x) => x,
    //     None => move |e: Event<FormData>| {
    //         response.set(format!("INPUT: {:?}", e));
    //     },
    // };

    rsx! {
        div {
            class:"input",

            if let Some(label_txt) = props.label {
                label {"{label_txt}"}
            }

            input {
                placeholder: props.placeholder,
                value: props.value,
                oninput: move |e| {
                    if let Some(oninput) = props.oninput {
                        oninput(e)
                    }
                },
            }

            if let Some(err) = props.error {
                p { class:"error" }
            }


            if !response().is_empty() {
                p {
                    "Server logged in: "
                    i { "{response}" }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    /// The class attribute
    pub class: Option<String>,

    /// The label of input
    pub label: Option<String>,

    /// The placeholder of input
    pub placeholder: Option<String>,

    /// The value of input
    pub value: Option<String>,

    /// The error note for input
    pub error: Option<String>,

    /// The onclick event handler.
    pub onclick: Option<EventHandler<MouseEvent>>,

    /// The oninput event handler.
    pub oninput: Option<EventHandler<Event<FormData>>>,

    #[props(extends = GlobalAttributes)]
    attributes: Vec<Attribute>,
}
