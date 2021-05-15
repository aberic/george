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

use openssl::asn1::{Asn1Integer, Asn1Time};
use openssl::bn::{BigNum, MsbOption};
use openssl::error::ErrorStack;
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::{PKey, PKeyRef, Private, Public};
use openssl::x509::extension::{
    AuthorityKeyIdentifier, BasicConstraints, ExtendedKeyUsage, KeyUsage, SubjectAlternativeName,
    SubjectKeyIdentifier,
};
use openssl::x509::{
    X509Extension, X509Name, X509NameBuilder, X509NameRef, X509Req, X509ReqBuilder, X509,
};

use crate::cryptos::rsa::{RSALoadKey, RSA};
use crate::errors::entrances::GeorgeResult;
use crate::errors::entrances::{err_str, err_strs};

pub struct Cert {
    x509: X509,
}

impl Cert {
    /// 签发根证书
    ///
    /// * bits 以比特为单位的数字长度
    /// * msb 期望的最高位属性，是随机生成' BigNum '的最有效位的选项
    /// * odd 如果' true '，则生成的数字为奇数
    /// * sk 签发证书用的私钥
    /// * pk 待签发证书的公钥
    /// * subject_info 待签发证书信息，在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
    ///   CN字段用于普通名称，例如DNS名称
    /// * version 证书版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
    /// * not_before_day 证书上的有效期在指定天之后
    /// * not_after_day 证书上的有效期在指定天之前
    /// * message_digest 生成签名时摘要算法，如：MessageDigest::sha256()
    pub fn sign_root(
        bits: i32,
        msb_ca: MsbOptionCA,
        odd: bool,
        sk: PKey<Private>,
        pk: PKey<Public>,
        subject_info: X509NameInfo,
        version: i32,
        not_before_day: u32,
        not_after_day: u32,
        message_digest: MessageDigest,
    ) -> GeorgeResult<Cert> {
        match generate_x509(
            None,
            sk,
            pk,
            SerialNumber::new(bits, msb_ca, odd),
            subject_info,
            version,
            not_before_day,
            not_after_day,
            Extensions {
                basic_constraints: ca_basic_constraints_ext()?,
                key_usage: ca_key_usage_ext()?,
                ext_key_usage: None,
                subject_alternative_name: None,
            },
            message_digest,
        ) {
            Ok(x509) => Ok(Cert { x509 }),
            Err(err) => Err(err_strs("create_cert", err)),
        }
    }

    /// 签发中间证书
    ///
    /// * op_x509 根证书。待签发证书如果自签名则为None，否则不能为None
    /// * bits 以比特为单位的数字长度
    /// * msb 期望的最高位属性，是随机生成' BigNum '的最有效位的选项
    /// * odd 如果' true '，则生成的数字为奇数
    /// * sk 签发证书用的私钥
    /// * pk 待签发证书的公钥
    /// * subject_info 证书的主题信息，在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
    /// * issuer_info 证书的发布者信息，在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
    ///   CN字段用于普通名称，例如DNS名称
    /// * version 证书版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
    /// * not_before_day 证书上的有效期在指定天之后
    /// * not_after_day 证书上的有效期在指定天之前
    /// * message_digest 生成签名时摘要算法，如：MessageDigest::sha256()
    pub fn sign_intermediate(
        op_x509: Option<X509>,
        bits: i32,
        msb_ca: MsbOptionCA,
        odd: bool,
        sk: PKey<Private>,
        pk: PKey<Public>,
        subject_info: X509NameInfo,
        version: i32,
        not_before_day: u32,
        not_after_day: u32,
        message_digest: MessageDigest,
    ) -> GeorgeResult<Cert> {
        if op_x509.is_none() {
            return Err(err_str("root cert can't be none while sign intermediate!"));
        }
        match generate_x509(
            op_x509,
            sk,
            pk,
            SerialNumber::new(bits, msb_ca, odd),
            subject_info,
            version,
            not_before_day,
            not_after_day,
            Extensions {
                basic_constraints: ca_basic_constraints_ext()?,
                key_usage: ca_key_usage_ext()?,
                ext_key_usage: None,
                subject_alternative_name: None,
            },
            message_digest,
        ) {
            Ok(x509) => Ok(Cert { x509 }),
            Err(err) => Err(err_strs("create_cert", err)),
        }
    }

