mod mod_handler;

extern crate directories;

fn main() {
    let dirs = directories::ProjectDirs::from("xyz", "hexdev", "pkgmc");

    let installed_mod = mod_handler::installed_mod::InstalledMod {
        slug: "test_mod".to_string(),
        display_name: "Test Mod".to_string(),
        short_desc: "A test mod".to_string(),
        long_desc: "A test mod but this is a long bit of text".to_string(),
        mc_version: "1.20.4".to_string(),
        version: "355259".to_string(),
        path: dirs.unwrap().data_dir().display().to_string()

    };

    println!("{}", installed_mod.display_name);
}