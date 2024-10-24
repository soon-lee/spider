use blake2::{Blake2s256, Digest};

pub struct HashUtil{
    hasher:Blake2s256
}
impl HashUtil {
    pub fn new() -> Self {
        HashUtil{
            hasher:Blake2s256::new()
        }
    }
    pub fn mut_to_hex64(&mut self, s:&str) ->String{
        self.hasher.update(s.as_bytes());
        format!("{:x}",self.hasher.finalize_reset())
    }
}