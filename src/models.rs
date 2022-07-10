#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Error {
    #[serde(rename = "code")]
    pub code: i32,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "error_code")]
    pub error_code: String,

    #[serde(rename = "message")]
    pub message: String,

}

impl Error {
    pub fn new(code: i32, error_code: String, message: String, ) -> Error {
        Error {
            code: code,
            error_code: error_code,
            message: message,
        }
    }
}

/// Converts the Error value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];

        params.push("code".to_string());
        params.push(self.code.to_string());


        params.push("error_code".to_string());
        params.push(self.error_code.to_string());


        params.push("message".to_string());
        params.push(self.message.to_string());

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Error value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Error {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub code: Vec<i32>,
            pub error_code: Vec<String>,
            pub message: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Error".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "code" => intermediate_rep.code.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "error_code" => intermediate_rep.error_code.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "message" => intermediate_rep.message.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Error".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Error {
            code: intermediate_rep.code.into_iter().next().ok_or("code missing in Error".to_string())?,
            error_code: intermediate_rep.error_code.into_iter().next().ok_or("error_code missing in Error".to_string())?,
            message: intermediate_rep.message.into_iter().next().ok_or("message missing in Error".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Error> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Error>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Error>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Error - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Error> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Error as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Error - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct File {
    #[serde(rename = "FileId")]
    pub file_id: uuid::Uuid,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,

    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "UploadTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub upload_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "DeletionTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletion_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "RecoverTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub recover_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "ExpirationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub expiration_time: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub owner_id: Option<uuid::Uuid>,

    #[serde(rename = "ShareURL")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub share_url: Option<String>,

    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<Vec<String>>,

}

impl File {
    pub fn new(file_id: uuid::Uuid, ) -> File {
        File {
            file_id: file_id,
            status: None,
            creation_time: None,
            upload_time: None,
            deletion_time: None,
            recover_time: None,
            expiration_time: None,
            owner_id: None,
            share_url: None,
            tags: None,
        }
    }
}

/// Converts the File value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for File {
    fn to_string(&self) -> String {
        let mut params: Vec<String> = vec![];
        // Skipping FileId in query parameter serialization


        if let Some(ref status) = self.status {
            params.push("Status".to_string());
            params.push(status.to_string());
        }

        // Skipping CreationTime in query parameter serialization

        // Skipping UploadTime in query parameter serialization

        // Skipping DeletionTime in query parameter serialization

        // Skipping RecoverTime in query parameter serialization

        // Skipping ExpirationTime in query parameter serialization

        // Skipping OwnerId in query parameter serialization


        if let Some(ref share_url) = self.share_url {
            params.push("ShareURL".to_string());
            params.push(share_url.to_string());
        }


        if let Some(ref tags) = self.tags {
            params.push("Tags".to_string());
            params.push(tags.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",").to_string());
        }

        params.join(",").to_string()
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a File value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for File {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        #[derive(Default)]
        // An intermediate representation of the struct to use for parsing.
        struct IntermediateRep {
            pub file_id: Vec<uuid::Uuid>,
            pub status: Vec<String>,
            pub creation_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub upload_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub deletion_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub recover_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub expiration_time: Vec<chrono::DateTime::<chrono::Utc>>,
            pub owner_id: Vec<uuid::Uuid>,
            pub share_url: Vec<String>,
            pub tags: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',').into_iter();
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing File".to_string())
            };

            if let Some(key) = key_result {
                match key {
                    "FileId" => intermediate_rep.file_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "Status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "CreationTime" => intermediate_rep.creation_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "UploadTime" => intermediate_rep.upload_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "DeletionTime" => intermediate_rep.deletion_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "RecoverTime" => intermediate_rep.recover_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ExpirationTime" => intermediate_rep.expiration_time.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "OwnerId" => intermediate_rep.owner_id.push(<uuid::Uuid as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "ShareURL" => intermediate_rep.share_url.push(<String as std::str::FromStr>::from_str(val).map_err(|x| format!("{}", x))?),
                    "Tags" => return std::result::Result::Err("Parsing a container in this style is not supported in File".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing File".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(File {
            file_id: intermediate_rep.file_id.into_iter().next().ok_or("FileId missing in File".to_string())?,
            status: intermediate_rep.status.into_iter().next(),
            creation_time: intermediate_rep.creation_time.into_iter().next(),
            upload_time: intermediate_rep.upload_time.into_iter().next(),
            deletion_time: intermediate_rep.deletion_time.into_iter().next(),
            recover_time: intermediate_rep.recover_time.into_iter().next(),
            expiration_time: intermediate_rep.expiration_time.into_iter().next(),
            owner_id: intermediate_rep.owner_id.into_iter().next(),
            share_url: intermediate_rep.share_url.into_iter().next(),
            tags: intermediate_rep.tags.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<File> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<File>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<File>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for File - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<File> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <File as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into File - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

