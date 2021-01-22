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

use std::fs::read;

use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, Private};
use openssl::x509::extension::{
    AuthorityKeyIdentifier, BasicConstraints, KeyUsage, SubjectAlternativeName,
    SubjectKeyIdentifier,
};
use openssl::x509::{X509Name, X509NameBuilder, X509NameRef, X509Req, X509ReqBuilder, X509};

use crate::cryptos::rsa;
use crate::errors::entrances::err_strs;
use crate::errors::entrances::GeorgeResult;

pub struct X509NameInfo {
    /// ISO国家代码（两位字符），如CN
    pub country: String,
    /// 公司名称，如George Technology Inc
    pub organization: String,
    /// 部门名称	sales Dep
    pub organizational_unit: String,
    /// 所在城市，如Tianjin
    pub locality: String,
    /// 所在省份，如Tianjin
    pub province: String,
    /// 公用名(Common Name)是主机名+域名，比如：www.domain.net<p>
    ///
    /// 数字证书的服务器证书是颁发给某一台主机的，而不是一个域
    ///
    /// 公用名（Common Name）必须与要使用服务器证书的主机的域名完全相同，因为www.domain.com与domain.com是不同的
    pub common_name: String,
}

impl X509NameInfo {
    pub fn build(&self) -> X509Name {
        let mut x509_name = X509NameBuilder::new().unwrap();
        if !String::is_empty(&self.country) {
            x509_name
                .append_entry_by_nid(Nid::COUNTRYNAME, self.country.as_str())
                .unwrap()
        }
        if !String::is_empty(&self.organization) {
            x509_name
                .append_entry_by_nid(Nid::ORGANIZATIONNAME, self.organization.as_str())
                .unwrap()
        }
        if !String::is_empty(&self.organizational_unit) {
            x509_name
                .append_entry_by_nid(
                    Nid::ORGANIZATIONALUNITNAME,
                    self.organizational_unit.as_str(),
                )
                .unwrap()
        }
        if !String::is_empty(&self.locality) {
            x509_name
                .append_entry_by_nid(Nid::LOCALITYNAME, self.locality.as_str())
                .unwrap()
        }
        if !String::is_empty(&self.province) {
            x509_name
                .append_entry_by_nid(Nid::STATEORPROVINCENAME, self.province.as_str())
                .unwrap()
        }
        if !String::is_empty(&self.common_name) {
            x509_name
                .append_entry_by_nid(Nid::COMMONNAME, self.common_name.as_str())
                .unwrap()
        }
        x509_name.build()
    }
}

pub fn create_cert_request(sk: &PKey<Private>, info: X509NameInfo) -> GeorgeResult<X509Req> {
    match X509ReqBuilder::new() {
        Ok(mut req_builder) => match req_builder.set_pubkey(&sk) {
            Ok(()) => match req_builder.set_subject_name(&info.build()) {
                Ok(()) => match req_builder.sign(&sk, MessageDigest::sha256()) {
                    Ok(()) => Ok(req_builder.build()),
                    Err(err) => Err(err_strs("sign", err)),
                },
                Err(err) => Err(err_strs("set_subject_name", err)),
            },
            Err(err) => Err(err_strs("set_pubkey", err)),
        },
        Err(err) => Err(err_strs("X509ReqBuilder_new", err)),
    }
}

/// 创建证书
///
/// bits 生成一个bits位奇数随机数
///
/// sk 与证书关联的公钥
///
/// subject_info 证书的主题名称。在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
///
/// issuer_info 证书的发布者名称
///
/// version 设置证书的版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
fn create_cert(
    bits: i32,
    sk: &PKey<Private>,
    issuer_info: X509NameInfo,
    version: i32,
    not_before_day: u32,
    not_after_day: u32,
) -> Result<X509, ErrorStack> {
    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(version)?;
    let serial_number = {
        let mut serial = BigNum::new().unwrap();
        serial.rand(bits, MsbOption::MAYBE_ZERO, false).unwrap();
        serial.to_asn1_integer().unwrap()
    };
    // 序列号
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(&issuer_info.build())?;
    cert_builder.set_issuer_name(&issuer_info.build())?;
    cert_builder.set_pubkey(&sk)?;
    let not_before = Asn1Time::days_from_now(not_before_day)?;
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = Asn1Time::days_from_now(not_after_day)?;
    cert_builder.set_not_after(&not_after)?;

    cert_builder.append_extension(BasicConstraints::new().critical().ca().build().unwrap())?;
    cert_builder.append_extension(
        KeyUsage::new()
            .critical()
            .key_cert_sign()
            .crl_sign()
            .build()
            .unwrap(),
    )?;
    // 主题唯一标识符（可选）。如果证书是自签名的，则将“发布者”设置为“None”。
    let subject_key_identifier =
        SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;
    cert_builder.append_extension(subject_key_identifier)?;
    cert_builder.sign(&sk, MessageDigest::sha256())?;
    Ok(cert_builder.build())
}

