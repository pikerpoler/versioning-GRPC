mod wrappers;

use tonic::transport::Channel;
use tonic::{ IntoRequest};


pub mod api{
    pub mod v1{
        pub const VERSION_NAME: &'static str = "V1";
        include!("api.v1.rs");
    }

    pub mod v2{
        pub const VERSION_NAME: &'static str = "V2";
        pub use super::v1::*;
        include!("api.v2.rs");

        impl From<super::v1::SumRequest> for SumRequest{
            fn from(value: super::v1::SumRequest) -> Self {
                match value.vector{
                    Some(vector)=>SumRequest{vectors: vec![vector]},
                    None => SumRequest{vectors: vec![]}
                }
            }
        }
        impl From<SumRequest> for super::v1::SumRequest{
            fn from(value: SumRequest) -> Self {
                        assert_eq!(value.vectors.len(), 1);
                        super::v1::SumRequest{
                            vector: Some(value.vectors[0].clone()),
                    }
                }
            }


        impl From<super::v1::SumResponse> for SumResponse{
            fn from(value: super::v1::SumResponse) -> Self {
                SumResponse{
                    sum: vec![value.sum]
                }
            }
        }
        impl From<SumResponse> for super::v1::SumResponse{
            fn from(value: SumResponse) -> Self {
                assert_eq!(value.sum.len(), 1);
                super::v1::SumResponse{
                    sum: value.sum[0],
                }
            }
        }

    }

    pub mod inner{
        use tonic::{async_trait, Request, Response, Status};
        pub use super::v2::{PrintResponse, PrintRequest, SumRequest, SumResponse};
        include!("api.inner.rs");
        pub use vector_service_server::VectorService;

        macro_rules! impl_vector_service {
            ($version:ident) => {
                #[async_trait]
                impl<T> $version::vector_service_server::VectorService for T
                where
                    T: VectorService,
                {
                    async fn print(
                        &self,
                        request: Request<$version::PrintRequest>,
                    ) -> Result<Response<$version::PrintResponse>, Status> {
                        let tmp = $version::VERSION_NAME;
                        println!("rerouting upsert from {tmp:?}");
                        let (metadata, extensions, inner_request) = request.into_parts();
                        let inner_request = PrintRequest::from(inner_request);
                        let request = Request::from_parts(metadata, extensions, inner_request);
                        let response = VectorService::print(self, request).await?;
                        Ok(Response::new($version::PrintResponse::from(
                            response.into_inner(),
                        )))
                    }

                    async fn sum(
                        &self,
                        request: Request<$version::SumRequest>,
                    ) -> Result<Response<$version::SumResponse>, Status> {
                        let tmp = $version::VERSION_NAME;
                        println!("rerouting delete from {tmp:?}");
                        let (metadata, extensions, inner_request) = request.into_parts();
                        let inner_request = SumRequest::from(inner_request);
                        let request = Request::from_parts(metadata, extensions, inner_request);
                        let response = VectorService::sum(self, request).await?;
                        Ok(Response::new($version::SumResponse::from(
                            response.into_inner(),
                        )))
                    }

                }
            };
        }
        pub use super::v1;
        pub use super::v2;
        impl_vector_service!(v1);
        impl_vector_service!(v2);
    }
}

pub use api::inner::*;
pub use api::v1::vector_service_server::VectorServiceServer as VectorServiceServerV1;
pub use api::v1::vector_service_server::VectorServiceServer as VectorServiceServerV2;
pub use api::inner::vector_service_server::VectorService;


#[derive(Clone)]
// Define the Client struct with versioned clients
pub struct MultiVersionedClient {
    v1: api::v1::vector_service_client::VectorServiceClient<Channel>,
    v2: api::v2::vector_service_client::VectorServiceClient<Channel>,
}

impl MultiVersionedClient {
    pub async fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let v1 = api::v1::vector_service_client::VectorServiceClient::connect(format!(
            "http://localhost:{}",
            port
        ))
        .await?;
        let v2 = api::v2::vector_service_client::VectorServiceClient::connect(format!(
            "http://localhost:{}",
            port
        ))
        .await?;

        Ok(MultiVersionedClient { v1, v2 })
    }
    pub async fn print_v1(&self, request: PrintRequest) -> PrintResponse {
        let inner_request = api::v1::PrintRequest::from(request);
        let inner_response = self
            .v1
            .clone()
            .print(inner_request)
            .await
            .unwrap()
            .into_inner();
        PrintResponse::from(inner_response)
    }
    pub async fn print_v2(&self, request: PrintRequest) -> PrintResponse {
        let inner_request = api::v2::PrintRequest::from(request);
        let inner_response = self
            .v2
            .clone()
            .print(inner_request)
            .await
            .unwrap()
            .into_inner();
        PrintResponse::from(inner_response)
    }
    pub async fn sum_v1(&self, request: SumRequest) -> SumResponse {
        let inner_request = api::v1::SumRequest::from(request);
        let inner_response = self
            .v1
            .clone()
            .sum(inner_request)
            .await
            .unwrap()
            .into_inner();
        SumResponse::from(inner_response)
    }
    pub async fn sum_v2(&self, request: SumRequest) -> SumResponse {
        let inner_request = api::v2::SumRequest::from(request);
        let inner_response = self
            .v2
            .clone()
            .sum(inner_request)
            .await
            .unwrap()
            .into_inner();
        SumResponse::from(inner_response)
    }
}



