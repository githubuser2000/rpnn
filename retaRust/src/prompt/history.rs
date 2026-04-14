use std::path::PathBuf;

pub fn default_history_path(program_name: &str) -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut dir = home;
    dir.push(".local");
    dir.push("share");
    dir.push("reta");

    let _ = std::fs::create_dir_all(&dir);
    dir.push(format!("{}_history.txt", program_name));
    dir
}


pub fn default_log_path(program_name: &str) -> PathBuf {
    let home = std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let mut dir = home;
    dir.push(".local");
    dir.push("share");
    dir.push("reta");

    let _ = std::fs::create_dir_all(&dir);
    dir.push(format!("{}_session.log", program_name));
    dir
}
