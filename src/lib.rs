#[macro_use]
extern crate rust_elm_types_derive;

#[macro_use]
extern crate serde_derive;

mod temp;

#[derive(Elm, Serialize)]
#[elm(opts(
    rename = "ElmUser",
    path = "/Users/abrarkhan/Documents/github/rust_elm_types"
))]
struct User<'a> {
    #[elm(rename = "elm_rename")]
    name: Option<Vec<i32>>,
    id: &'a Vec<std::collections::HashMap<String, Vec<String>>>,
    vector: Vec<i32>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_me() {
        println!("asfdasf");
        super::test_check();
    }
}
