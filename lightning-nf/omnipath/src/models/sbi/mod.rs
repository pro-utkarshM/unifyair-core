use std::backtrace::Backtrace;
use oasbi::common::error;
use oasbi::common::error::ConversionError;
use thiserror::Error;


pub mod nrf;


#[derive(Error, Debug)]
pub enum ModelBuildError {
	#[error("The model cannot be converted: {0}")]
	InvalidConversion(#[from] ConversionError),

}