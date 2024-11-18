use std::{fmt::Display, str::FromStr};

use http::header::{InvalidHeaderName, InvalidHeaderValue};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HeaderSerDeError {
	#[error("Invalid header type used")]
	InvalidType,
	#[error("Error Parsing Value to json")]
	InvalidJsonValue(#[from] serde_json::Error),
	#[error("Serde Parsing Error: {0}")]
	SerdeParsingError(String),
	#[error("Header Name is invalid")]
	InvalidHeaderName(#[from] InvalidHeaderName),
	#[error("Header Value is invalid")]
	InvalidHeaderValue(#[from] InvalidHeaderValue),
}

impl serde::ser::Error for HeaderSerDeError {
	fn custom<T: Display>(msg: T) -> Self {
		HeaderSerDeError::SerdeParsingError(msg.to_string())
	}
}

type Result<T> = std::result::Result<T, HeaderSerDeError>;

pub fn to_headers<T>(value: &T) -> Result<HeaderMap>
where
	T: Serialize,
{
	let mut serializer = HeaderMapSerializer::new();
	value.serialize(&mut serializer)?;
	Ok(serializer.into_inner())
}


struct HeaderMapSerializer {
	map: HeaderMap,
}

impl HeaderMapSerializer {
	fn new() -> Self {
		Self {
			map: HeaderMap::new(),
		}
	}

	fn into_inner(self) -> HeaderMap {
		self.map
	}
}

impl<'a> Serializer for &'a mut HeaderMapSerializer {
	type Ok = ();
	type Error = HeaderSerDeError;

	// The other associated types can be `Unsupported`.
	// We're only implementing map serialization for this example.
	type SerializeSeq = serde::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeMap = serde::ser::Impossible<Self::Ok, Self::Error>;
	type SerializeStruct = Self;
	type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

	fn serialize_struct(
		self,
		__name: &'static str,
		_: usize,
	) -> Result<Self::SerializeStruct> {
		Ok(self)
	}

	// Implement other serialization functions if needed (like `serialize_bool`,
	// `serialize_i32`, etc.) These should convert the value to a string and insert
	// it into the HeaderMap.
	fn serialize_map(
		self,
		_: Option<usize>,
	) -> Result<Self::SerializeMap> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_bool(
		self,
		_v: bool,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_i8(
		self,
		_v: i8,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_i16(
		self,
		_v: i16,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_i32(
		self,
		_v: i32,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_i64(
		self,
		_v: i64,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_u8(
		self,
		_v: u8,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_u16(
		self,
		_v: u16,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_u32(
		self,
		_v: u32,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_u64(
		self,
		_v: u64,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_f32(
		self,
		_v: f32,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_f64(
		self,
		_v: f64,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_char(
		self,
		_v: char,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_str(
		self,
		_v: &str,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_bytes(
		self,
		_v: &[u8],
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_none(self) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_some<T>(
		self,
		_value: &T,
	) -> Result<Self::Ok>
	where
		T: ?Sized + Serialize,
	{
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_unit(self) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_unit_struct(
		self,
		_name: &'static str,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
	) -> Result<Self::Ok> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_newtype_struct<T>(
		self,
		_name: &'static str,
		_value: &T,
	) -> Result<Self::Ok>
	where
		T: ?Sized + Serialize,
	{
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_newtype_variant<T>(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_value: &T,
	) -> Result<Self::Ok>
	where
		T: ?Sized + Serialize,
	{
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_seq(
		self,
		_len: Option<usize>,
	) -> Result<Self::SerializeSeq> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_tuple(
		self,
		_len: usize,
	) -> Result<Self::SerializeTuple> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleStruct> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeTupleVariant> {
		Err(HeaderSerDeError::InvalidType)
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		_len: usize,
	) -> Result<Self::SerializeStructVariant> {
		Err(HeaderSerDeError::InvalidType)
	}
}

// impl SerializeMap for HeaderMapSerializeMap {
// 	type Ok = HeaderMap;
// 	type Error = serde::ser::Error;
//
// 	fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<()> {
// 		// Serialize the key as a string
// 		self.current_key = Some(key.serialize(serde_json::Serializer)?);
// 		Ok(())
// 	}
//
// 	fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) ->
// Result<()> { 		if let Some(ref key) = self.current_key {
// 			// Check if the value is a nested struct
// 			let serialized_value = match serde_json::to_string(value) {
// 				Ok(json_value) => json_value, // Use JSON for nested struct
// 				Err(_) => value.serialize(serde_json::Serializer)?.to_string(),
// 			};
//
// 			self.map.insert(key.clone(),
// HeaderValue::from_str(&serialized_value).unwrap()); 			self.current_key =
// None; 		}
// 		Ok(())
// 	}
//
// 	fn end(self) -> Result<Self::Ok> {
// 		Ok(self.map)
// 	}
//}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> SerializeStruct for &'a mut HeaderMapSerializer {
	type Ok = ();
	type Error = HeaderSerDeError;

	fn serialize_field<T>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<()>
	where
		T: ?Sized + Serialize,
	{
		let header_key = key.replace("_", "-");
		let key = HeaderName::from_str(&header_key)?;
		let header_value = serde_json::to_string(value)?;
		// TODO: improve the logic here
		let value = if &header_value[0..1] == "\"" {
			HeaderValue::from_str(&header_value[1..header_value.len() -1])?
		} else {
			HeaderValue::from_str(&header_value)?
		};
		self.map.insert(key, value);
		Ok(())
	}

	fn end(self) -> Result<()> {
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use reqwest::header::{HeaderMap, HeaderValue};
	use serde::Serialize;

	use super::*;

	#[derive(Serialize)]
	struct MyStruct {
		field1: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		optional_field: Option<String>,
		nested: NestedStruct,
		integer: u32,
	}

	#[derive(Serialize)]
	struct NestedStruct {
		inner_field: String,
		#[serde(skip_serializing_if = "Option::is_none")]
		optional_nested_field: Option<String>,
	}

	#[test]
	fn test_header_map_serializer_with_optional_fields() {
		// Create an instance of the struct with `None` values for optional fields
		let my_struct = MyStruct {
			field1: "value1".to_string(),
			optional_field: None,
			nested: NestedStruct {
				inner_field: "inner_value".into(),
				optional_nested_field: None,
			},
			integer: 3
		};

		// Serialize into HeaderMap
		let headers: HeaderMap = to_headers(&my_struct).unwrap();

		// Check that the top-level field1 is serialized correctly
		assert_eq!(
			headers.get("field1").unwrap(),
			&HeaderValue::from_str("value1").unwrap()
		);
		assert_eq!(
			headers.get("integer").unwrap(),
			&HeaderValue::from_str("3").unwrap()
		);

		// Ensure `optional_field` is not present since it's None
		assert!(headers.get("optional_field").is_none());

		// Check that the nested struct is serialized as JSON
		let nested_json = headers.get("nested").unwrap().to_str().unwrap();
		assert_eq!(nested_json, r#"{"inner_field":"inner_value"}"#);

		// Ensure `optional_nested_field` is not present in the JSON since it's None
		assert!(!nested_json.contains("optional_nested_field"));

	}
}
