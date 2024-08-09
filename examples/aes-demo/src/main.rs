use aes::cipher::{
    block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyInit,
};

use generic_array::GenericArray;
use hex_literal::hex as hexdecode;

// https://github.com/RustCrypto/block-modes do NOT have ecb anymore
// https://crates.io/crates/ecb
// https://docs.rs/ecb/latest/ecb/
type Aes128CbcEnc = ecb::Encryptor<aes::Aes128>;
type Aes128CbcDec = ecb::Decryptor<aes::Aes128>;

fn main() {
    // let key = "1234567890abcdef";
    let key = GenericArray::from_slice(b"1234567890abcdef");
    let plaintext = *b"hello world! this is my plaintext.";
    let ciphertext = hexdecode!(
        "52e47e367c5de0c3d6f22ae852461998429d768c476c20aa151a043638fd2690e7b712b0e9d864e209126fe912f82b96"
    );

    // encrypt/decrypt in-place
    // buffer must be big enough for padded plaintext
    let mut buf = [0u8; 48];
    let pt_len = plaintext.len();
    buf[..pt_len].copy_from_slice(&plaintext);
    let ct = Aes128CbcEnc::new(key)
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
        .unwrap();

    println!("{}", hex::encode(ct));
    assert_eq!(ct, &ciphertext[..]);

    let pt = Aes128CbcDec::new(key)
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .unwrap();
    assert_eq!(pt, &plaintext);
}
