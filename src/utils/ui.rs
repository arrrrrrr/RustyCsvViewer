/// Convert a menu string with an accelerator key to one without and in lowercase
pub fn menu_resource_to_lc(name: &str) -> String {
    name.to_lowercase().replace("&", "")
}