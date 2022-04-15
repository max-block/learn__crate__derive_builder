use derive_builder::Builder;

#[derive(Debug, Builder, Default)]
struct Client {
    #[builder(setter(into), default = "\"sdf\".into()")]
    base_url: String,

    #[builder(setter(into), default)]
    proxy: Option<String>,

    #[builder(setter(into))]
    api_key: String,

    #[builder(setter(into))]
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
