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

use std::fs::read_to_string;

use libsm::sm2::signature::{SigCtx, Signature};

use crate::errors::entrances::GeorgeResult;
use crate::errors::entrances::{err_str, err_str_enhance};
use crate::io::writer::write_bytes;

pub fn generate() -> (Vec<u8>, Vec<u8>) {
    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();
    (ctx.serialize_seckey(&sk), ctx.serialize_pubkey(&pk, true))
}

pub fn generate_hex() -> (String, String) {
    let (sk, pk) = generate();
    (hex::encode(sk), hex::encode(pk))
}

pub fn generate_pk_from_sk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    let ctx = SigCtx::new();
    match ctx.load_seckey(sk.as_slice()) {
        Ok(p) => Ok(ctx.serialize_pubkey(&ctx.pk_from_sk(&p), true)),
        Err(()) => Err(err_str("unknown")),
    }
}

pub fn generate_pk_from_sk_str(sk: String) -> GeorgeResult<Vec<u8>> {
    match hex::decode(sk) {
        Ok(sk_bytes) => generate_pk_from_sk(sk_bytes),
        Err(err) => Err(err_str_enhance("hex decode", err.to_string())),
    }
}

pub fn generate_pk_from_sk_file(sk_filepath: String) -> GeorgeResult<Vec<u8>> {
    match read_to_string(sk_filepath) {
        Ok(sk) => generate_pk_from_sk_str(sk),
        Err(err) => Err(err_str_enhance("read_to_string", err.to_string())),
    }
}

pub fn generate_in_file(
    sk_filepath: String,
    pk_filepath: String,
) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
    let (sk_bytes, pk_bytes) = generate();
    match write_bytes(
        sk_filepath,
        hex::encode(sk_bytes.clone()).into_bytes(),
        true,
    ) {
        Err(err) => return Err(err_str_enhance("sk write_bytes", err.to_string())),
        _ => {}
    }
    match write_bytes(
        pk_filepath,
        hex::encode(pk_bytes.clone()).into_bytes(),
        true,
    ) {
        Err(err) => return Err(err_str_enhance("pk write_bytes", err.to_string())),
        _ => {}
    }
    Ok((sk_bytes, pk_bytes))
}

pub fn generate_in_files(sk_filepath: &str, pk_filepath: &str) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
    generate_in_file(sk_filepath.to_string(), pk_filepath.to_string())
}

pub fn generate_hex_in_file(
    sk_filepath: String,
    pk_filepath: String,
) -> GeorgeResult<(String, String)> {
    let (sk_str, pk_str) = generate_hex();
    match write_bytes(sk_filepath, sk_str.clone().into_bytes(), true) {
        Err(err) => return Err(err_str_enhance("sk write_bytes", err.to_string())),
        _ => {}
    }
    match write_bytes(pk_filepath, pk_str.clone().into_bytes(), true) {
        Err(err) => return Err(err_str_enhance("pk write_bytes", err.to_string())),
        _ => {}
    }
    Ok((sk_str, pk_str))
}

pub fn generate_hex_in_files(
    sk_filepath: &str,
    pk_filepath: &str,
) -> GeorgeResult<(String, String)> {
    generate_hex_in_file(sk_filepath.to_string(), pk_filepath.to_string())
}

pub fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> Vec<u8> {
    let ctx = SigCtx::new();
    let signature = ctx.sign(
        msg,
        &ctx.load_seckey(sk).unwrap(),
        &ctx.load_pubkey(pk).unwrap(),
    );
    signature.der_encode()
}

pub fn sign_string(msg: String, sk: String, pk: String) -> Vec<u8> {
    let mut sk_bytes = vec![];
    let mut pk_bytes = vec![];
    match hex::decode(sk) {
        Ok(res) => sk_bytes = res,
        Err(err) => {
            println!("hex decode error: {}", err.to_string());
            return vec![];
        }
    }
    match hex::decode(pk) {
        Ok(res) => pk_bytes = res,
        Err(err) => {
            println!("hex decode error: {}", err.to_string());
            return vec![];
        }
    }
    sign(msg.as_bytes(), sk_bytes.as_slice(), pk_bytes.as_slice())
}

pub fn sign_to_string(msg: &[u8], sk: &[u8], pk: &[u8]) -> String {
    hex::encode(sign(msg, sk, pk))
}

pub fn sign_str(msg: &str, sk: &str, pk: &str) -> Vec<u8> {
    sign_string(msg.to_string(), sk.to_string(), pk.to_string())
}

pub fn sign_string_to_string(msg: String, sk: String, pk: String) -> String {
    hex::encode(sign_string(msg, sk, pk))
}

pub fn sign_str_to_string(msg: &str, sk: &str, pk: &str) -> String {
    hex::encode(sign_string(msg.to_string(), sk.to_string(), pk.to_string()))
}

pub fn verify(msg: &[u8], pk: &[u8], der: &[u8]) -> bool {
    let ctx = SigCtx::new();
    ctx.verify(
        msg,
        &ctx.load_pubkey(pk).unwrap(),
        &Signature::der_decode(der).unwrap(),
    )
}

pub fn verify_from_string(msg: &[u8], pk: &[u8], der: String) -> bool {
    match hex::decode(der) {
        Ok(res) => verify(msg, pk, res.as_slice()),
        Err(err) => {
            println!("hex decode error: {}", err.to_string());
            false
        }
    }
}

pub fn verify_from_str(msg: &[u8], pk: &[u8], der: &str) -> bool {
    match hex::decode(der) {
        Ok(res) => verify(msg, pk, res.as_slice()),
        Err(err) => {
            println!("hex decode error: {}", err.to_string());
            false
        }
    }
}

pub fn verify_string(msg: String, pk: String, der: &[u8]) -> bool {
    match hex::decode(pk) {
        Ok(res) => verify(msg.as_bytes(), res.as_slice(), der),
        Err(err) => {
            println!("hex decode error: {}", err.to_string());
            false
        }
    }
}

pub fn verify_string_from_string(msg: String, pk: String, der: String) -> bool {
    match hex::decode(der) {
        Ok(res) => verify_string(msg, pk, res.as_slice()),
        Err(err) => {
            println!("hex decode error: {}", err.to_string());
            false
        }
    }
}

pub fn verify_str_from_string(msg: &str, pk: &str, der: String) -> bool {
    verify_string_from_string(msg.to_string(), pk.to_string(), der)
}

pub fn verify_str_from_str(msg: &str, pk: &str, der: &str) -> bool {
    verify_string_from_string(msg.to_string(), pk.to_string(), der.to_string())
}

pub fn verify_string_from_str(msg: String, pk: String, der: &str) -> bool {
    verify_string_from_string(msg, pk, der.to_string())
}
