pub fn get_version(product: &str) -> String {
    format!(
        "inferno {}.{}.{}",
        if cfg!(debug_assertions) {
            format!("dev-{}", &env!("GIT_HASH")[0..9])
        } else {
            env!("CARGO_PKG_VERSION").to_string()
        },
        product,
        &env!("GIT_DATE")[0..10]
    )
}

