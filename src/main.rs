fn main() {
    println!("Starting Minecraft Server Creator...");
    let version: Vec<&str> = env!("CARGO_PKG_VERSION").split("-").collect();
    let revision: &str = env!("GIT_SHORT_HASH");
    println!("Version {0} {1} ({2})", version[0], version[1], revision);
}