/// 创建根证书
///
/// bits 生成一个bits位奇数随机数
///
/// sk 与证书关联的私钥
///
/// subject_info 证书的主题名称。在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
///
/// issuer_info 证书的发布者名称
///
/// version 设置证书的版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
pub fn create(
    bits: i32,
    sk: &PKey<Private>,
    issuer_info: X509NameInfo,
    version: i32,
    not_before_day: u32,
    not_after_day: u32,
) -> GeorgeResult<X509> {
    match create_cert(
        bits,
        sk,
        issuer_info,
        version,
        not_before_day,
        not_after_day,
    ) {
        Ok(x509) => Ok(x509),
        Err(err) => Err(err_strs("create_cert", err)),
    }
}

pub struct AltName {
    /// DNSNames DNS限制
    pub dns_names: Vec<String>,
    /// EmailAddresses 邮箱地址限制
    pub email_addresses: Vec<String>,
    /// IPAddresses IP地址限制
    pub ip_addresses: Vec<String>,
    /// URIs URL地址限制
    pub uris: Vec<String>,
}

impl AltName {
    pub fn build(&self) -> SubjectAlternativeName {
        let mut subject_alt_name = SubjectAlternativeName::new();
        for dns_name in &self.dns_names {
            subject_alt_name.dns(dns_name.as_str());
        }
        for email_address in &self.email_addresses {
            subject_alt_name.email(email_address.as_str());
        }
        for ip_address in &self.ip_addresses {
            subject_alt_name.ip(ip_address.as_str());
        }
        for uri in &self.uris {
            subject_alt_name.uri(uri.as_str());
        }
        subject_alt_name
    }
}

/// 签发子证书
///
/// ca_cert 根证书
///
/// ca_sk 自签根证书的私钥
///
/// bits 生成一个bits位奇数随机数
///
/// sk 与证书关联的私钥
///
/// subject_info 证书的主题名称。在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
///
/// issuer_info 证书的发布者名称
///
/// version 设置证书的版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
fn sign_cert(
    ca_cert: X509,
    ca_sk: &PKey<Private>,
    bits: i32,
    sk: &PKey<Private>,
    subject_name: &X509NameRef,
    alt_name: AltName,
    version: i32,
    not_before_day: u32,
    not_after_day: u32,
) -> Result<X509, ErrorStack> {
    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(version)?;
    // 序列号
    let serial_number = {
        let mut serial = BigNum::new().unwrap();
        serial.rand(bits, MsbOption::MAYBE_ZERO, false).unwrap();
        serial.to_asn1_integer().unwrap()
    };
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(subject_name)?;
    cert_builder.set_issuer_name(ca_cert.subject_name())?;
    cert_builder.set_pubkey(&sk)?;
    let not_before = Asn1Time::days_from_now(not_before_day)?;
    cert_builder.set_not_before(&not_before).unwrap();
    let not_after = Asn1Time::days_from_now(not_after_day)?;
    cert_builder.set_not_after(&not_after)?;

    cert_builder.append_extension(BasicConstraints::new().critical().ca().build().unwrap())?;
    cert_builder.append_extension(
        KeyUsage::new()
            .critical()
            .key_cert_sign()
            .crl_sign()
            .build()
            .unwrap(),
    )?;
    // 主题唯一标识符（可选）。如果证书是自签名的，则将“发布者”设置为“None”。
    let subject_key_identifier = SubjectKeyIdentifier::new()
        .build(&cert_builder.x509v3_context(Some(ca_cert.as_ref()), None))?;
    cert_builder.append_extension(subject_key_identifier)?;

    // 颁发者唯一标识符（可选）
    let auth_key_identifier = AuthorityKeyIdentifier::new()
        .keyid(false)
        .issuer(false)
        .build(&cert_builder.x509v3_context(Some(ca_cert.as_ref()), None))?;
    cert_builder.append_extension(auth_key_identifier)?;

    let subject_alt_name = alt_name
        .build()
        .build(&cert_builder.x509v3_context(Some(ca_cert.as_ref()), None))?;
    cert_builder.append_extension(subject_alt_name)?;

    cert_builder.sign(&ca_sk, MessageDigest::sha256())?;
    Ok(cert_builder.build())
}

