use crate::hashes::traits::Hash;

pub struct MD5 {
}
impl Hash for MD5{
    fn get_name(&self) -> &str{
        return "md5"
    }
    fn hash(&self, string: String) -> String{
        use md5::{Md5, Digest};
        let mut hasher = Md5::new();
        let data = string.as_bytes();
        hasher.update(data);
        let hash = hasher.finalize();
        return base16ct::lower::encode_string(&hash);
    }
}