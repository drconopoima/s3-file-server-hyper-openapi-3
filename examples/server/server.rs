//! Main library entry point for harden_file_transfer_rs implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use harden_file_transfer_rs::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    let mut service =
        harden_file_transfer_rs::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use harden_file_transfer_rs::{
    Api,
    CreateFileResponse,
    CreateShareURLResponse,
    DeleteFileByIdResponse,
    DownloadFileResponse,
    GetFileByIdResponse,
    ListFilesResponse,
    RecoverFileResponse,
    UploadFileResponse,
};
use harden_file_transfer_rs::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Create a file object metadata
    async fn create_file(
        &self,
        file: models::File,
        context: &C) -> Result<CreateFileResponse, ApiError>
    {
        let context = context.clone();
        info!("create_file({:?}) - X-Span-ID: {:?}", file, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Generate a shareable URL for a file object
    async fn create_share_url(
        &self,
        file_id: uuid::Uuid,
        expires: Option<chrono::DateTime::<chrono::Utc>>,
        context: &C) -> Result<CreateShareURLResponse, ApiError>
    {
        let context = context.clone();
        info!("create_share_url({:?}, {:?}) - X-Span-ID: {:?}", file_id, expires, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Delete a file object content and metadata properties
    async fn delete_file_by_id(
        &self,
        file_id: uuid::Uuid,
        context: &C) -> Result<DeleteFileByIdResponse, ApiError>
    {
        let context = context.clone();
        info!("delete_file_by_id({:?}) - X-Span-ID: {:?}", file_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Download a file
    async fn download_file(
        &self,
        file_id: uuid::Uuid,
        token: Option<String>,
        context: &C) -> Result<DownloadFileResponse, ApiError>
    {
        let context = context.clone();
        info!("download_file({:?}, {:?}) - X-Span-ID: {:?}", file_id, token, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Retrieve specific file object metadata properties
    async fn get_file_by_id(
        &self,
        file_id: uuid::Uuid,
        context: &C) -> Result<GetFileByIdResponse, ApiError>
    {
        let context = context.clone();
        info!("get_file_by_id({:?}) - X-Span-ID: {:?}", file_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List all files
    async fn list_files(
        &self,
        limit: Option<i32>,
        context: &C) -> Result<ListFilesResponse, ApiError>
    {
        let context = context.clone();
        info!("list_files({:?}) - X-Span-ID: {:?}", limit, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Recovers a deleted file object content and metadata properties
    async fn recover_file(
        &self,
        file_id: uuid::Uuid,
        context: &C) -> Result<RecoverFileResponse, ApiError>
    {
        let context = context.clone();
        info!("recover_file({:?}) - X-Span-ID: {:?}", file_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Upload a file
    async fn upload_file(
        &self,
        file_id: uuid::Uuid,
        context: &C) -> Result<UploadFileResponse, ApiError>
    {
        let context = context.clone();
        info!("upload_file({:?}) - X-Span-ID: {:?}", file_id, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
