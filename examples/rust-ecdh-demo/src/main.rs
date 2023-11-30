use anyhow::__private::kind::TraitKind;
use crypto::buffer::{ReadBuffer, WriteBuffer};
use ring::agreement::{self, EphemeralPrivateKey, PublicKey};
use ring::digest;
use ring::rand::SystemRandom;

fn main() -> anyhow::Result<()> {
    // https://docs.rs/ring/latest/ring/agreement/index.html
    // Generate ephemeral key pair for local party
    let rng = SystemRandom::new();
    let local_private_key =
        EphemeralPrivateKey::generate(&agreement::X25519, &rng)
            .unwrap();
    let local_public_key = local_private_key.compute_public_key().unwrap();

    let peer_public_key_bytes = {
        // In a real application, the peer public key would be parsed out of a
        // protocol message. Here we just generate one.
        let peer_private_key =
            agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng).map_err(|e|{
                anyhow::format_err!("EphemeralPrivateKey::generate failed: {}", e)
            })?;
        peer_private_key.compute_public_key().map_err(|e|{
            anyhow::format_err!("compute_public_key failed: {}", e)
        })?
    };

    let peer_public_key = agreement::UnparsedPublicKey::new(
        &agreement::X25519,
        peer_public_key_bytes);


    // Agree on a shared key
    let shared_key =
        agreement::agree_ephemeral(local_private_key, &peer_public_key, |_key_material| {
            // In a real application, we'd apply a KDF to the key material and the
            // public keys (as recommended in RFC 7748) and then derive session
            // keys from the result. We omit all that here.
            println!("_key_material: {:?}", &_key_material);
            _key_material.to_owned()
        })
        .unwrap();

    println!("shared_key bytes: {:?}", shared_key.len());
    println!("shared_key: {:?}", shared_key);

    // Encrypt/Decrypt using the shared key
    // For simplicity, you can use a crate like `crypto` for encryption
    // and `ecb` mode. https://github.com/DaGenix/rust-crypto
    // TODO add https://github.com/RustCrypto/block-ciphers/tree/master/aes example?

    let mut read_buffer = crypto::buffer::RefReadBuffer::new(b"hello world");
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
    let mut encryptor = crypto::aes::ecb_encryptor(crypto::aes::KeySize::KeySize256, &shared_key, crypto::blockmodes::PkcsPadding);
    let encrypted_data = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();


    let mut decryptor = crypto::aes::ecb_decryptor(crypto::aes::KeySize::KeySize256, &shared_key, crypto::blockmodes::PkcsPadding);
    let mut buffer2 = [0; 4096];
    let mut write_buffer2 = crypto::buffer::RefWriteBuffer::new(&mut buffer2);
    let decrypted_data = decryptor.decrypt(&mut write_buffer.take_read_buffer(), &mut write_buffer2, true).unwrap();

    let mut binding = write_buffer2.peek_read_buffer();
    let dec = binding.take_remaining();
    println!("decrypted_data: {:?}", std::str::from_utf8(dec).unwrap());
    // print  "hello world"

    Ok(())
}
