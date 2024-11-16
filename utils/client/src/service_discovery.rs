use core::future::Future;
use std::{collections::HashMap, sync::Arc, task::Poll};

use pin_project_lite::pin_project;
use reqwest::Client;
use thiserror::Error;
use tower::Service;
use oasbi::common::NfType;

use crate::{
	ServiceError,
	nrf_client::{NrfClient, NrfError},
};


pub struct NFResolverService<S> {
	nrf_client: Arc<NrfClient>,
	service: S,
}

//impl<Request, S> Service<Request> for NFResolverService<S>
//where
//	S: Service<Request>,
//{
//	type Error = ServiceError;
//	type Future = ResponseFuture<S::Future>;
//}

pin_project! {
	pub struct ResponseFuture<F1, F2> {
		#[pin]
		inner: F1,
		#[pin]
		discover: Option<F2>
	}
}

//impl<F1, F2, Resp, E> Future for ResponseFuture<F1, F2>
//where
//	F1: Future<Output = Result<Resp, E>>,
//	F2: Future<Output = Result<, NrfError>>,
//	E: std::error::Error,
//{
//	type Output = Result<Resp, ServiceError<NrfError, E>>;
//	fn poll(
//		self: std::pin::Pin<&mut Self>,
//		cx: &mut std::task::Context<'_>,
//	) -> std::task::Poll<Self::Output> {
//		Poll::Pending
//	}
//}