    /// 签发用户证书
    ///
    /// * op_x509 根证书。待签发证书如果自签名则为None，否则不能为None
    /// * bits 以比特为单位的数字长度
    /// * msb 期望的最高位属性，是随机生成' BigNum '的最有效位的选项
    /// * odd 如果' true '，则生成的数字为奇数
    /// * sk 签发证书用的私钥
    /// * pk 待签发证书的公钥
    /// * subject_info 证书的主题信息，在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
    /// * issuer_info 证书的发布者信息，在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
    ///   CN字段用于普通名称，例如DNS名称
    /// * version 证书版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
    /// * not_before_day 证书上的有效期在指定天之后
    /// * not_after_day 证书上的有效期在指定天之前
    /// * message_digest 生成签名时摘要算法，如：MessageDigest::sha256()
    pub fn sign_user(
        op_x509: Option<X509>,
        bits: i32,
        msb_ca: MsbOptionCA,
        odd: bool,
        sk: PKey<Private>,
        pk: PKey<Public>,
        subject_info: X509NameInfo,
        version: i32,
        not_before_day: u32,
        not_after_day: u32,
        message_digest: MessageDigest,
    ) -> GeorgeResult<Cert> {
        if op_x509.is_none() {
            return Err(err_str("root cert can't be none while sign intermediate!"));
        }
        let basic_constraints: X509Extension;
        match BasicConstraints::new() // 基本约束
            .critical() // 关键
            .build()
        {
            Ok(ext) => basic_constraints = ext,
            Err(err) => return Err(err_strs("BasicConstraints build", err)),
        }
        let key_usage: X509Extension;
        match KeyUsage::new() // 密钥使用
            .critical() // 关键
            .data_encipherment() // 数字签名
            .key_encipherment() // 密钥加密
            .build()
        {
            Ok(ext) => key_usage = ext,
            Err(err) => return Err(err_strs("BasicConstraints build", err)),
        }
        let ext_key_usage: Option<X509Extension>;
        match ExtendedKeyUsage::new() // 扩展的密钥使用
            .server_auth() // 服务器认证
            .client_auth() // 客户端认证
            .build()
        {
            Ok(ext) => ext_key_usage = Some(ext),
            Err(err) => return Err(err_strs("BasicConstraints build", err)),
        }
        match generate_x509(
            op_x509,
            sk,
            pk,
            SerialNumber::new(bits, msb_ca, odd),
            subject_info,
            version,
            not_before_day,
            not_after_day,
            Extensions {
                basic_constraints,
                key_usage,
                ext_key_usage,
                subject_alternative_name: None,
            },
            message_digest,
        ) {
            Ok(x509) => Ok(Cert { x509 }),
            Err(err) => Err(err_strs("create_cert", err)),
        }
    }
}

/// 签发证书
///
/// * op_x509 根证书。待签发证书如果自签名则为None，否则不能为None
/// * sk 签发证书用的私钥
/// * pk 待签发证书的公钥
/// * subject_info 证书的主题信息，在构建证书时，使用openssl等命令行工具时通常使用C、ST和O选项。CN字段用于通用名称，比如DNS名称
///   CN字段用于普通名称，例如DNS名称
/// * version 证书版本。版本是零索引的，也就是说，对应于X.509标准版本3的证书应该将“2”传递给该方法。
/// * not_before_day 证书上的有效期在指定天之后
/// * not_after_day 证书上的有效期在指定天之前
/// * is_ca 是否证书颁发机构
/// * extensions 证书扩展对象
/// * message_digest 生成签名时摘要算法，如：MessageDigest::sha256()
fn generate_x509(
    op_x509: Option<X509>,
    sk: PKey<Private>,
    pk: PKey<Public>,
    serial_number: SerialNumber,
    subject_info: X509NameInfo,
    version: i32,
    not_before_day: u32,
    not_after_day: u32,
    extensions: Extensions,
    message_digest: MessageDigest,
) -> Result<X509, ErrorStack> {
    // 新建用于构造X509的构造器
    let mut cert_builder = X509::builder()?;
    // 设置证书版本
    cert_builder.set_version(version)?;
    let serial_number = serial_number.generate()?;
    // 设置证书的序列号
    cert_builder.set_serial_number(&serial_number)?;
    // 设置待签发证书的主题信息
    cert_builder.set_subject_name(&subject_info.build())?;
    // 设置与证书关联的公钥
    cert_builder.set_pubkey(&pk)?;
    // 从现在开始按指定的天数间隔创建一个新的时间
    let not_before = Asn1Time::days_from_now(not_before_day)?;
    // 设置证书上的有效期在指定天之后
    cert_builder.set_not_before(&not_before)?;
    // 从现在开始按指定的天数间隔创建一个新的时间
    let not_after = Asn1Time::days_from_now(not_after_day)?;
    // 设置证书上的有效期在指定天之前
    cert_builder.set_not_after(&not_after)?;
    // 将X509扩展值添加到证书
    cert_builder.append_extension(extensions.basic_constraints)?;
    cert_builder.append_extension(extensions.key_usage)?;
    match extensions.ext_key_usage {
        Some(ext) => cert_builder.append_extension(ext)?,
        _ => {}
    }
    match op_x509 {
        Some(x509) => {
            // 设置签发证书的颁发者信息
            cert_builder.set_issuer_name(x509.subject_name())?;
            cert_builder.append_extension(
                SubjectKeyIdentifier::new() // 主题密钥标识符
                    // 如果证书是自签名的，则将“发布者”设置为“None”。
                    .build(&cert_builder.x509v3_context(Some(x509.as_ref()), None))?,
            )?;
            cert_builder.append_extension(
                AuthorityKeyIdentifier::new() // 授权密钥标识符
                    .keyid(true)
                    .build(&cert_builder.x509v3_context(Some(x509.as_ref()), None))?,
            )?;
        }
        None => {
            // 设置签发证书的颁发者信息
            cert_builder.set_issuer_name(&subject_info.build())?;
            cert_builder.append_extension(
                SubjectKeyIdentifier::new() // 主题密钥标识符
                    // 如果证书是自签名的，则将“发布者”设置为“None”。
                    .build(&cert_builder.x509v3_context(None, None))?,
            )?;
            cert_builder.append_extension(
                AuthorityKeyIdentifier::new() // 授权密钥标识符
                    .keyid(true)
                    .build(&cert_builder.x509v3_context(None, None))?,
            )?;
        }
    }
    match extensions.subject_alternative_name {
        Some(ext) => cert_builder.append_extension(ext)?,
        _ => {}
    }
    // 使用私钥签名证书
    cert_builder.sign(&sk, message_digest)?;
    Ok(cert_builder.build())
}

