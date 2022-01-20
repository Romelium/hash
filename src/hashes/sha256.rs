use crate::hashes::traits::Hash;

pub struct Sha256 {
}
impl Hash for Sha256{
    fn get_name(&self) -> &str{
        return "sha256"
    }
    fn hash(&self, string: String) -> String{
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        let data = string.as_bytes();
        hasher.update(data);
        let hash = hasher.finalize();
        return base16ct::lower::encode_string(&hash);
    }
}