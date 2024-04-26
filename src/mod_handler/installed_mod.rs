pub struct InstalledMod {
    pub slug: String,
    pub display_name: String,
    pub short_desc: String,
    pub long_desc: String,
    pub mc_version: String,
    pub version: String,
    pub path: String
}

impl InstalledMod {
    pub fn check_for_update(&mut self) -> bool {
        // Check for updates
        return false;
    }
}