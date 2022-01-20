pub trait Hash {
    fn get_name(&self) -> &str;
    fn hash(&self, string: String) -> String;
}