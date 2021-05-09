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
use crate::errors::entrances::{err_str, err_strs};
use crate::io::file::{Filer, FilerWriter};
use std::path::Path;

pub struct SM2;

pub trait SM2New {
    fn generate() -> (Vec<u8>, Vec<u8>);
    fn generate_string() -> (String, String);
}

pub trait SM2NewStore<T> {
    fn generate(sk_filepath: T, pk_filepath: T) -> GeorgeResult<(Vec<u8>, Vec<u8>)>;
    fn generate_string(sk_filepath: T, pk_filepath: T) -> GeorgeResult<(String, String)>;
}

pub trait SM2PKV8s<T> {
    fn generate_pk(sk: T) -> GeorgeResult<Vec<u8>>;
}

pub trait SM2PKString<T> {
    fn generate_pk(sk: T) -> GeorgeResult<String>;
}

pub trait SM2PKV8sPath {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>>;
}

pub trait SM2PKStringPath {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String>;
}

pub trait SM2Sign<M, N> {
    fn sign(msg: M, sk: N, pk: N) -> Vec<u8>;
    fn sign_string(msg: M, sk: N, pk: N) -> String;
}

pub trait SM2SignString<T> {
    fn sign(msg: T, sk: String, pk: String) -> GeorgeResult<Vec<u8>>;
    fn sign_string(msg: T, sk: String, pk: String) -> GeorgeResult<String>;
}

pub trait SM2SignPath<T> {
    fn sign<P: AsRef<Path>>(msg: T, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>>;
    fn sign_string<P: AsRef<Path>>(msg: T, sk_filepath: P, pk_filepath: P) -> GeorgeResult<String>;
}

impl SM2New for SM2 {
    fn generate() -> (Vec<u8>, Vec<u8>) {
        generate()
    }

    fn generate_string() -> (String, String) {
        generate_hex()
    }
}

impl SM2NewStore<String> for SM2 {
    fn generate(sk_filepath: String, pk_filepath: String) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
        generate_in_file(sk_filepath, pk_filepath)
    }

    fn generate_string(sk_filepath: String, pk_filepath: String) -> GeorgeResult<(String, String)> {
        generate_hex_in_file(sk_filepath, pk_filepath)
    }
}

impl SM2NewStore<&str> for SM2 {
    fn generate(sk_filepath: &str, pk_filepath: &str) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
        generate_in_file(sk_filepath.to_string(), pk_filepath.to_string())
    }

    fn generate_string(sk_filepath: &str, pk_filepath: &str) -> GeorgeResult<(String, String)> {
        generate_hex_in_file(sk_filepath.to_string(), pk_filepath.to_string())
    }
}

impl SM2PKV8s<Vec<u8>> for SM2 {
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk(sk)
    }
}

impl SM2PKV8s<String> for SM2 {
    fn generate_pk(sk: String) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_str(sk)
    }
}

impl SM2PKString<Vec<u8>> for SM2 {
    fn generate_pk(sk: Vec<u8>) -> GeorgeResult<String> {
        Ok(hex::encode(generate_pk_from_sk(sk)?))
    }
}

impl SM2PKString<String> for SM2 {
    fn generate_pk(sk: String) -> GeorgeResult<String> {
        Ok(hex::encode(generate_pk_from_sk_str(sk)?))
    }
}

impl SM2PKV8sPath for SM2 {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
        generate_pk_from_sk_file(sk_filepath)
    }
}

impl SM2PKStringPath for SM2 {
    fn generate_pk<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<String> {
        Ok(hex::encode(generate_pk_from_sk_file(sk_filepath)?))
    }
}

impl SM2Sign<&[u8], &[u8]> for SM2 {
    fn sign(msg: &[u8], sk: &[u8], pk: &[u8]) -> Vec<u8> {
        sign(msg, sk, pk)
    }

    fn sign_string(msg: &[u8], sk: &[u8], pk: &[u8]) -> String {
        hex::encode(sign(msg, sk, pk))
    }
}

