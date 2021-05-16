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

#[cfg(test)]
mod ca {
    use crate::cryptos::ca::{Cert, MsbOptionCA, X509NameInfo};
    use crate::cryptos::rsa::RSA;
    use crate::cryptos::sm2::SM2;
    use openssl::hash::MessageDigest;

    #[test]
    fn test() {
        let rsa_root = RSA::new(1024).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNRoot".to_string(),
            "CN".to_string(),
            Some("org".to_string()),
            Some("org unit".to_string()),
            Some("loc".to_string()),
            Some("pro".to_string()),
            Some("sa".to_string()),
            Some("email@tt.cn".to_string()),
            Some("128.0.9.1".to_string()),
            Some("tt.cn".to_string()),
        )
        .unwrap();
        let root = Cert::sign_root(
            512,
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            rsa_root.pk(),
            subject_info,
            2,
            0,
            365,
            MessageDigest::sha384(),
        )
        .unwrap();
        root.save_pem("src/test/crypto/ca/sign/root.pem.crt")
            .unwrap();
        root.save_der("src/test/crypto/ca/sign/root.der.crt")
            .unwrap();

        let rsa_intermediate = RSA::new(512).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNIntermediate".to_string(),
            "CN".to_string(),
            Some("org inter".to_string()),
            Some("org unit inter".to_string()),
            Some("loc inter".to_string()),
            Some("pro inter".to_string()),
            Some("sa inter".to_string()),
            Some("email@inter.cn".to_string()),
            Some("128.0.9.2".to_string()),
            Some("inter.cn".to_string()),
        )
        .unwrap();
        let intermediate_cert = Cert::sign_intermediate(
            root.x509,
            512,
            MsbOptionCA::MaybeZero,
            true,
            rsa_root.sk(),
            rsa_intermediate.pk(),
            subject_info,
            2,
            0,
            365,
            MessageDigest::sha384(),
        )
        .unwrap();
        intermediate_cert
            .save_pem("src/test/crypto/ca/sign/intermediate.pem.crt")
            .unwrap();
        intermediate_cert
            .save_der("src/test/crypto/ca/sign/intermediate.der.crt")
            .unwrap();

        let rsa_user = RSA::new(512).unwrap();
        let subject_info = X509NameInfo::new_cus(
            "CNUser".to_string(),
            "CN".to_string(),
            Some("org user".to_string()),
            Some("org unit user".to_string()),
            Some("loc user".to_string()),
            Some("pro user".to_string()),
            Some("sa user".to_string()),
            Some("email@user.cn".to_string()),
            Some("128.0.9.3".to_string()),
            Some("user.cn".to_string()),
        )
        .unwrap();
        let user_cert = Cert::sign_user(
            intermediate_cert.x509,
            256,
            MsbOptionCA::MaybeZero,
            true,
            rsa_intermediate.sk(),
            rsa_user.pk(),
            subject_info,
            2,
            0,
            365,
            MessageDigest::sha3_256(),
        )
        .unwrap();
        user_cert
            .save_pem("src/test/crypto/ca/sign/user.pem.crt")
            .unwrap();
        user_cert
            .save_der("src/test/crypto/ca/sign/user.der.crt")
            .unwrap();
    }
}
