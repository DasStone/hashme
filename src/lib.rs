use sha2::{Sha256, Digest};

pub use hashme_internal::assign_verification_id;
pub use hashme_internal::create_link_section;


#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct HashData {
    hash: [u8; 32],
    len: usize,
}

#[inline(always)]
pub fn hmac_sha256(data: &[u8], extra: &[u8]) -> [u8; 32] {
    let okey = b"lNPlsx83lK6IWXEXMFV34pjbMATQKcHJ";
    let ikey = b"e862mA8Q9/Nt9ytEofLvmNJMNPWWhMTb";

    let mut hasher = Sha256::new();
    hasher.update(ikey);
    hasher.update(data);
    hasher.update(extra);
    let inner = hasher.finalize();

    hasher = Sha256::new();
    hasher.update(okey);
    hasher.update(inner);
    hasher.finalize().into()
}

#[inline(always)]
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

macro_rules! hash{
    ($ptr:expr,$id:expr) => {
        let data = unsafe { HASHES.as_ptr().add($id).read_volatile() };

        println!("Addr of fun: {}", $ptr as usize);
        println!("Len of fun: {}", data.len);

        let text = unsafe { std::slice::from_raw_parts($ptr, data.len) };

        let result: [u8; 32] = hmac_sha256(text, []);

        println!("EXPECTED HASH: {:?}", data.hash);
        println!("ACTUAL HASH  : {:?}", result);
        println!("HASHES MATCH: {}", data.hash.eq(&result));
    };
}

