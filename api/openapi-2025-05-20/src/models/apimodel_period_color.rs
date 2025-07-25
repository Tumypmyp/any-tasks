/*
 * Anytype API
 *
 * This API enables seamless interaction with Anytype's resources - spaces, objects, properties, types, templates, and beyond.
 *
 * The version of the OpenAPI document: 2025-05-20
 * Contact: support@anytype.io
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ApimodelPeriodColor : The color of the icon
/// The color of the icon
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApimodelPeriodColor {
    #[serde(rename = "grey")]
    ColorGrey,
    #[serde(rename = "yellow")]
    ColorYellow,
    #[serde(rename = "orange")]
    ColorOrange,
    #[serde(rename = "red")]
    ColorRed,
    #[serde(rename = "pink")]
    ColorPink,
    #[serde(rename = "purple")]
    ColorPurple,
    #[serde(rename = "blue")]
    ColorBlue,
    #[serde(rename = "ice")]
    ColorIce,
    #[serde(rename = "teal")]
    ColorTeal,
    #[serde(rename = "lime")]
    ColorLime,

}

impl std::fmt::Display for ApimodelPeriodColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ColorGrey => write!(f, "grey"),
            Self::ColorYellow => write!(f, "yellow"),
            Self::ColorOrange => write!(f, "orange"),
            Self::ColorRed => write!(f, "red"),
            Self::ColorPink => write!(f, "pink"),
            Self::ColorPurple => write!(f, "purple"),
            Self::ColorBlue => write!(f, "blue"),
            Self::ColorIce => write!(f, "ice"),
            Self::ColorTeal => write!(f, "teal"),
            Self::ColorLime => write!(f, "lime"),
        }
    }
}

impl Default for ApimodelPeriodColor {
    fn default() -> ApimodelPeriodColor {
        Self::ColorGrey
    }
}

