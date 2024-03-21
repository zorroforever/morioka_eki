extern crate crypto;

use crypto::aes::{self, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::symmetriccipher::{Decryptor, Encryptor};
use rand::Rng;

pub async fn aes_encrypt(plaintext: &[u8], key: &[u8]) -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let combined_string = format!(
        "{}{}",
        std::str::from_utf8(plaintext).unwrap(),
        random_string
    ); // 字符串C

    let mut encryptor = aes::cbc_encryptor(
        KeySize::KeySize256,
        key,
        &[0; 16], // Initialization vector, here set to all zeros for simplicity
        PkcsPadding,
    );

    let mut ciphertext = Vec::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(combined_string.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor
            .encrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        ciphertext.extend(write_buffer.take_read_buffer().take_remaining().iter());

        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => {}
        }
    }

    let mut combined_ciphertext = ciphertext.clone();
    combined_ciphertext.extend_from_slice(random_string.as_bytes());
    hex::encode(combined_ciphertext)
}

pub async fn aes_decrypt(human_read: &str, key: &[u8]) -> String {
    let random_string_length = 8;
    let ciphertext = hex::decode(human_read).unwrap_or(Vec::from(""));
    if ciphertext.len() < random_string_length {
        panic!("Invalid ciphertext length!");
    }
    let ciphertext_without_b = &ciphertext[..ciphertext.len() - random_string_length];
    let mut decryptor = aes::cbc_decryptor(
        KeySize::KeySize256,
        key,
        &[0; 16], // Initialization vector, here set to all zeros for simplicity
        PkcsPadding,
    );
    let mut plaintext = Vec::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(ciphertext_without_b);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);
    loop {
        let result = decryptor
            .decrypt(&mut read_buffer, &mut write_buffer, true)
            .unwrap();
        plaintext.extend(write_buffer.take_read_buffer().take_remaining().iter());

        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            crypto::buffer::BufferResult::BufferOverflow => {}
        }
    }
    let plaintext_without_b = &plaintext[..plaintext.len() - random_string_length];
    std::str::from_utf8(&plaintext_without_b)
        .unwrap()
        .to_string()
}
#[tokio::test]
async fn test() {
    let plaintext = b"Hello, world!";
    let key = b"8ea8593bb2e44ccda1ccbb1fa07db5b6";
    let ciphertext = aes_encrypt(plaintext, key).await;
    println!("Encrypted text: {:?}", &ciphertext);
    let decrypted_text = aes_decrypt(&ciphertext, key).await;
    println!("Decrypted text: {}", decrypted_text);
}
