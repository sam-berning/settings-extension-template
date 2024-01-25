use bottlerocket_settings_sdk::{BottlerocketSetting, NullMigratorExtensionBuilder};
use {{crate_name}}::{{SettingStruct}};
use std::process::ExitCode;

fn main() -> ExitCode {
    env_logger::init();

    match NullMigratorExtensionBuilder::with_name("{{setting-name}}")
        .with_models(vec![BottlerocketSetting::<{{SettingStruct}}>::model()])
        .build()
    {
        Ok(extension) => extension.run(),
        Err(e) => {
            println!("{}", e);
            ExitCode::FAILURE
        }
    }
}
