/*
 * authentik
 *
 * Making authentication simple.
 *
 * The version of the OpenAPI document: 2024.2.1
 * Contact: hello@goauthentik.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// PromptTypeEnum : * `text` - Text: Simple Text input * `text_area` - Text area: Multiline Text Input. * `text_read_only` - Text (read-only): Simple Text input, but cannot be edited. * `text_area_read_only` - Text area (read-only): Multiline Text input, but cannot be edited. * `username` - Username: Same as Text input, but checks for and prevents duplicate usernames. * `email` - Email: Text field with Email type. * `password` - Password: Masked input, multiple inputs of this type on the same prompt need to be identical. * `number` - Number * `checkbox` - Checkbox * `radio-button-group` - Fixed choice field rendered as a group of radio buttons. * `dropdown` - Fixed choice field rendered as a dropdown. * `date` - Date * `date-time` - Date Time * `file` - File: File upload for arbitrary files. File content will be available in flow context as data-URI * `separator` - Separator: Static Separator Line * `hidden` - Hidden: Hidden field, can be used to insert data into form. * `static` - Static: Static value, displayed as-is. * `ak-locale` - authentik: Selection of locales authentik supports
/// * `text` - Text: Simple Text input * `text_area` - Text area: Multiline Text Input. * `text_read_only` - Text (read-only): Simple Text input, but cannot be edited. * `text_area_read_only` - Text area (read-only): Multiline Text input, but cannot be edited. * `username` - Username: Same as Text input, but checks for and prevents duplicate usernames. * `email` - Email: Text field with Email type. * `password` - Password: Masked input, multiple inputs of this type on the same prompt need to be identical. * `number` - Number * `checkbox` - Checkbox * `radio-button-group` - Fixed choice field rendered as a group of radio buttons. * `dropdown` - Fixed choice field rendered as a dropdown. * `date` - Date * `date-time` - Date Time * `file` - File: File upload for arbitrary files. File content will be available in flow context as data-URI * `separator` - Separator: Static Separator Line * `hidden` - Hidden: Hidden field, can be used to insert data into form. * `static` - Static: Static value, displayed as-is. * `ak-locale` - authentik: Selection of locales authentik supports
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PromptTypeEnum {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "text_area")]
    TextArea,
    #[serde(rename = "text_read_only")]
    TextReadOnly,
    #[serde(rename = "text_area_read_only")]
    TextAreaReadOnly,
    #[serde(rename = "username")]
    Username,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "password")]
    Password,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "checkbox")]
    Checkbox,
    #[serde(rename = "radio-button-group")]
    RadioButtonGroup,
    #[serde(rename = "dropdown")]
    Dropdown,
    #[serde(rename = "date")]
    Date,
    #[serde(rename = "date-time")]
    DateTime,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "separator")]
    Separator,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "static")]
    Static,
    #[serde(rename = "ak-locale")]
    AkLocale,

}

impl ToString for PromptTypeEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Text => String::from("text"),
            Self::TextArea => String::from("text_area"),
            Self::TextReadOnly => String::from("text_read_only"),
            Self::TextAreaReadOnly => String::from("text_area_read_only"),
            Self::Username => String::from("username"),
            Self::Email => String::from("email"),
            Self::Password => String::from("password"),
            Self::Number => String::from("number"),
            Self::Checkbox => String::from("checkbox"),
            Self::RadioButtonGroup => String::from("radio-button-group"),
            Self::Dropdown => String::from("dropdown"),
            Self::Date => String::from("date"),
            Self::DateTime => String::from("date-time"),
            Self::File => String::from("file"),
            Self::Separator => String::from("separator"),
            Self::Hidden => String::from("hidden"),
            Self::Static => String::from("static"),
            Self::AkLocale => String::from("ak-locale"),
        }
    }
}

impl Default for PromptTypeEnum {
    fn default() -> PromptTypeEnum {
        Self::Text
    }
}