/// 生成证书颁发机构的基本约束扩展
fn ca_basic_constraints_ext() -> GeorgeResult<X509Extension> {
    match BasicConstraints::new() // 基本约束
        .critical() // 关键
        .ca() // 是证书颁发机构
        .build()
    {
        Ok(ext) => Ok(ext),
        Err(err) => Err(err_strs("BasicConstraints build", err)),
    }
}

/// 生成证书颁发机构的密钥使用扩展
fn ca_key_usage_ext() -> GeorgeResult<X509Extension> {
    match KeyUsage::new() // 密钥使用
        .critical() // 关键
        .data_encipherment() // 数字签名
        .key_cert_sign() // 密钥证书签名
        .crl_sign() // CRL签名
        .build()
    {
        Ok(ext) => Ok(ext),
        Err(err) => Err(err_strs("KeyUsage build", err)),
    }
}

/// 证书扩展对象
pub struct Extensions {
    /// 基本约束
    ///
    /// # Examples
    ///
    /// ```
    /// use openssl::x509::X509Extension;
    /// use openssl::x509::extension::BasicConstraints;
    ///
    /// fn basic_constraints() -> X509Extension {
    ///     BasicConstraints::new() // 基本约束
    ///         .critical() // 关键
    ///         .ca() // 是证书颁发机构
    ///         .build().unwrap()
    /// }
    /// ```
    basic_constraints: X509Extension,
    /// 密钥使用
    ///
    /// # Examples
    ///
    /// ```
    /// use openssl::x509::X509Extension;
    /// use openssl::x509::extension::KeyUsage;
    ///
    /// fn key_usage() -> X509Extension {
    ///     KeyUsage::new() // 密钥使用
    ///         .critical() // 关键
    ///         .data_encipherment() // 数字签名
    ///         .key_cert_sign() // 密钥证书签名
    ///         .crl_sign() // CRL签名
    ///         .build().unwrap()
    /// }
    /// ```
    key_usage: X509Extension,
    /// 扩展的密钥使用/指示证书公钥用途扩展
    ///
    /// # Examples
    ///
    /// ```
    /// use openssl::x509::X509Extension;
    /// use openssl::x509::extension::ExtendedKeyUsage;
    ///
    /// fn ext_key_usage() -> X509Extension {
    ///     ExtendedKeyUsage::new() // 扩展的密钥使用
    ///         .server_auth() // 服务器认证
    ///         .client_auth() // 客户端认证
    ///         .other("2.999.1")
    ///         .build().unwrap()
    /// }
    /// ```
    ext_key_usage: Option<X509Extension>,
    /// 主题备用名称
    ///
    /// # Examples
    ///
    /// ```
    /// use openssl::x509::extension::SubjectAlternativeName;
    /// use openssl::x509::X509Extension;
    ///
    /// fn subject_alternative_name() -> X509Extension {
    ///     SubjectAlternativeName::new() // 主题备用名称
    ///         .dns("example.com")
    ///         .email("info@example.com")
    ///         .build(&cert_builder.x509v3_context(None, None)).unwrap()
    /// }
    /// ```
    subject_alternative_name: Option<X509Extension>,
}

