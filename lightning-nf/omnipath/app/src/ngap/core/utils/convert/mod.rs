mod mcc_mnc_plmnid;
mod tac;
mod snssai;
mod transform;

use nonempty::NonEmpty;
use oasbi::common::error::ConversionError;

pub(crate) struct Element<T>(T);

pub fn try_convert<T, U>(value: T) -> Result<U, ConversionError>
where
	Element<U>: TryFrom<Element<T>, Error = ConversionError>,
{
	let element = Element(value);
	let converted = Element::try_from(element)?;
	Ok(converted.0)
}

pub fn convert<T, U>(value: T) -> U
where
	Element<U>: From<Element<T>>,
{
	let element = Element(value);
	let converted = Element::from(element);
	converted.0
}

pub fn transform_nonempty<T, U>(
	value: &NonEmpty<T>,
	f: impl Fn(&T) -> U,
) -> NonEmpty<U> {
	let head = f(&value.head);
	let tail = value.tail().iter().map(f).collect::<Vec<_>>();
	NonEmpty { head, tail }
}
