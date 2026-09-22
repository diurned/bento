pub fn fs() {
    let config_path = format!("{}/.bento", env!("HOME"));
    let _ = std::fs::create_dir_all(config_path);
}
