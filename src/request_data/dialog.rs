use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Dialog {
    callback_id: String,
    title: String,
    submit_label: String,
    notify_on_cancel: bool,
    state: String,
    elements: Vec<DialogElement>,
}

impl Dialog {
    pub fn new_dialog(title: &str, submit_label: &str) -> Dialog {
        Dialog {
            callback_id: thread_rng().sample_iter(&Alphanumeric).take(25).collect(),
            title: title.to_owned(),
            submit_label: submit_label.to_owned(),
            notify_on_cancel: false,
            state: "".to_string(),
            elements: Default::default(),
        }
    }

    pub fn new_dialog_with_elements<T: Into<DialogElement>>(
        title: &str,
        submit_label: &str,
        elements: Vec<T>,
    ) -> Dialog {
        Dialog {
            callback_id: thread_rng().sample_iter(&Alphanumeric).take(25).collect(),
            title: title.to_owned(),
            submit_label: submit_label.to_owned(),
            notify_on_cancel: false,
            state: "".to_string(),
            elements: elements.into_iter().map(|e| e.into()).collect(),
        }
    }

    pub fn new_dialog_with_callback(
        title: &str,
        callback_id: &str,
        submit_label: &str,
    ) -> Dialog {
        Dialog {
            callback_id: callback_id.to_owned(),
            title: title.to_owned(),
            submit_label: submit_label.to_owned(),
            notify_on_cancel: false,
            state: "".to_string(),
            elements: Default::default(),
        }
    }

    pub fn add_element<T: Into<DialogElement>>(mut self, element: T) -> Dialog {
        self.elements.push(element.into());
        self
    }

    pub fn set_state(mut self, state: &str) -> Dialog {
        self.state = state.to_owned();
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DialogElement {
    Text {
        #[serde(flatten)]
        text_element: TextElement
    },
    //    Textarea {
//        #[serde(faltten)]
//        text_area: Textarea
//    },
    Select {
        #[serde(flatten)]
        select: Select
    },

    SelectOptions {
        #[serde(flatten)]
        select: Select
    },
}

impl DialogElement {
    pub fn new_text_element(label: &str, name: &str) -> TextElement {
        TextElement {
            label: label.to_owned(),
            name: name.to_owned(),
            max_length: None,
            min_length: None,
            optional: true,
            hint: None,
            subtype: None,
            value: None,
            placeholder: None,
        }
    }

    pub fn new_select_element(label: &str, name: &str) -> SelectOptions {
        SelectOptions {
            label: label.to_owned(),
            name: name.to_owned(),
            option_groups: Vec::new(),
        }
    }

    pub fn new_select_element_with_options_groups<T: Into<DialogOptionGroup>>(
        label: &str,
        name: &str,
        options: Vec<T>,
    ) -> SelectOptions {
        SelectOptions {
            label: label.to_owned(),
            name: name.to_owned(),
            option_groups: options.into_iter().map(|e| e.into()).collect(),
        }
    }

    pub fn new_select_element_with_options<T: Into<DialogOption>>(
        label: &str,
        name: &str,
        options: Vec<T>,
    ) -> Select {
        Select {
            label: label.to_owned(),
            name: name.to_owned(),
            options: options.into_iter().map(|e| e.into()).collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Subtype {
    Email,
    Number,
    Tel,
    Url,
}

impl Into<DialogElement> for TextElement {
    fn into(self) -> DialogElement {
        DialogElement::Text {
            text_element: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct TextElement {
    label: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<u8>,
    optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtype: Option<Subtype>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<String>,
}

impl TextElement {
    pub fn set_hint(mut self, hint: &str) -> TextElement {
        self.hint = Some(hint.to_owned());
        self
    }

    pub fn set_subtype(mut self, subtype: Subtype) -> TextElement {
        self.subtype = Some(subtype);
        self
    }

    pub fn set_placeholder(mut self, placeholder: &str) -> TextElement {
        self.placeholder = Some(placeholder.to_owned());
        self
    }

    pub fn set_value(mut self, value: &str) -> TextElement {
        self.value = Some(value.to_owned());
        self
    }

    pub fn set_min_max_length(mut self, min: u8, max: u8) -> TextElement {
        self.min_length = Some(min);
        self.max_length = Some(max);
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Select {
    label: String,
    name: String,
    options: Vec<DialogOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct SelectOptions {
    label: String,
    name: String,
    option_groups: Vec<DialogOptionGroup>,
}

impl Into<DialogElement> for Select {
    fn into(self) -> DialogElement {
        DialogElement::Select {
            select: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogOption {
    pub label: String,
    pub value: String,
}

impl DialogOption {
    pub fn new_dialog_option(label: &str) -> DialogOption {
        DialogOption {
            label: label.to_string(),
            value: label.replace(" ", "").to_lowercase(),
        }
    }

    pub fn set_value(mut self, value: &str) -> DialogOption {
        self.value = value.to_owned();
        self
    }
}

impl Into<DialogOption> for &str {
    fn into(self) -> DialogOption {
        DialogOption::new_dialog_option(self)
    }
}

impl Into<DialogOption> for String {
    fn into(self) -> DialogOption {
        DialogOption::new_dialog_option(&self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogOptionGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub options: Vec<DialogOption>,
}

impl DialogOptionGroup {
    pub fn new_dialog_option() -> DialogOptionGroup {
        DialogOptionGroup {
            label: None,
            options: vec![],
        }
    }

    pub fn new_dialog_option_group<T: Into<DialogOption>>(options: Vec<T>) -> DialogOptionGroup {
        DialogOptionGroup {
            label: None,
            options: options.into_iter().map(|e| e.into()).collect(),
        }
    }

    pub fn new_dialog_option_group_with_label(label: &str) -> DialogOptionGroup {
        DialogOptionGroup {
            label: Some(label.to_owned()),
            options: vec![],
        }
    }

    pub fn add_option<T: Into<DialogOption>>(mut self, option: T) -> DialogOptionGroup {
        self.options.push(option.into());
        self
    }
}