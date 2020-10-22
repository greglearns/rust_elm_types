#[macro_use]
extern crate elm_rusty;

#[macro_use]
extern crate serde_derive;

mod temp {
    pub struct User {}
}

#[derive(Elm)]
struct Foo {
    id: i32,
    name: String,
}

#[derive(Elm)]
#[elm(opts(rename = "ElmUser",))]
struct User<'a> {
    #[elm(rename = "foo")]
    name: Option<Vec<i32>>,
    id: &'a Vec<std::collections::HashMap<String, Vec<temp::User>>>,
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
