wit_bindgen_rust::export!("./domain.wit");

struct Domain;

impl domain::Domain for Domain {
    fn add(input: String) -> String {
        format!("{} there", input)
    }
}
