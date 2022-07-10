use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     CreateFileResponse,
     CreateShareURLResponse,
     DeleteFileByIdResponse,
     DownloadFileResponse,
     GetFileByIdResponse,
     ListFilesResponse,
     RecoverFileResponse,
     UploadFileResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/v1/files$",
            r"^/v1/files/(?P<fileId>[^/?#]*)$",
            r"^/v1/files/(?P<fileId>[^/?#]*)/download$",
            r"^/v1/files/(?P<fileId>[^/?#]*)/recover$",
            r"^/v1/files/(?P<fileId>[^/?#]*)/share$",
            r"^/v1/files/(?P<fileId>[^/?#]*)/upload$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_FILES: usize = 0;
    pub(crate) static ID_FILES_FILEID: usize = 1;
    lazy_static! {
        pub static ref REGEX_FILES_FILEID: regex::Regex =
            regex::Regex::new(r"^/v1/files/(?P<fileId>[^/?#]*)$")
                .expect("Unable to create regex for FILES_FILEID");
    }
    pub(crate) static ID_FILES_FILEID_DOWNLOAD: usize = 2;
    lazy_static! {
        pub static ref REGEX_FILES_FILEID_DOWNLOAD: regex::Regex =
            regex::Regex::new(r"^/v1/files/(?P<fileId>[^/?#]*)/download$")
                .expect("Unable to create regex for FILES_FILEID_DOWNLOAD");
    }
    pub(crate) static ID_FILES_FILEID_RECOVER: usize = 3;
    lazy_static! {
        pub static ref REGEX_FILES_FILEID_RECOVER: regex::Regex =
            regex::Regex::new(r"^/v1/files/(?P<fileId>[^/?#]*)/recover$")
                .expect("Unable to create regex for FILES_FILEID_RECOVER");
    }
    pub(crate) static ID_FILES_FILEID_SHARE: usize = 4;
    lazy_static! {
        pub static ref REGEX_FILES_FILEID_SHARE: regex::Regex =
            regex::Regex::new(r"^/v1/files/(?P<fileId>[^/?#]*)/share$")
                .expect("Unable to create regex for FILES_FILEID_SHARE");
    }
    pub(crate) static ID_FILES_FILEID_UPLOAD: usize = 5;
    lazy_static! {
        pub static ref REGEX_FILES_FILEID_UPLOAD: regex::Regex =
            regex::Regex::new(r"^/v1/files/(?P<fileId>[^/?#]*)/upload$")
                .expect("Unable to create regex for FILES_FILEID_UPLOAD");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl: api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker.clone(),
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString>  + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match &method {

            // CreateFile - POST /files
            &hyper::Method::POST if path.matched(paths::ID_FILES) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_file: Option<models::File> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_file) => param_file,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter File - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter File due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_file = match param_file {
                                    Some(param_file) => param_file,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter File"))
                                                        .expect("Unable to create Bad Request response for missing body parameter File")),
                                };

                                let result = api_impl.create_file(
                                            param_file,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateFileResponse::SuccessfulFileObjectMetadataCreationRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_FILE_SUCCESSFUL_FILE_OBJECT_METADATA_CREATION_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateFileResponse::NullResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                },
                                                CreateFileResponse::AuthenticationInformationIsMissingOrInvalid
                                                    {
                                                        www_authenticate
                                                    }
                                                => {
                                                    if let Some(www_authenticate) = www_authenticate {
                                                    let www_authenticate = match header::IntoHeaderValue(www_authenticate).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling www_authenticate header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("www_authenticate"),
                                                        www_authenticate
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                CreateFileResponse::UnexpectedError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_FILE_UNEXPECTED_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter File: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter File")),
                        }
            },

            // CreateShareURL - GET /files/{fileId}/share
            &hyper::Method::GET if path.matched(paths::ID_FILES_FILEID_SHARE) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FILES_FILEID_SHARE
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FILES_FILEID_SHARE in set but failed match against \"{}\"", path, paths::REGEX_FILES_FILEID_SHARE.as_str())
                    );

                let param_file_id = match percent_encoding::percent_decode(path_params["fileId"].as_bytes()).decode_utf8() {
                    Ok(param_file_id) => match param_file_id.parse::<uuid::Uuid>() {
                        Ok(param_file_id) => param_file_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter fileId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["fileId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_expires = query_params.iter().filter(|e| e.0 == "expires").map(|e| e.1.to_owned())
                    .nth(0);
                let param_expires = match param_expires {
                    Some(param_expires) => {
                        let param_expires =
                            <chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str
                                (&param_expires);
                        match param_expires {
                            Ok(param_expires) => Some(param_expires),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter expires - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter expires")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.create_share_url(
                                            param_file_id,
                                            param_expires,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateShareURLResponse::SuccessfulGenerateShareableURLRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_SHARE_URL_SUCCESSFUL_GENERATE_SHAREABLE_URL_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                CreateShareURLResponse::AuthenticationInformationIsMissingOrInvalid
                                                    {
                                                        www_authenticate
                                                    }
                                                => {
                                                    if let Some(www_authenticate) = www_authenticate {
                                                    let www_authenticate = match header::IntoHeaderValue(www_authenticate).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling www_authenticate header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("www_authenticate"),
                                                        www_authenticate
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                CreateShareURLResponse::UnexpectedError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_SHARE_URL_UNEXPECTED_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DeleteFileById - DELETE /files/{fileId}
            &hyper::Method::DELETE if path.matched(paths::ID_FILES_FILEID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FILES_FILEID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FILES_FILEID in set but failed match against \"{}\"", path, paths::REGEX_FILES_FILEID.as_str())
                    );

                let param_file_id = match percent_encoding::percent_decode(path_params["fileId"].as_bytes()).decode_utf8() {
                    Ok(param_file_id) => match param_file_id.parse::<uuid::Uuid>() {
                        Ok(param_file_id) => param_file_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter fileId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["fileId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.delete_file_by_id(
                                            param_file_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DeleteFileByIdResponse::DeleteFileSuccessResponseToAValidRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DELETE_FILE_BY_ID_DELETE_FILE_SUCCESS_RESPONSE_TO_A_VALID_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                DeleteFileByIdResponse::AuthenticationInformationIsMissingOrInvalid
                                                    {
                                                        www_authenticate
                                                    }
                                                => {
                                                    if let Some(www_authenticate) = www_authenticate {
                                                    let www_authenticate = match header::IntoHeaderValue(www_authenticate).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling www_authenticate header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("www_authenticate"),
                                                        www_authenticate
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DownloadFile - GET /files/{fileId}/download
            &hyper::Method::GET if path.matched(paths::ID_FILES_FILEID_DOWNLOAD) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FILES_FILEID_DOWNLOAD
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FILES_FILEID_DOWNLOAD in set but failed match against \"{}\"", path, paths::REGEX_FILES_FILEID_DOWNLOAD.as_str())
                    );

                let param_file_id = match percent_encoding::percent_decode(path_params["fileId"].as_bytes()).decode_utf8() {
                    Ok(param_file_id) => match param_file_id.parse::<uuid::Uuid>() {
                        Ok(param_file_id) => param_file_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter fileId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["fileId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_token = query_params.iter().filter(|e| e.0 == "token").map(|e| e.1.to_owned())
                    .nth(0);
                let param_token = match param_token {
                    Some(param_token) => {
                        let param_token =
                            <String as std::str::FromStr>::from_str
                                (&param_token);
                        match param_token {
                            Ok(param_token) => Some(param_token),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter token - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter token")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.download_file(
                                            param_file_id,
                                            param_token,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DownloadFileResponse::DownloadFileSuccessResponseToAValidRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/octet-stream")
                                                            .expect("Unable to create Content-Type header for DOWNLOAD_FILE_DOWNLOAD_FILE_SUCCESS_RESPONSE_TO_A_VALID_REQUEST"));
                                                    let body = body.0;
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetFileById - GET /files/{fileId}
            &hyper::Method::GET if path.matched(paths::ID_FILES_FILEID) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FILES_FILEID
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FILES_FILEID in set but failed match against \"{}\"", path, paths::REGEX_FILES_FILEID.as_str())
                    );

                let param_file_id = match percent_encoding::percent_decode(path_params["fileId"].as_bytes()).decode_utf8() {
                    Ok(param_file_id) => match param_file_id.parse::<uuid::Uuid>() {
                        Ok(param_file_id) => param_file_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter fileId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["fileId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.get_file_by_id(
                                            param_file_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetFileByIdResponse::DescribeFileSuccessResponseToAValidRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_FILE_BY_ID_DESCRIBE_FILE_SUCCESS_RESPONSE_TO_A_VALID_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                GetFileByIdResponse::AuthenticationInformationIsMissingOrInvalid
                                                    {
                                                        www_authenticate
                                                    }
                                                => {
                                                    if let Some(www_authenticate) = www_authenticate {
                                                    let www_authenticate = match header::IntoHeaderValue(www_authenticate).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling www_authenticate header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("www_authenticate"),
                                                        www_authenticate
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                GetFileByIdResponse::UnexpectedError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_FILE_BY_ID_UNEXPECTED_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ListFiles - GET /files
            &hyper::Method::GET if path.matched(paths::ID_FILES) => {
                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_limit = query_params.iter().filter(|e| e.0 == "limit").map(|e| e.1.to_owned())
                    .nth(0);
                let param_limit = match param_limit {
                    Some(param_limit) => {
                        let param_limit =
                            <i32 as std::str::FromStr>::from_str
                                (&param_limit);
                        match param_limit {
                            Ok(param_limit) => Some(param_limit),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter limit - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter limit")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.list_files(
                                            param_limit,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ListFilesResponse::APagedArrayOfFiles
                                                    {
                                                        body,
                                                        x_next
                                                    }
                                                => {
                                                    if let Some(x_next) = x_next {
                                                    let x_next = match header::IntoHeaderValue(x_next).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling x_next header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("x-next"),
                                                        x_next
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_FILES_A_PAGED_ARRAY_OF_FILES"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                ListFilesResponse::AuthenticationInformationIsMissingOrInvalid
                                                    {
                                                        www_authenticate
                                                    }
                                                => {
                                                    if let Some(www_authenticate) = www_authenticate {
                                                    let www_authenticate = match header::IntoHeaderValue(www_authenticate).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling www_authenticate header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("www_authenticate"),
                                                        www_authenticate
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                ListFilesResponse::UnexpectedError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_FILES_UNEXPECTED_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // RecoverFile - POST /files/{fileId}/recover
            &hyper::Method::POST if path.matched(paths::ID_FILES_FILEID_RECOVER) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FILES_FILEID_RECOVER
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FILES_FILEID_RECOVER in set but failed match against \"{}\"", path, paths::REGEX_FILES_FILEID_RECOVER.as_str())
                    );

                let param_file_id = match percent_encoding::percent_decode(path_params["fileId"].as_bytes()).decode_utf8() {
                    Ok(param_file_id) => match param_file_id.parse::<uuid::Uuid>() {
                        Ok(param_file_id) => param_file_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter fileId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["fileId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.recover_file(
                                            param_file_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RecoverFileResponse::SuccessfulRecoverDeletedFileRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RECOVER_FILE_SUCCESSFUL_RECOVER_DELETED_FILE_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                RecoverFileResponse::AuthenticationInformationIsMissingOrInvalid
                                                    {
                                                        www_authenticate
                                                    }
                                                => {
                                                    if let Some(www_authenticate) = www_authenticate {
                                                    let www_authenticate = match header::IntoHeaderValue(www_authenticate).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling www_authenticate header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("www_authenticate"),
                                                        www_authenticate
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                RecoverFileResponse::UnexpectedError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RECOVER_FILE_UNEXPECTED_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UploadFile - POST /files/{fileId}/upload
            &hyper::Method::POST if path.matched(paths::ID_FILES_FILEID_UPLOAD) => {
                // Path parameters
                let path: &str = &uri.path().to_string();
                let path_params =
                    paths::REGEX_FILES_FILEID_UPLOAD
                    .captures(&path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE FILES_FILEID_UPLOAD in set but failed match against \"{}\"", path, paths::REGEX_FILES_FILEID_UPLOAD.as_str())
                    );

                let param_file_id = match percent_encoding::percent_decode(path_params["fileId"].as_bytes()).decode_utf8() {
                    Ok(param_file_id) => match param_file_id.parse::<uuid::Uuid>() {
                        Ok(param_file_id) => param_file_id,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter fileId: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["fileId"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.upload_file(
                                            param_file_id,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().to_string().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UploadFileResponse::SuccessfulFileUploadRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPLOAD_FILE_SUCCESSFUL_FILE_UPLOAD_REQUEST"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                                UploadFileResponse::NullResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(201).expect("Unable to turn 201 into a StatusCode");
                                                },
                                                UploadFileResponse::AuthenticationInformationIsMissingOrInvalid
                                                    {
                                                        www_authenticate
                                                    }
                                                => {
                                                    if let Some(www_authenticate) = www_authenticate {
                                                    let www_authenticate = match header::IntoHeaderValue(www_authenticate).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Ok(Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling www_authenticate header - {}", e)))
                                                                    .expect("Unable to create Internal Server Error for invalid response header"))
                                                        }
                                                    };

                                                    response.headers_mut().insert(
                                                        HeaderName::from_static("www_authenticate"),
                                                        www_authenticate
                                                    );
                                                    }
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                UploadFileResponse::UnexpectedError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(0).expect("Unable to turn 0 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPLOAD_FILE_UNEXPECTED_ERROR"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_FILES) => method_not_allowed(),
            _ if path.matched(paths::ID_FILES_FILEID) => method_not_allowed(),
            _ if path.matched(paths::ID_FILES_FILEID_DOWNLOAD) => method_not_allowed(),
            _ if path.matched(paths::ID_FILES_FILEID_RECOVER) => method_not_allowed(),
            _ if path.matched(paths::ID_FILES_FILEID_SHARE) => method_not_allowed(),
            _ if path.matched(paths::ID_FILES_FILEID_UPLOAD) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match request.method() {
            // CreateFile - POST /files
            &hyper::Method::POST if path.matched(paths::ID_FILES) => Some("CreateFile"),
            // CreateShareURL - GET /files/{fileId}/share
            &hyper::Method::GET if path.matched(paths::ID_FILES_FILEID_SHARE) => Some("CreateShareURL"),
            // DeleteFileById - DELETE /files/{fileId}
            &hyper::Method::DELETE if path.matched(paths::ID_FILES_FILEID) => Some("DeleteFileById"),
            // DownloadFile - GET /files/{fileId}/download
            &hyper::Method::GET if path.matched(paths::ID_FILES_FILEID_DOWNLOAD) => Some("DownloadFile"),
            // GetFileById - GET /files/{fileId}
            &hyper::Method::GET if path.matched(paths::ID_FILES_FILEID) => Some("GetFileById"),
            // ListFiles - GET /files
            &hyper::Method::GET if path.matched(paths::ID_FILES) => Some("ListFiles"),
            // RecoverFile - POST /files/{fileId}/recover
            &hyper::Method::POST if path.matched(paths::ID_FILES_FILEID_RECOVER) => Some("RecoverFile"),
            // UploadFile - POST /files/{fileId}/upload
            &hyper::Method::POST if path.matched(paths::ID_FILES_FILEID_UPLOAD) => Some("UploadFile"),
            _ => None,
        }
    }
}
