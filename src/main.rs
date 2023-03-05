use sha2::{Sha256, Digest};


fn main() {
    let mut hasher = Sha256::new();
    let data = b"Hello world!";
    hasher.update(data);
    // `input` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update("String data");
    // Note that calling `finalize()` consumes hasher
    let hash = hasher.finalize();
    println!("Result: {:x}", hash);
}
