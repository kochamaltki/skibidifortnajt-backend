use std::fs;

pub fn get_secret() -> String{
    let contents = fs::read_to_string("./SECRET")
        .expect("Should have been able to read the file");
    contents
}
