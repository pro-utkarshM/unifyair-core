use std::{fmt, str::FromStr};

use http::header::HeaderValue;
use mediatype::{
	MediaType,
	names::{APPLICATION, JSON, JSON_PATCH, x_::WWW_FORM_URLENCODED},
};
use thiserror::Error;

pub const APP_JSON: MediaType<'static> = MediaType::new(APPLICATION, JSON);
pub const APP_FORM: MediaType<'static> = MediaType::new(APPLICATION, WWW_FORM_URLENCODED);
pub const APP_PATCH_JSON: MediaType<'static> =
	MediaType::from_parts(APPLICATION, JSON_PATCH, Some(JSON), &[]);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ContentType {
	APP_JSON,
	APP_FORM,
	APP_PATCH_JSON,
}
/// Custom error for parsing ContentType from a string
#[derive(Error, Debug)]
pub enum ContentTypeParseError {
	#[error("InvalidContentType: {0}")]
	InvalidContentType(String),
	#[error("MediaTypeError: {1}")]
	MediaTypeError(#[source] mediatype::MediaTypeError, String),
}

macro_rules! generate_content_type_impl {
    ($($variant:ident, $media_type:ident, $str_value:expr);+ $(;)?) => {
        impl ContentType {
            /// Returns the string representation of the enum variant.
            pub const fn to_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $str_value),+
                }
            }

            /// Returns the MediaType constant associated with the enum variant.
            pub const fn to_mediatype(&self) -> MediaType<'static> {
                match self {
                    $(Self::$variant => $media_type),+
                }
            }

            /// Converts the enum variant to a `HeaderValue`.
            pub const fn to_header_value(&self) -> HeaderValue {
                HeaderValue::from_static(self.to_str())
            }
        }

		impl fmt::Display for ContentType {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				write!(f, "{}", self.to_str())
			}
		}

        impl FromStr for ContentType {
            type Err = ContentTypeParseError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let m = MediaType::parse(s)
                    .map_err(|e| ContentTypeParseError::MediaTypeError(e, s.to_owned()))?;
                // Match the MediaType with corresponding ContentType variant
                let content_type = match m {
                    $(_ if m == $variant => ContentType::$variant,)*
                    _ => return Err(ContentTypeParseError::InvalidContentType(s.to_owned())),
                };
                Ok(content_type)
            }
        }

    };
}

generate_content_type_impl!(
	APP_JSON, APP_JSON, "application/json";
	APP_FORM, APP_FORM, "application/x-www-form-urlencoded";
	APP_PATCH_JSON, APP_PATCH_JSON, "application/json-patch+json";
);
