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

//! A openssl adaptor for `tonic`.
//!
//! Examples can be found in the `example` crate
//! within the repository.

#![doc(html_root_url = "https://docs.rs/tonic-openssl/0.1.0")]
#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

use std::{
    fmt::Debug,
    io,
    pin::Pin,
    task::{Context, Poll},
};

use async_stream::try_stream;
use futures_util::{Stream, TryStream, TryStreamExt};
use openssl::ssl::SslAcceptor;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tonic::transport::server::Connected;

/// Wrapper error type.
pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

/// A const that contains the on the wire `h2` alpn
/// value that can be passed directly to OpenSSL.
pub const ALPN_H2_WIRE: &[u8] = b"\x02h2";

/// Wrap some incoming stream of io types with OpenSSL's
/// `SslStream` type. This will take some acceptor and a
/// stream of io types and accept connections.
pub fn incoming<S>(
    tcp_listener_stream: S,
    acceptor: SslAcceptor,
) -> impl Stream<Item = Result<SslStream<S::Ok>, Error>>
where
    S: TryStream + Unpin,
    S::Ok: AsyncRead + AsyncWrite + Send + Sync + Debug + Unpin + 'static,
    S::Error: Into<Error>,
{
    let mut tcp_listener_stream = tcp_listener_stream;

    // try_stream! {
    //     while let Some(stream) = tcp_listener_stream.try_next().await? {
    //         let ssl = openssl::ssl::Ssl::new(acceptor.context()).unwrap();
    //         let mut tls = super::tokio::SslStream::new(ssl, stream).unwrap();
    //         Pin::new(&mut tls).accept().await?;
    //
    //         let ssl = SslStream {
    //             inner: tls
    //         };
    //
    //         yield ssl;
    //     }
    // }

    try_stream! {
        while let Some(stream) = tcp_listener_stream.try_next().await? {
            match openssl::ssl::Ssl::new(acceptor.context()) {
                Ok(ssl) => {
                    if ssl.verify_result() != openssl::x509::X509VerifyResult::OK {
                        println!("ssl verify failed!")
                    }
                    match super::tokio::SslStream::new(ssl, stream) {
                        Ok(mut tls) => {
                            Pin::new(&mut tls).accept().await?;

                            let ssl = SslStream {
                                inner: tls
                            };

                            yield ssl;
                        },
                        Err(err) => println!("error is {}", err)
                    }
                },
                Err(err) => println!("error is {}", err)
            }
        }
    }

    // while let Some(stream) = tcp_listener_stream.try_next().await? {
    //     // let tls = tokio_openssl::accept(&acceptor, stream).await?;
    //
    //     match openssl::ssl::Ssl::new(acceptor.context()) {
    //         Ok(ssl) => {
    //             if ssl.verify_result() != openssl::x509::X509VerifyResult::OK {
    //                 println!("ssl verify failed!")
    //             }
    //             match super::tokio::SslStream::new(ssl, stream) {
    //                 Ok(mut tls) => {
    //                     Pin::new(&mut tls).accept().await?;
    //
    //                     let ssl = SslStream {
    //                         inner: res
    //                     };
    //
    //                     yield ssl;
    //                 },
    //                 Err(err) => println!("error is {}", err)
    //             }
    //         },
    //         Err(err) => println!("error is {}", err)
    //     }
    //
    //     let ssl = openssl::ssl::Ssl::new(acceptor.context()).unwrap();
    //     let tls = super::tokio::SslStream::new(ssl, stream).unwrap();
    //     Pin::new(&tls).accept().await?;
    //
    //     let ssl = SslStream {
    //         inner: tls
    //     };
    //
    //     yield ssl;
    // }
}

/// A `SslStream` wrapper type that implements tokio's io traits
/// and tonic's `Connected` trait.
#[derive(Debug)]
pub struct SslStream<S> {
    inner: super::tokio::SslStream<S>,
}

impl<S: Connected> Connected for SslStream<S> {
    type ConnectInfo = ();

    fn connect_info(&self) -> Self::ConnectInfo {}
}

impl<S> AsyncRead for SslStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl<S> AsyncWrite for SslStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}
