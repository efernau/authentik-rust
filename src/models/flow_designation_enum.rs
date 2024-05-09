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

/// FlowDesignationEnum : * `authentication` - Authentication * `authorization` - Authorization * `invalidation` - Invalidation * `enrollment` - Enrollment * `unenrollment` - Unrenollment * `recovery` - Recovery * `stage_configuration` - Stage Configuration
/// * `authentication` - Authentication * `authorization` - Authorization * `invalidation` - Invalidation * `enrollment` - Enrollment * `unenrollment` - Unrenollment * `recovery` - Recovery * `stage_configuration` - Stage Configuration
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FlowDesignationEnum {
    #[serde(rename = "authentication")]
    Authentication,
    #[serde(rename = "authorization")]
    Authorization,
    #[serde(rename = "invalidation")]
    Invalidation,
    #[serde(rename = "enrollment")]
    Enrollment,
    #[serde(rename = "unenrollment")]
    Unenrollment,
    #[serde(rename = "recovery")]
    Recovery,
    #[serde(rename = "stage_configuration")]
    StageConfiguration,

}

impl ToString for FlowDesignationEnum {
    fn to_string(&self) -> String {
        match self {
            Self::Authentication => String::from("authentication"),
            Self::Authorization => String::from("authorization"),
            Self::Invalidation => String::from("invalidation"),
            Self::Enrollment => String::from("enrollment"),
            Self::Unenrollment => String::from("unenrollment"),
            Self::Recovery => String::from("recovery"),
            Self::StageConfiguration => String::from("stage_configuration"),
        }
    }
}

impl Default for FlowDesignationEnum {
    fn default() -> FlowDesignationEnum {
        Self::Authentication
    }
}

