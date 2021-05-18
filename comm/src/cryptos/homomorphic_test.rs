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

    #[cfg(test)]
    mod demo {
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
}