/// 签发子证书
///
/// ca_cert 根证书
///
/// ca_sk 自签根证书的私钥
///
/// bits 生成一个bits位奇数随机数
///
/// sk 与证书关联的私钥
///
/// subject_info 证书的主题名称。在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
///
/// issuer_info 证书的发布者名称
///
/// version 设置证书的版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
pub fn sign_csr(
    ca_cert: X509,
    ca_sk: &PKey<Private>,
    bits: i32,
    sk: &PKey<Private>,
    csr: X509Req,
    alt_name: AltName,
    version: i32,
    not_before_day: u32,
    not_after_day: u32,
) -> GeorgeResult<X509> {
    match sign_cert(
        ca_cert,
        ca_sk,
        bits,
        sk,
        csr.subject_name(),
        alt_name,
        version,
        not_before_day,
        not_after_day,
    ) {
        Ok(x509) => Ok(x509),
        Err(err) => Err(err_strs("sign_cert", err)),
    }
}

/// 签发子证书
///
/// ca_cert 根证书
///
/// ca_sk 自签根证书的私钥
///
/// bits 生成一个bits位奇数随机数
///
/// sk 与证书关联的私钥
///
/// subject_info 证书的主题名称。在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
///
/// issuer_info 证书的发布者名称
///
/// version 设置证书的版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
pub fn sign_obj(
    ca_cert: X509,
    ca_sk: &PKey<Private>,
    bits: i32,
    sk: &PKey<Private>,
    subject_info: X509NameInfo,
    alt_name: AltName,
    version: i32,
    not_before_day: u32,
    not_after_day: u32,
) -> GeorgeResult<X509> {
    match sign_cert(
        ca_cert,
        ca_sk,
        bits,
        sk,
        subject_info.build().as_ref(),
        alt_name,
        version,
        not_before_day,
        not_after_day,
    ) {
        Ok(x509) => Ok(x509),
        Err(err) => Err(err_strs("sign_cert", err)),
    }
}

/// 签发子证书
///
/// ca_cert 根证书
///
/// ca_sk 自签根证书的私钥
///
/// bits 生成一个bits位奇数随机数
///
/// sk 与证书关联的私钥
///
/// subject_info 证书的主题名称。在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
///
/// issuer_info 证书的发布者名称
///
/// version 设置证书的版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
pub fn sign(
    ca_cert_path: String,
    ca_sk_path: String,
    bits: i32,
    sk_path: String,
    subject_info: X509NameInfo,
    alt_name: AltName,
    version: i32,
    not_before_day: u32,
    not_after_day: u32,
) -> GeorgeResult<X509> {
    let ca_sk = rsa::load_sk_file(ca_sk_path.to_string())?;
    let sk = rsa::load_sk_file(sk_path.to_string())?;
    let ca_cert = load_ca_file(ca_cert_path.to_string())?;
    match sign_cert(
        ca_cert,
        &ca_sk,
        bits,
        &sk,
        subject_info.build().as_ref(),
        alt_name,
        version,
        not_before_day,
        not_after_day,
    ) {
        Ok(x509) => Ok(x509),
        Err(err) => Err(err_strs("sign", err)),
    }
}

pub fn load_ca(sk: Vec<u8>) -> GeorgeResult<X509> {
    match X509::from_pem(sk.as_slice()) {
        Ok(key) => Ok(key),
        Err(err) => Err(err_strs("x509_from_pem", err)),
    }
}

pub fn load_ca_file(filepath: String) -> GeorgeResult<X509> {
    match read(filepath) {
        Ok(u8s) => load_ca(u8s),
        Err(err) => Err(err_strs("read", err)),
    }
}
