/// {{setting-name}} settings extension
/// provides {{SettingStruct}}
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

// to provide all of these traits and automatically set every field as optional, you can use the
// #[model] macro from model-derive in the main bottlerocket repo.
#[derive(Deserialize, Serialize, Default, Debug, PartialEq, Eq)]
pub struct {{SettingStruct}} {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for {{SettingStruct}} {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // allow anything that parses as {{SettingStruct}}
        Ok(())
    }

    fn generate(
        _existing_partial: Option<Self::PartialKind>,
        _dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialKind, Self>> {
        Ok(GenerateResult::Complete({{SettingStruct}} { name: None }))
    }

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_{{setting_variable}}() {
        let generated = {{SettingStruct}}::generate(None, None).unwrap();
        assert_eq!(
            generated,
            GenerateResult::Complete({{SettingStruct}} { name: None })
        )
    }

    #[test]
    fn test_serde_{{setting_variable}}() {
        let test_json = r#"{
            "name": "foo"
        }"#;

        let {{setting_variable}}: {{SettingStruct}} = serde_json::from_str(test_json).unwrap();

        assert_eq!(
            {{setting_variable}},
            {{SettingStruct}} {
                name: Some(String::from("foo")),
            }
        );
    }
}