impl Extensions {
    /// 新建证书扩展集
    ///
    /// # Examples
    ///
    /// ```
    /// use openssl::x509::extension::{SubjectAlternativeName, BasicConstraints, KeyUsage, ExtendedKeyUsage, AuthorityKeyIdentifier};
    /// use openssl::x509::X509Extension;
    ///
    /// fn basic_constraints() -> X509Extension {
    ///     BasicConstraints::new() // 基本约束
    ///         .critical() // 关键
    ///         .ca() // 是证书颁发机构
    ///         .build().unwrap()
    /// }
    ///
    /// fn key_usage() -> X509Extension {
    ///     KeyUsage::new() // 密钥使用
    ///         .critical() // 关键
    ///         .data_encipherment() // 数字签名
    ///         .key_cert_sign() // 密钥证书签名
    ///         .crl_sign() // CRL签名
    ///         .build().unwrap()
    /// }
    ///
    /// fn ext_key_usage() -> X509Extension {
    ///     ExtendedKeyUsage::new() // 扩展的密钥使用
    ///         .server_auth() // 服务器认证
    ///         .client_auth() // 客户端认证
    ///         .other("2.999.1")
    ///         .build().unwrap()
    /// }
    ///
    /// fn subject_alternative_name() -> X509Extension {
    ///     SubjectAlternativeName::new() // 主题备用名称
    ///         .dns("example.com")
    ///         .email("info@example.com")
    ///         .build(&cert_builder.x509v3_context(None, None)).unwrap()
    /// }
    /// ```
    pub fn new(
        basic_constraints: X509Extension,
        key_usage: X509Extension,
        ext_key_usage: Option<X509Extension>,
        subject_alternative_name: Option<X509Extension>,
    ) -> Extensions {
        Extensions {
            basic_constraints,
            key_usage,
            ext_key_usage,
            subject_alternative_name,
        }
    }
}

/// Options for the most significant bits of a randomly generated `BigNum`.
pub enum MsbOptionCA {
    /// The most significant bit of the number may be 0.
    One,
    /// The most significant bit of the number must be 1.
    MaybeZero,
    /// The most significant two bits of the number must be 1.
    ///
    /// The number of bits in the product of two such numbers will always be exactly twice the
    /// number of bits in the original numbers.
    TwoOnes,
}

/// 证书体系序列号
pub struct SerialNumber {
    /// * bits 以比特为单位的数字长度，用于生成一个bits位奇数随机数
    bits: i32,
    /// * msb 期望的最高位属性，是随机生成' BigNum '的最有效位的选项
    msb_ca: MsbOptionCA,
    /// * odd 如果' true '，则生成的数字为奇数
    odd: bool,
}

impl SerialNumber {
    /// 生成序列号对象
    ///
    /// * bits 以比特为单位的数字长度
    /// * msb 期望的最高位属性，是随机生成' BigNum '的最有效位的选项
    /// * odd 如果' true '，则生成的数字为奇数
    pub fn new(bits: i32, msb_ca: MsbOptionCA, odd: bool) -> SerialNumber {
        SerialNumber { bits, msb_ca, odd }
    }

    /// 生成序列号
    ///
    /// 数字表示法ASN.1中的整数可能包括BigNum、int64或uint64
    ///
    /// * bits 以比特为单位的数字长度
    /// * msb 期望的最高位属性，是随机生成' BigNum '的最有效位的选项
    /// * odd 如果' true '，则生成的数字为奇数
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use comm::errors::entrances::GeorgeResult;
    /// use openssl::asn1::Asn1Integer;
    /// use openssl::bn::MsbOption;
    ///
    /// fn serial_number() -> GeorgeResult<Asn1Integer> {
    ///    generate_serial_number(128, MsbOption::MAYBE_ZERO, false)
    /// }
    /// ```
    fn generate(&self) -> Result<Asn1Integer, ErrorStack> {
        // 创建一个值为0的新' BigNum '。
        let mut big_num = BigNum::new()?;
        // 生成一个加密强伪随机' BigNum '
        match self.msb_ca {
            MsbOptionCA::One => big_num.rand(self.bits, MsbOption::ONE, self.odd)?,
            MsbOptionCA::MaybeZero => big_num.rand(self.bits, MsbOption::MAYBE_ZERO, self.odd)?,
            MsbOptionCA::TwoOnes => big_num.rand(self.bits, MsbOption::TWO_ONES, self.odd)?,
        }
        // 返回' Asn1Integer '
        big_num.to_asn1_integer()
    }
}

#[derive(Debug, Clone)]
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

// todo 追加用于主题备用名称
impl X509NameInfo {
    fn build(&self) -> X509Name {
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

////////////////////////////////////////////////////////////////////////////////

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
    let ca_sk = RSA::load_sk(ca_sk_path)?;
    let sk = RSA::load_sk(sk_path)?;
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
