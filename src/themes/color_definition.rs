use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize};


#[derive(Debug, Serialize)]
pub struct ColorDefinition {
    pub color_name: String,
    pub hex: String,
    pub notes: String,

    #[serde(skip)]
    color: Color,
}

impl ColorDefinition {
    pub fn to_color(&self) -> Color {
        self.color
    }
}

/// Custom deserialization logic to convert hex string to Color type during deserialization.
impl<'de> Deserialize<'de> for ColorDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize only the serializable part
        #[derive(Deserialize)]
        struct ColorDefRaw {
            color_name: String,
            hex: String,
            notes: String,
        }

        let raw = ColorDefRaw::deserialize(deserializer)?;

        let parsed_color = Srgba::hex(&raw.hex)
            .map(Color::from)
            .map_err(serde::de::Error::custom)?;

        Ok(ColorDefinition {
            color_name: raw.color_name,
            hex: raw.hex,
            notes: raw.notes,
            color: parsed_color,
        })
    }
}