// pub fn get_servers<T: vector_service_server::VectorService + std::marker::Sync + std::marker::Send + 'static>(
//     service: T,
// ) -> (
//     api::v1::vector_service_server::VectorServiceServer<T>,
//     api::v2::vector_service_server::VectorServiceServer<T>,
// ) {
//     let arc = Arc::new(service);
//     (
//         api::v1::vector_service_server::VectorServiceServer::from_arc(arc.clone()),
//         api::v2::vector_service_server::VectorServiceServer::from_arc(arc),
//     )
// }
//
// pub enum VersionedVectorServiceServer {
//     V1(
//         api::v1::vector_service_server::VectorServiceServer<
//             Box<dyn api::v1::vector_service_server::VectorService>,
//         >,
//     ),
//     V2(
//         api::v2::vector_service_server::VectorServiceServer<
//             Box<dyn api::v2::vector_service_server::VectorService>,
//         >,
//     ),
// }
//
// impl VersionedVectorServiceServer {
//     pub fn new_v1(service: Box<dyn api::v1::vector_service_server::VectorService>) -> Self {
//         VersionedVectorServiceServer::V1(api::v1::vector_service_server::VectorServiceServer::new(
//             service,
//         ))
//     }
//
//     pub fn new_v2(service: Box<dyn api::v2::vector_service_server::VectorService>) -> Self {
//         VersionedVectorServiceServer::V2(api::v2::vector_service_server::VectorServiceServer::new(
//             service,
//         ))
//     }
// }


// #[async_trait]
// pub trait VectorService {
//     async fn print(&self, request: PrintRequest) -> PrintResponse;
//     async fn sum(&self, request: SumRequest) -> SumResponse;
// }

// #[async_trait]
// impl<T> api::v1::vector_service_server::VectorService for T
// where
//     T: VectorService + std::marker::Sync + std::marker::Send + 'static,
// {
//     async fn print(
//         &self,
//         request: tonic::Request<api::v1::PrintRequest>,
//     ) -> Result<tonic::Response<api::v1::PrintResponse>, tonic::Status> {
//         let inner_request = PrintRequest::from(request.into_inner());
//         let inner_response = VectorService::print(self, inner_request).await;
//         Ok(tonic::Response::new(api::v1::PrintResponse::from(
//             inner_response,
//         )))
//     }
//     async fn sum(
//         &self,
//         request: tonic::Request<api::v1::SumRequest>,
//     ) -> Result<tonic::Response<api::v1::SumResponse>, tonic::Status> {
//         let inner_request = SumRequest::from(request.into_inner());
//         let inner_response = VectorService::sum(self, inner_request).await;
//         Ok(tonic::Response::new(api::v1::SumResponse::from(
//             inner_response,
//         )))
//     }
// }
//
// #[async_trait]
// impl VectorService for Box<dyn api::v1::vector_service_server::VectorService + 'static> {
//     async fn print(&self, request: PrintRequest) -> PrintResponse {
//         let inner = api::v1::PrintRequest::from(request).into_request();
//         let response = self.as_ref().print(inner).await.unwrap().into_inner();
//         PrintResponse::from(response)
//     }
//
//     async fn sum(&self, request: SumRequest) -> SumResponse {
//         let inner = api::v1::SumRequest::from(request).into_request();
//         let response = self.as_ref().sum(inner).await.unwrap().into_inner();
//         SumResponse::from(response)
//     }
// }
//
// #[async_trait]
// impl<T> api::v2::vector_service_server::VectorService for T
// where
//     T: VectorService + std::marker::Sync + std::marker::Send + 'static,
// {
//     async fn print(
//         &self,
//         request: tonic::Request<api::v2::PrintRequest>,
//     ) -> Result<tonic::Response<api::v2::PrintResponse>, tonic::Status> {
//         let inner_request = PrintRequest::from(request.into_inner());
//         let inner_response = VectorService::print(self, inner_request).await;
//         Ok(tonic::Response::new(api::v2::PrintResponse::from(
//             inner_response,
//         )))
//     }
//     async fn sum(
//         &self,
//         request: tonic::Request<api::v2::SumRequest>,
//     ) -> Result<tonic::Response<api::v2::SumResponse>, tonic::Status> {
//         let inner_request = SumRequest::from(request.into_inner());
//         let inner_response = VectorService::sum(self, inner_request).await;
//         Ok(tonic::Response::new(api::v2::SumResponse::from(
//             inner_response,
//         )))
//     }
// }
//
// #[async_trait]
// impl VectorService for Box<dyn api::v2::vector_service_server::VectorService + 'static> {
//     async fn print(&self, request: PrintRequest) -> PrintResponse {
//         let inner = api::v2::PrintRequest::from(request).into_request();
//         let response = self.as_ref().print(inner).await.unwrap().into_inner();
//         PrintResponse::from(response)
//     }
//
//     async fn sum(&self, request: SumRequest) -> SumResponse {
//         let inner = api::v2::SumRequest::from(request).into_request();
//         let response = self.as_ref().sum(inner).await.unwrap().into_inner();
//         SumResponse::from(response)
//     }
// }
