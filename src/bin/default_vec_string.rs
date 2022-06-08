use derive_builder::Builder;

#[derive(Builder, Debug)]
pub struct Config {
    pub address: String,
    #[builder(default = r#"vec!["123a".into()]"#)]
    pub nodes: Vec<String>
}



fn main() {
    println!("it works");
    let c = ConfigBuilder::default().address("a1".into()).build().unwrap();
    dbg!(c);

}


// [src/bin/default_vec_string.rs:15] c = Config {
//     address: "a1",
//     nodes: [
//         "123a",
//     ],
// }
