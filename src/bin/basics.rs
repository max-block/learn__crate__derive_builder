use derive_builder::Builder;

#[derive(Debug, Builder, Default)]
#[builder(setter(into))]
struct Client {
    #[builder(default = "\"sdf\".into()")]
    base_url: String,

    #[builder(default)]
    proxy: Option<String>,

    api_key: String,

    api_secret: String,
}

fn main() {
    let client = ClientBuilder::default()
        .api_key("k1")
        .api_secret("s1")
        .build()
        .unwrap();
    dbg!(client);
}