impl SM2Sign<&[u8], Vec<u8>> for SM2 {
    fn sign(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> Vec<u8> {
        sign(msg, sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: &[u8], sk: Vec<u8>, pk: Vec<u8>) -> String {
        hex::encode(sign(msg, sk.as_slice(), pk.as_slice()))
    }
}

impl SM2Sign<Vec<u8>, Vec<u8>> for SM2 {
    fn sign(msg: Vec<u8>, sk: Vec<u8>, pk: Vec<u8>) -> Vec<u8> {
        sign(msg.as_slice(), sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: Vec<u8>, sk: Vec<u8>, pk: Vec<u8>) -> String {
        hex::encode(sign(msg.as_slice(), sk.as_slice(), pk.as_slice()))
    }
}

impl SM2Sign<String, Vec<u8>> for SM2 {
    fn sign(msg: String, sk: Vec<u8>, pk: Vec<u8>) -> Vec<u8> {
        sign(msg.as_bytes(), sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: String, sk: Vec<u8>, pk: Vec<u8>) -> String {
        hex::encode(sign(msg.as_bytes(), sk.as_slice(), pk.as_slice()))
    }
}

impl SM2Sign<&str, Vec<u8>> for SM2 {
    fn sign(msg: &str, sk: Vec<u8>, pk: Vec<u8>) -> Vec<u8> {
        sign(msg.as_bytes(), sk.as_slice(), pk.as_slice())
    }

    fn sign_string(msg: &str, sk: Vec<u8>, pk: Vec<u8>) -> String {
        hex::encode(sign(msg.as_bytes(), sk.as_slice(), pk.as_slice()))
    }
}

impl SM2Sign<Vec<u8>, &[u8]> for SM2 {
    fn sign(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> Vec<u8> {
        sign(msg.as_slice(), sk, pk)
    }

    fn sign_string(msg: Vec<u8>, sk: &[u8], pk: &[u8]) -> String {
        hex::encode(sign(msg.as_slice(), sk, pk))
    }
}

impl SM2Sign<String, &[u8]> for SM2 {
    fn sign(msg: String, sk: &[u8], pk: &[u8]) -> Vec<u8> {
        sign(msg.as_bytes(), sk, pk)
    }

    fn sign_string(msg: String, sk: &[u8], pk: &[u8]) -> String {
        hex::encode(sign(msg.as_bytes(), sk, pk))
    }
}

impl SM2Sign<&str, &[u8]> for SM2 {
    fn sign(msg: &str, sk: &[u8], pk: &[u8]) -> Vec<u8> {
        sign(msg.as_bytes(), sk, pk)
    }

    fn sign_string(msg: &str, sk: &[u8], pk: &[u8]) -> String {
        hex::encode(sign(msg.as_bytes(), sk, pk))
    }
}

impl SM2SignString<&[u8]> for SM2 {
    fn sign(msg: &[u8], sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        Ok(sign(
            msg,
            &key2bytes(sk)?.as_slice(),
            &key2bytes(pk)?.as_slice(),
        ))
    }

    fn sign_string(msg: &[u8], sk: String, pk: String) -> GeorgeResult<String> {
        Ok(hex::encode(sign(
            msg,
            key2bytes(sk)?.as_slice(),
            key2bytes(pk)?.as_slice(),
        )))
    }
}

impl SM2SignString<Vec<u8>> for SM2 {
    fn sign(msg: Vec<u8>, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        Ok(sign(
            msg.as_slice(),
            &key2bytes(sk)?.as_slice(),
            &key2bytes(pk)?.as_slice(),
        ))
    }

    fn sign_string(msg: Vec<u8>, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(hex::encode(sign(
            msg.as_slice(),
            key2bytes(sk)?.as_slice(),
            key2bytes(pk)?.as_slice(),
        )))
    }
}

impl SM2SignString<String> for SM2 {
    fn sign(msg: String, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        Ok(sign(
            msg.as_bytes(),
            &key2bytes(sk)?.as_slice(),
            &key2bytes(pk)?.as_slice(),
        ))
    }

    fn sign_string(msg: String, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(hex::encode(sign(
            msg.as_bytes(),
            key2bytes(sk)?.as_slice(),
            key2bytes(pk)?.as_slice(),
        )))
    }
}

impl SM2SignString<&str> for SM2 {
    fn sign(msg: &str, sk: String, pk: String) -> GeorgeResult<Vec<u8>> {
        Ok(sign(
            msg.as_bytes(),
            &key2bytes(sk)?.as_slice(),
            &key2bytes(pk)?.as_slice(),
        ))
    }

    fn sign_string(msg: &str, sk: String, pk: String) -> GeorgeResult<String> {
        Ok(hex::encode(sign(
            msg.as_bytes(),
            key2bytes(sk)?.as_slice(),
            key2bytes(pk)?.as_slice(),
        )))
    }
}

impl SM2SignPath<&[u8]> for SM2 {
    fn sign<P: AsRef<Path>>(msg: &[u8], sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        Ok(sign(
            msg,
            key_string_from_file(sk_filepath)?.as_bytes(),
            key_string_from_file(pk_filepath)?.as_bytes(),
        ))
    }

    fn sign_string<P: AsRef<Path>>(
        msg: &[u8],
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(hex::encode(sign(
            msg,
            key_string_from_file(sk_filepath)?.as_bytes(),
            key_string_from_file(pk_filepath)?.as_bytes(),
        )))
    }
}

impl SM2SignPath<Vec<u8>> for SM2 {
    fn sign<P: AsRef<Path>>(msg: Vec<u8>, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        Ok(sign(
            msg.as_slice(),
            key_string_from_file(sk_filepath)?.as_bytes(),
            key_string_from_file(pk_filepath)?.as_bytes(),
        ))
    }

    fn sign_string<P: AsRef<Path>>(
        msg: Vec<u8>,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(hex::encode(sign(
            msg.as_slice(),
            key_string_from_file(sk_filepath)?.as_bytes(),
            key_string_from_file(pk_filepath)?.as_bytes(),
        )))
    }
}

impl SM2SignPath<String> for SM2 {
    fn sign<P: AsRef<Path>>(msg: String, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        Ok(sign(
            msg.as_bytes(),
            key_string_from_file(sk_filepath)?.as_bytes(),
            key_string_from_file(pk_filepath)?.as_bytes(),
        ))
    }

    fn sign_string<P: AsRef<Path>>(
        msg: String,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(hex::encode(sign(
            msg.as_bytes(),
            key_string_from_file(sk_filepath)?.as_bytes(),
            key_string_from_file(pk_filepath)?.as_bytes(),
        )))
    }
}

impl SM2SignPath<&str> for SM2 {
    fn sign<P: AsRef<Path>>(msg: &str, sk_filepath: P, pk_filepath: P) -> GeorgeResult<Vec<u8>> {
        Ok(sign(
            msg.as_bytes(),
            key_string_from_file(sk_filepath)?.as_bytes(),
            key_string_from_file(pk_filepath)?.as_bytes(),
        ))
    }

    fn sign_string<P: AsRef<Path>>(
        msg: &str,
        sk_filepath: P,
        pk_filepath: P,
    ) -> GeorgeResult<String> {
        Ok(hex::encode(sign(
            msg.as_bytes(),
            key_string_from_file(sk_filepath)?.as_bytes(),
            key_string_from_file(pk_filepath)?.as_bytes(),
        )))
    }
}

fn key_string_from_file<P: AsRef<Path>>(key_filepath: P) -> GeorgeResult<String> {
    match read_to_string(key_filepath) {
        Ok(sk_string) => Ok(sk_string),
        Err(err) => Err(err_strs("read_to_string", err)),
    }
}

fn key_bytes_from_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
    let key_string = key_string_from_file(sk_filepath)?;
    match hex::decode(key_string) {
        Ok(sk_bytes) => Ok(sk_bytes),
        Err(err) => Err(err_strs("hex decode", err)),
    }
}

fn generate() -> (Vec<u8>, Vec<u8>) {
    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();
    (ctx.serialize_seckey(&sk), ctx.serialize_pubkey(&pk, true))
}

fn generate_hex() -> (String, String) {
    let (sk, pk) = generate();
    (hex::encode(sk), hex::encode(pk))
}

fn generate_pk_from_sk(sk: Vec<u8>) -> GeorgeResult<Vec<u8>> {
    let ctx = SigCtx::new();
    match ctx.load_seckey(sk.as_slice()) {
        Ok(p) => Ok(ctx.serialize_pubkey(&ctx.pk_from_sk(&p), true)),
        Err(()) => Err(err_str("unknown")),
    }
}

fn generate_pk_from_sk_str(sk: String) -> GeorgeResult<Vec<u8>> {
    match hex::decode(sk) {
        Ok(sk_bytes) => generate_pk_from_sk(sk_bytes),
        Err(err) => Err(err_strs("hex decode", err)),
    }
}

fn generate_pk_from_sk_file<P: AsRef<Path>>(sk_filepath: P) -> GeorgeResult<Vec<u8>> {
    match read_to_string(sk_filepath) {
        Ok(sk) => generate_pk_from_sk_str(sk),
        Err(err) => Err(err_strs("read_to_string", err)),
    }
}

fn generate_in_file(sk_filepath: String, pk_filepath: String) -> GeorgeResult<(Vec<u8>, Vec<u8>)> {
    let (sk_bytes, pk_bytes) = generate();
    match Filer::write_force(sk_filepath, hex::encode(sk_bytes.clone()).into_bytes()) {
        Err(err) => return Err(err_strs("sk write_bytes", err)),
        _ => {}
    }
    match Filer::write_force(pk_filepath, hex::encode(pk_bytes.clone()).into_bytes()) {
        Err(err) => return Err(err_strs("pk write_bytes", err)),
        _ => {}
    }
    Ok((sk_bytes, pk_bytes))
}

fn generate_hex_in_file(
    sk_filepath: String,
    pk_filepath: String,
) -> GeorgeResult<(String, String)> {
    let (sk_str, pk_str) = generate_hex();
    match Filer::write_force(sk_filepath, sk_str.clone().into_bytes()) {
        Err(err) => return Err(err_strs("sk write_bytes", err)),
        _ => {}
    }
    match Filer::write_force(pk_filepath, pk_str.clone().into_bytes()) {
        Err(err) => return Err(err_strs("pk write_bytes", err)),
        _ => {}
    }
    Ok((sk_str, pk_str))
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

fn key2bytes(key: String) -> GeorgeResult<Vec<u8>> {
    match hex::decode(key) {
        Ok(res) => Ok(res),
        Err(err) => Err(err_strs("hex decode", err)),
    }
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
