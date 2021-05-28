/*
 * Copyright (c) 2020. Aberic - All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 * http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::cryptos::SM4;
use libsm::sm4::cipher_mode::CipherMode;
use libsm::sm4::Cipher;
use rand::rngs::OsRng;
use rand::RngCore;

pub trait SM4Handler {
    fn rand_block() -> [u8; 16];
}

pub trait SM4New1 {
    fn new() -> SM4;
}

pub trait SM4New2 {
    fn new(mode: CipherMode) -> SM4;
}

pub trait SM4New3 {
    fn new(key: [u8; 16], mode: CipherMode) -> SM4;
}

pub trait SM4New4 {
    fn new(key: [u8; 16], iv: [u8; 16], mode: CipherMode) -> SM4;
}

pub trait SM4SelfHandler {
    fn key(&self) -> [u8; 16];

    fn iv(&self) -> [u8; 16];
}

pub trait SM4Crypt {
    fn encrypt(key: [u8; 16], iv: [u8; 16], data: &[u8]) -> Vec<u8>;

    fn decrypt(key: [u8; 16], iv: [u8; 16], data: &[u8]) -> Vec<u8>;
}

pub trait SM4CryptMode {
    fn encrypt(key: [u8; 16], iv: [u8; 16], data: &[u8], mode: CipherMode) -> Vec<u8>;

    fn decrypt(key: [u8; 16], iv: [u8; 16], data: &[u8], mode: CipherMode) -> Vec<u8>;
}

pub trait SM4SelfCrypt1 {
    fn encrypt(&self, data: &[u8]) -> Vec<u8>;

    fn decrypt(&self, data: &[u8]) -> Vec<u8>;
}

pub trait SM4SelfCrypt2 {
    fn encrypt(&self, iv: &[u8; 16], data: &[u8]) -> Vec<u8>;

    fn decrypt(&self, iv: &[u8; 16], data: &[u8]) -> Vec<u8>;
}

impl SM4Handler for SM4 {
    fn rand_block() -> [u8; 16] {
        rand_block()
    }
}

impl SM4New1 for SM4 {
    fn new() -> SM4 {
        create_sm4(rand_block(), rand_block(), CipherMode::Cfb)
    }
}

impl SM4New2 for SM4 {
    fn new(mode: CipherMode) -> SM4 {
        create_sm4(rand_block(), rand_block(), mode)
    }
}

impl SM4New3 for SM4 {
    fn new(key: [u8; 16], mode: CipherMode) -> SM4 {
        create_sm4(key, rand_block(), mode)
    }
}

impl SM4New4 for SM4 {
    fn new(key: [u8; 16], iv: [u8; 16], mode: CipherMode) -> SM4 {
        create_sm4(key, iv, mode)
    }
}

impl SM4SelfHandler for SM4 {
    fn key(&self) -> [u8; 16] {
        self.key.clone()
    }

    fn iv(&self) -> [u8; 16] {
        self.iv.clone()
    }
}

impl SM4SelfCrypt1 for SM4 {
    fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        self.sm4_cipher_mode.encrypt(data, &self.iv)
    }

    fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        self.sm4_cipher_mode.decrypt(data, &self.iv)
    }
}

impl SM4SelfCrypt2 for SM4 {
    fn encrypt(&self, iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
        self.sm4_cipher_mode.encrypt(data, iv)
    }

    fn decrypt(&self, iv: &[u8; 16], data: &[u8]) -> Vec<u8> {
        self.sm4_cipher_mode.decrypt(data, iv)
    }
}

impl SM4Crypt for SM4 {
    fn encrypt(key: [u8; 16], iv: [u8; 16], data: &[u8]) -> Vec<u8> {
        create_sm4(key, iv, CipherMode::Cfb)
            .sm4_cipher_mode
            .encrypt(data, &iv)
    }

    fn decrypt(key: [u8; 16], iv: [u8; 16], data: &[u8]) -> Vec<u8> {
        create_sm4(key, iv, CipherMode::Cfb)
            .sm4_cipher_mode
            .decrypt(data, &iv)
    }
}

impl SM4CryptMode for SM4 {
    fn encrypt(key: [u8; 16], iv: [u8; 16], data: &[u8], mode: CipherMode) -> Vec<u8> {
        create_sm4(key, iv, mode).sm4_cipher_mode.encrypt(data, &iv)
    }

    fn decrypt(key: [u8; 16], iv: [u8; 16], data: &[u8], mode: CipherMode) -> Vec<u8> {
        create_sm4(key, iv, mode).sm4_cipher_mode.decrypt(data, &iv)
    }
}

fn create_sm4(key: [u8; 16], iv: [u8; 16], mode: CipherMode) -> SM4 {
    SM4 {
        key,
        iv,
        sm4_cipher_mode: Cipher::new(&key, mode),
    }
}

fn rand_block() -> [u8; 16] {
    let mut rng = OsRng::default();
    let mut block: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut block[..]);
    block
}
