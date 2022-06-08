use derive_builder::Builder;

#[allow(dead_code)]
#[derive(Debug, Builder)]
#[builder(setter(into))]
struct Client {
    #[builder(default = "\"sdf\".into()")]
    base_url: String,

    #[builder(default, setter(strip_option))]
    proxy: Option<String>,

    api_key: String,

    api_secret: String,

    #[builder(setter(skip))]
    calculated_field: String,
}

fn main() {
    let client = ClientBuilder::default()
        .api_key("k1")
        .api_secret("s1")
        .proxy("proxy2")
        .build()
        .unwrap();
    dbg!(client);
}

// [src/bin/basics.rs:27] client = Client {
//     base_url: "sdf",
//     proxy: Some(
//         "proxy2",
//     ),
//     api_key: "k1",
//     api_secret: "s1",
//     calculated_field: "",
// }
