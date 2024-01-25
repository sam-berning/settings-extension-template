variable::set("SettingStruct", to_upper_camel_case(variable::get("setting-name")) + "SettingsV1");
variable::set("setting_variable", to_snake_case(variable::get("setting-name")));
