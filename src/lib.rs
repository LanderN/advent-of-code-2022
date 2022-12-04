use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "inputs"]
struct Asset;

pub fn get_input(path: &str) -> String {
    std::str::from_utf8(Asset::get(path).unwrap().data.as_ref())
        .unwrap()
        .to_string()
}
