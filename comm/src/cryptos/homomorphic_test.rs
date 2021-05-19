/*
 * Copyright (c) 2021. Aberic - All Rights Reserved.
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
mod homomorphic {

    /// Facebook cupcake
    #[cfg(test)]
    mod cupcake {
        use cupcake::traits::*;

        #[test]
        fn basic() {
            let fv = cupcake::default();

            let (pk, sk) = fv.generate_keypair();

            println!("Encrypting a constant vector v of 1s...");
            let v = vec![1; fv.n];
            print!("created   v: ");
            smart_print(&v);

            let mut ctv = fv.encrypt(&v, &pk);

            let mut pt_actual = fv.decrypt(&ctv, &sk);
            print!("decrypted v: ");
            smart_print(&pt_actual);

            println!("Encrypting a constant vector w of 2s...");
            let w = vec![2; fv.n];
            print!("created   w: ");
            smart_print(&w);

            let ctw = fv.encrypt(&w, &pk);

            pt_actual = fv.decrypt(&ctw, &sk);
            print!("decrypted w: ");
            smart_print(&pt_actual);

            // add ctw into ctv
            fv.add_inplace(&mut ctv, &ctw);
            print!("Decrypting the sum...");
            pt_actual = fv.decrypt(&ctv, &sk);
            print!("decrypted v+w: ");
            smart_print(&pt_actual);

            // add the plaintext w into the ciphertext
            fv.add_plain_inplace(&mut ctv, &w);
            print!("Decrypting the sum...");
            pt_actual = fv.decrypt(&ctv, &sk);
            print!("decrypted v+w+w: ");
            smart_print(&pt_actual);
        }

        fn smart_print<T: std::fmt::Debug>(v: &Vec<T>) {
            println!("[{:?}, {:?}, ..., {:?}]", v[0], v[1], v[v.len() - 1]);
        }

        #[test]
        fn re_random_ize() {
            let fv = cupcake::default();

            let (pk, sk) = fv.generate_keypair();

            println!("Encrypting a vector [0,1,2,3,...]");
            let mut v = vec![];
            for i in 0..fv.n {
                v.push(i as u8);
            }

            let mut ctv = fv.encrypt(&v, &pk);

            let pt_original = fv.decrypt(&ctv, &sk);
            print!("decrypted value: ");
            smart_print(&pt_original);

            println!("Rerandomizing the ciphertext...");

            fv.rerandomize(&mut ctv, &pk);
            print!("decrypted value after reranromization: ");
            let pt_new = fv.decrypt(&ctv, &sk);
            smart_print(&pt_new);

            print!("Check that the plaintext has not changed...");
            assert_eq!(pt_original, pt_new);
            println!("ok");
        }
    }

    // /// zama concrete
    // #[cfg(test)]
    // mod cupcake {
    //
    //     use concrete::*;
    //
    //     #[test]
    //     fn example() {
    //         // generate a secret key
    //         let secret_key = LWESecretKey::new(&LWE128_630);
    //
    //         // the two values to add
    //         let m1 = 8.2;
    //         let m2 = 5.6;
    //
    //         // specify the range and precision to encode messages into plaintexts
    //         // here we encode in [0, 10[ with 8 bits of precision and 1 bit of padding
    //         let encoder = Encoder::new(0., 10., 8, 1)?;
    //
    //         // encode the messages into plaintexts
    //         let p1 = encoder.encode_single(m1)?;
    //         let p2 = encoder.encode_single(m2)?;
    //
    //         // encrypt plaintexts
    //         let mut c1 = VectorLWE::encrypt(&secret_key, &p1)?;
    //         let c2 = VectorLWE::encrypt(&secret_key, &p2)?;
    //
    //         // add the two ciphertexts homomorphically
    //         c1.add_with_padding_inplace(&c2)?;
    //
    //         // decrypt and decode the result
    //         let m3 = c1.decrypt_decode(&secret_key)?;
    //
    //         // print the result and compare to non-FHE addition
    //         println!("Real: {} + {} = {}", m1, m2, m1 + m2);
    //         println!("FHE: {} + {} = {}", p1.decode()?[0], p2.decode()?[0], m3[0]);
    //     }
    // }
}
