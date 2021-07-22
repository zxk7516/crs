use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::errors::MyError;
use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, ready};

//#[derive(Clone)]
//pub struct TokenParser<S, 'a> {
//    service: S,
//    tokentool: &'a crate::TokenTool,
//}

//impl<S: Service<Req>, Req, 'a> Service<Req> for TokenParser<S, 'a> {
//    type Response = S::Response;
//    type Error = MyError<S::Error>;
//}

//pub struct TokenParserTransformer {
//    jwt_parser: TokenParser,
//}

//impl TokenParserTransformer {}

//impl<S: Service<Seq>, Seq> Transform<S, Deq> for TokenParserTransformer
//where
//    Seq: ServiceRequest,
//{
//    type Response = S::Response;
//    type Error = Error;
//    type InitError = Error;
//    type Transform = TokenParserMiddleware<S>;
//    type Future = Ready<Self::Transform, Self::InitError>;
//
//    fn new_transform(&self, service: S) -> Self::Future {
//        ok(TokenParserMiddleware {
//            service: service,
//            jwt_parser: self.jwt_parser.clone(),
//        })
//    }
//}

//pub struct TokenParserMiddleware<S> {
//    service: S,
//    jwt_parser: TokenParser,
//}

//impl<S, B> Service for TokenParserTransformer<S>
//where
//    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//    S::Future: 'static,
//    B: 'static,
//{
//    type Request = ServiceRequest;
//    type Response = ServiceResponse;
//    type Error = Error;
//    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
//}
