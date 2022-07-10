#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &'static str = "/v1";
pub const API_VERSION: &'static str = "0.1.0";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateFileResponse {
    /// Successful file object metadata creation request
    SuccessfulFileObjectMetadataCreationRequest
    (models::File)
    ,
    /// Null response
    NullResponse
    ,
    /// Authentication information is missing or invalid
    AuthenticationInformationIsMissingOrInvalid
    {
        www_authenticate:
        Option<
        String
        >
    }
    ,
    /// Unexpected error
    UnexpectedError
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateShareURLResponse {
    /// Successful generate shareable URL request
    SuccessfulGenerateShareableURLRequest
    (models::File)
    ,
    /// Authentication information is missing or invalid
    AuthenticationInformationIsMissingOrInvalid
    {
        www_authenticate:
        Option<
        String
        >
    }
    ,
    /// unexpected error
    UnexpectedError
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DeleteFileByIdResponse {
    /// Delete file success response to a valid request
    DeleteFileSuccessResponseToAValidRequest
    (models::File)
    ,
    /// Authentication information is missing or invalid
    AuthenticationInformationIsMissingOrInvalid
    {
        www_authenticate:
        Option<
        String
        >
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DownloadFileResponse {
    /// Download file success response to a valid request
    DownloadFileSuccessResponseToAValidRequest
    (swagger::ByteArray)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetFileByIdResponse {
    /// Describe file success response to a valid request
    DescribeFileSuccessResponseToAValidRequest
    (models::File)
    ,
    /// Authentication information is missing or invalid
    AuthenticationInformationIsMissingOrInvalid
    {
        www_authenticate:
        Option<
        String
        >
    }
    ,
    /// Unexpected error
    UnexpectedError
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListFilesResponse {
    /// A paged array of files
    APagedArrayOfFiles
    {
        body: Vec<models::File>,
        x_next:
        Option<
        String
        >
    }
    ,
    /// Authentication information is missing or invalid
    AuthenticationInformationIsMissingOrInvalid
    {
        www_authenticate:
        Option<
        String
        >
    }
    ,
    /// Unexpected error
    UnexpectedError
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RecoverFileResponse {
    /// Successful recover deleted file request
    SuccessfulRecoverDeletedFileRequest
    (models::File)
    ,
    /// Authentication information is missing or invalid
    AuthenticationInformationIsMissingOrInvalid
    {
        www_authenticate:
        Option<
        String
        >
    }
    ,
    /// unexpected error
    UnexpectedError
    (models::Error)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UploadFileResponse {
    /// Successful file upload request
    SuccessfulFileUploadRequest
    (models::File)
    ,
    /// Null response
    NullResponse
    ,
    /// Authentication information is missing or invalid
    AuthenticationInformationIsMissingOrInvalid
    {
        www_authenticate:
        Option<
        String
        >
    }
    ,
    /// Unexpected error
    UnexpectedError
    (models::Error)
}

/// API
#[async_trait]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Create a file object metadata
    async fn create_file(
        &self,
        file: models::File,
        context: &C) -> Result<CreateFileResponse, ApiError>;

    /// Generate a shareable URL for a file object
    async fn create_share_url(
        &self,
        file_id: uuid::Uuid,
        expires: Option<chrono::DateTime::<chrono::Utc>>,
        context: &C) -> Result<CreateShareURLResponse, ApiError>;

    /// Delete a file object content and metadata properties
    async fn delete_file_by_id(
        &self,
        file_id: uuid::Uuid,
        context: &C) -> Result<DeleteFileByIdResponse, ApiError>;

    /// Download a file
    async fn download_file(
        &self,
        file_id: uuid::Uuid,
        token: Option<String>,
        context: &C) -> Result<DownloadFileResponse, ApiError>;

    /// Retrieve specific file object metadata properties
    async fn get_file_by_id(
        &self,
        file_id: uuid::Uuid,
        context: &C) -> Result<GetFileByIdResponse, ApiError>;

    /// List all files
    async fn list_files(
        &self,
        limit: Option<i32>,
        context: &C) -> Result<ListFilesResponse, ApiError>;

    /// Recovers a deleted file object content and metadata properties
    async fn recover_file(
        &self,
        file_id: uuid::Uuid,
        context: &C) -> Result<RecoverFileResponse, ApiError>;

    /// Upload a file
    async fn upload_file(
        &self,
        file_id: uuid::Uuid,
        context: &C) -> Result<UploadFileResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Create a file object metadata
    async fn create_file(
        &self,
        file: models::File,
        ) -> Result<CreateFileResponse, ApiError>;

    /// Generate a shareable URL for a file object
    async fn create_share_url(
        &self,
        file_id: uuid::Uuid,
        expires: Option<chrono::DateTime::<chrono::Utc>>,
        ) -> Result<CreateShareURLResponse, ApiError>;

    /// Delete a file object content and metadata properties
    async fn delete_file_by_id(
        &self,
        file_id: uuid::Uuid,
        ) -> Result<DeleteFileByIdResponse, ApiError>;

    /// Download a file
    async fn download_file(
        &self,
        file_id: uuid::Uuid,
        token: Option<String>,
        ) -> Result<DownloadFileResponse, ApiError>;

    /// Retrieve specific file object metadata properties
    async fn get_file_by_id(
        &self,
        file_id: uuid::Uuid,
        ) -> Result<GetFileByIdResponse, ApiError>;

    /// List all files
    async fn list_files(
        &self,
        limit: Option<i32>,
        ) -> Result<ListFilesResponse, ApiError>;

    /// Recovers a deleted file object content and metadata properties
    async fn recover_file(
        &self,
        file_id: uuid::Uuid,
        ) -> Result<RecoverFileResponse, ApiError>;

    /// Upload a file
    async fn upload_file(
        &self,
        file_id: uuid::Uuid,
        ) -> Result<UploadFileResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self: Self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Create a file object metadata
    async fn create_file(
        &self,
        file: models::File,
        ) -> Result<CreateFileResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_file(file, &context).await
    }

    /// Generate a shareable URL for a file object
    async fn create_share_url(
        &self,
        file_id: uuid::Uuid,
        expires: Option<chrono::DateTime::<chrono::Utc>>,
        ) -> Result<CreateShareURLResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_share_url(file_id, expires, &context).await
    }

    /// Delete a file object content and metadata properties
    async fn delete_file_by_id(
        &self,
        file_id: uuid::Uuid,
        ) -> Result<DeleteFileByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().delete_file_by_id(file_id, &context).await
    }

    /// Download a file
    async fn download_file(
        &self,
        file_id: uuid::Uuid,
        token: Option<String>,
        ) -> Result<DownloadFileResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().download_file(file_id, token, &context).await
    }

    /// Retrieve specific file object metadata properties
    async fn get_file_by_id(
        &self,
        file_id: uuid::Uuid,
        ) -> Result<GetFileByIdResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_file_by_id(file_id, &context).await
    }

    /// List all files
    async fn list_files(
        &self,
        limit: Option<i32>,
        ) -> Result<ListFilesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_files(limit, &context).await
    }

    /// Recovers a deleted file object content and metadata properties
    async fn recover_file(
        &self,
        file_id: uuid::Uuid,
        ) -> Result<RecoverFileResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().recover_file(file_id, &context).await
    }

    /// Upload a file
    async fn upload_file(
        &self,
        file_id: uuid::Uuid,
        ) -> Result<UploadFileResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().upload_file(file_id, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
