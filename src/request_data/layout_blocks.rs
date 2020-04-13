#![allow(dead_code)]

use crate::request_data::block_elements::BlockElement;
use crate::request_data::composite_object::TextObject;
use serde::{Serialize, Deserialize};
use crate::ConfirmationDialogObject;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LayoutBlock {
    Section {
        #[serde(flatten)]
        section: Section
    },
    Divider {
        #[serde(flatten)]
        divider: Divider
    },
    Image {
        #[serde(flatten)]
        image: Image
    },
    Actions {
        #[serde(flatten)]
        actions: Actions
    },
    Context {
        #[serde(flatten)]
        context: Context
    },
    File {
        #[serde(flatten)]
        file: File
    },
    Input {
        label: TextObject,
        element: TextInputElement,
        #[serde(skip_serializing_if = "Option::is_none")]
        block_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        optional: Option<bool>,
    },
}

impl LayoutBlock {
    pub fn new_section<T: Into<TextObject>>(text_object: T) -> Section {
        Section {
            text: text_object.into(),
            block_id: None,
            fields: None,
            accessory: None,
        }
    }

    pub fn new_divider() -> Divider {
        Divider {
            block_id: None
        }
    }

    pub fn new_image<T: Into<TextObject>>(image_url: T, alt_text: T) -> Image {
        Image {
            image_url: image_url.into(),
            alt_text: alt_text.into(),
            title: None,
            block_id: None,
        }
    }

    pub fn new_action<T: Into<BlockElement>>(elements: Vec<T>) -> Actions {
        Actions {
            elements: elements.into_iter().map(|e| e.into()).collect(),
            block_id: None,
        }
    }

    pub fn new_context<T: Into<BlockElement>>(elements: Vec<T>) -> Context {
        Context {
            elements: elements.into_iter().map(|e| e.into()).collect(),
            block_id: None,
        }
    }

    pub fn new_file(external_id: String, source: String) -> File {
        File {
            external_id,
            source,
            block_id: None,
        }
    }

    pub fn new_plain_text_input<T: Into<TextObject>>(label: T, action_id: String) -> LayoutBlock {
        LayoutBlock::Input {
            label: label.into(),
            element: TextInputElement {
                r#type: "plain_text_input".to_owned(),
                action_id: action_id.clone(),
                multiline: Some(true),
                ..Default::default()
            },
            block_id: Some(action_id),
            optional: Some(true),
        }
    }

    pub fn new_plain_single_line_text_input<T: Into<TextObject>>(
        label: T,
        action_id: String,
        place_holder: T
    ) -> LayoutBlock {
        LayoutBlock::Input {
            label: label.into(),
            element: TextInputElement {
                r#type: "plain_text_input".to_owned(),
                action_id: action_id.clone(),
                multiline: Some(false),
                placeholder: Some(place_holder.into()),
                ..Default::default()
            },
            block_id: Some(action_id),
            optional: Some(true),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Section {
    text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<TextObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accessory: Option<BlockElement>,
}

impl Into<LayoutBlock> for Section {
    fn into(self) -> LayoutBlock {
        LayoutBlock::Section {
            section: self
        }
    }
}

impl Section {
    pub fn set_text(mut self, text: TextObject) -> Section {
        self.text = text;
        self
    }

    pub fn set_block_id(mut self, block_id: String) -> Section {
        self.block_id = Some(block_id);
        self
    }

    pub fn set_fields(mut self, fields: Vec<TextObject>) -> Section {
        self.fields = Some(fields);
        self
    }

    pub fn add_field(mut self, field: TextObject) -> Section {
        if let Some(ref mut vec) = self.fields {
            vec.push(field)
        } else {
            self.fields = Some(vec![field])
        }
        self
    }

    pub fn set_accessory<T: Into<BlockElement>>(mut self, accessory: T) -> Section {
        self.accessory = Some(accessory.into());
        self
    }

    pub fn build(self) -> LayoutBlock {
        LayoutBlock::Section {
            section: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Divider {
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Into<LayoutBlock> for Divider {
    fn into(self) -> LayoutBlock {
        LayoutBlock::Divider {
            divider: self
        }
    }
}

impl Divider {
    pub fn set_block_id(mut self, block_id: String) -> Divider {
        self.block_id = Some(block_id);
        self
    }

    pub fn build(self) -> LayoutBlock {
        LayoutBlock::Divider {
            divider: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    image_url: TextObject,
    alt_text: TextObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Into<LayoutBlock> for Image {
    fn into(self) -> LayoutBlock {
        LayoutBlock::Image {
            image: self
        }
    }
}

impl Image {
    pub fn set_image_url(mut self, image_url: TextObject) -> Image {
        self.image_url = image_url;
        self
    }

    pub fn set_alt_text(mut self, alt_text: TextObject) -> Image {
        self.alt_text = alt_text;
        self
    }

    pub fn set_title(mut self, title: String) -> Image {
        self.title = Some(title);
        self
    }

    pub fn set_block_id(mut self, block_id: String) -> Image {
        self.block_id = Some(block_id);
        self
    }

    pub fn build(self) -> LayoutBlock {
        LayoutBlock::Image {
            image: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Actions {
    elements: Vec<BlockElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Into<LayoutBlock> for Actions {
    fn into(self) -> LayoutBlock {
        LayoutBlock::Actions {
            actions: self
        }
    }
}

impl Actions {
    pub fn set_elements(mut self, elements: Vec<BlockElement>) -> Actions {
        self.elements = elements;
        self
    }

    pub fn add_element(mut self, element: BlockElement) -> Actions {
        self.elements.push(element);
        self
    }

    pub fn set_block_id(mut self, block_id: String) -> Actions {
        self.block_id = Some(block_id);
        self
    }

    pub fn build(self) -> LayoutBlock {
        LayoutBlock::Actions {
            actions: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct Context {
    elements: Vec<BlockElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Into<LayoutBlock> for Context {
    fn into(self) -> LayoutBlock {
        LayoutBlock::Context {
            context: self
        }
    }
}

impl Context {
    pub fn set_elements<T: Into<BlockElement>>(mut self, elements: Vec<T>) -> Context {
        self.elements = elements.into_iter().map(Into::into).collect();
        self
    }

    pub fn add_element<T: Into<BlockElement>>(mut self, element: T) -> Context {
        self.elements.push(element.into());
        self
    }
    pub fn set_block_id(mut self, block_id: String) -> Context {
        self.block_id = Some(block_id);
        self
    }

    pub fn build(self) -> LayoutBlock {
        LayoutBlock::Context {
            context: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(default)]
pub struct File {
    external_id: String,
    source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Into<LayoutBlock> for File {
    fn into(self) -> LayoutBlock {
        LayoutBlock::File {
            file: self
        }
    }
}

impl File {
    pub fn set_external_id(mut self, external_id: String) -> File {
        self.external_id = external_id;
        self
    }

    pub fn set_source(mut self, source: String) -> File {
        self.source = source;
        self
    }

    pub fn set_block_id(mut self, block_id: String) -> File {
        self.block_id = Some(block_id);
        self
    }

    pub fn build(self) -> LayoutBlock {
        LayoutBlock::File {
            file: self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TextInputElement {
    r#type: String,
    pub action_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialogObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiline: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<i32>,
}
