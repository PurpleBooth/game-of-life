use std::env;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let default_app_name = current_dir.as_path().file_name().unwrap().to_str().unwrap();
    let app_name = env::var("REPOSITORY_NAME").unwrap_or_else(|_| default_app_name.to_string());
    println!("cargo:rustc-env=APP_NAME={}", app_name);
    println!(
        "cargo:rustc-env=AUTHOR_EMAIL=Billie Thompson <billie+{}@purplebooth.co.uk>",
        app_name
    );
    let version = env::var("VERSION").unwrap_or_else(|_| "dev".to_string());
    println!("cargo:rustc-env=VERSION={}", version);
}
