use tonic::transport::Channel;
use tonic::{async_trait, IntoRequest};

#[path = "api.v1.rs"]
#[rustfmt::skip]
mod api_v1;

#[path = "api.v2.rs"]
#[rustfmt::skip]
mod api_v2;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportedVersion {
    V1,
    V2,
}

#[derive(Clone)]
// Define the Client struct with versioned clients
pub struct MultiVersionedClient {
    v1: api_v1::vector_service_client::VectorServiceClient<Channel>,
    v2: api_v2::vector_service_client::VectorServiceClient<Channel>,
}

impl MultiVersionedClient {
    pub async fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let v1 = api_v1::vector_service_client::VectorServiceClient::connect(format!(
            "http://localhost:{}",
            port
        ))
        .await?;
        let v2 = api_v2::vector_service_client::VectorServiceClient::connect(format!(
            "http://localhost:{}",
            port
        ))
        .await?;

        Ok(MultiVersionedClient { v1, v2 })
    }
    pub async fn print_v1(&self, request: PrintRequest) -> PrintResponse {
        VectorService::<1>::print(self, request).await
    }
    pub async fn print_v2(&self, request: PrintRequest) -> PrintResponse {
        VectorService::<2>::print(self, request).await
    }
    pub async fn sum_v1(&self, request: SumRequest) -> SumResponse {
        VectorService::<1>::sum(self, request).await
    }
    pub async fn sum_v2(&self, request: SumRequest) -> SumResponse {
        VectorService::<2>::sum(self, request).await
    }
}

#[async_trait]
impl VectorService<1> for MultiVersionedClient {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let inner_request = request.to_v1();
        let inner_response = self
            .v1
            .clone()
            .print(inner_request)
            .await
            .unwrap()
            .into_inner();
        PrintResponse::from_v1(inner_response)
    }

    async fn sum(&self, request: SumRequest) -> SumResponse {
        let inner_request = request.to_v1();
        let inner_response = self
            .v1
            .clone()
            .sum(inner_request)
            .await
            .unwrap()
            .into_inner();
        SumResponse::from_v1(inner_response)
    }
}

#[async_trait]
impl VectorService<2> for MultiVersionedClient {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let inner_request = request.to_v2();
        let inner_response = self
            .v2
            .clone()
            .print(inner_request)
            .await
            .unwrap()
            .into_inner();
        PrintResponse::from_v2(inner_response)
    }

    async fn sum(&self, request: SumRequest) -> SumResponse {
        let inner_request = request.to_v2();
        let inner_response = self
            .v2
            .clone()
            .sum(inner_request)
            .await
            .unwrap()
            .into_inner();
        SumResponse::from_v2(inner_response)
    }
}

pub mod api_versions {
    pub const V1: u8 = 1;
    pub const V2: u8 = 2;
}
pub enum VersionedVectorServiceServer {
    V1(
        api_v1::vector_service_server::VectorServiceServer<
            Box<dyn api_v1::vector_service_server::VectorService>,
        >,
    ),
    V2(
        api_v2::vector_service_server::VectorServiceServer<
            Box<dyn api_v2::vector_service_server::VectorService>,
        >,
    ),
}

impl VersionedVectorServiceServer {
    pub fn new_v1(service: Box<dyn api_v1::vector_service_server::VectorService>) -> Self {
        VersionedVectorServiceServer::V1(api_v1::vector_service_server::VectorServiceServer::new(
            service,
        ))
    }

    pub fn new_v2(service: Box<dyn api_v2::vector_service_server::VectorService>) -> Self {
        VersionedVectorServiceServer::V2(api_v2::vector_service_server::VectorServiceServer::new(
            service,
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub id: String,
    pub values: Vec<f32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintRequest {
    pub vector: Option<Vector>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintResponse {
    pub printed_count: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SumRequest {
    pub vector: Option<Vector>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SumResponse {
    pub sum: f32,
}

impl Vector {
    pub fn to_v1(&self) -> api_v1::Vector {
        api_v1::Vector {
            id: self.id.clone(),
            values: self.values.clone(),
        }
    }
    pub fn to_v2(&self) -> api_v2::Vector {
        api_v2::Vector {
            id: self.id.clone(),
            values: self.values.clone(),
        }
    }
    pub fn from_v1(vector: api_v1::Vector) -> Vector {
        Vector {
            id: vector.id,
            values: vector.values,
        }
    }
    pub fn from_v2(vector: api_v2::Vector) -> Vector {
        Vector {
            id: vector.id,
            values: vector.values,
        }
    }
}

impl PrintRequest {
    pub fn to_v1(&self) -> api_v1::PrintRequest {
        api_v1::PrintRequest {
            vectors: self.vector.as_ref().map(|v| v.to_v1()),
        }
    }
    pub fn to_v2(&self) -> api_v2::PrintRequest {
        api_v2::PrintRequest {
            vectors: self.vector.as_ref().map(|v| v.to_v2()),
        }
    }
    pub fn from_v1(request: api_v1::PrintRequest) -> PrintRequest {
        PrintRequest {
            vector: request.vectors.map(Vector::from_v1),
        }
    }
    pub fn from_v2(request: api_v2::PrintRequest) -> PrintRequest {
        PrintRequest {
            vector: request.vectors.map(Vector::from_v2),
        }
    }
}
// TODO: impl From<> for <struct>
// TODO: unbreaking changes:adding fields
// TODO: breaking changes: changing integer types, fields, removing stuff.. same name different type
impl PrintResponse {
    pub fn to_v1(&self) -> api_v1::PrintResponse {
        api_v1::PrintResponse {
            printed_count: self.printed_count,
        }
    }
    pub fn to_v2(&self) -> api_v2::PrintResponse {
        api_v2::PrintResponse {
            printed_count: self.printed_count,
        }
    }
    pub fn from_v1(response: api_v1::PrintResponse) -> PrintResponse {
        PrintResponse {
            printed_count: response.printed_count,
        }
    }
    pub fn from_v2(response: api_v2::PrintResponse) -> PrintResponse {
        PrintResponse {
            printed_count: response.printed_count,
        }
    }
}

impl SumRequest {
    pub fn to_v1(&self) -> api_v1::SumRequest {
        api_v1::SumRequest {
            vector: self.vector.as_ref().map(|v| v.to_v1()),
        }
    }
    pub fn to_v2(&self) -> api_v2::SumRequest {
        api_v2::SumRequest {
            vector: self.vector.as_ref().map(|v| v.to_v2()),
        }
    }
    pub fn from_v1(request: api_v1::SumRequest) -> SumRequest {
        SumRequest {
            vector: request.vector.map(Vector::from_v1),
        }
    }
    pub fn from_v2(request: api_v2::SumRequest) -> SumRequest {
        SumRequest {
            vector: request.vector.map(Vector::from_v2),
        }
    }
}

impl SumResponse {
    pub fn to_v1(&self) -> api_v1::SumResponse {
        api_v1::SumResponse { sum: self.sum }
    }
    pub fn to_v2(&self) -> api_v2::SumResponse {
        api_v2::SumResponse { sum: self.sum }
    }
    pub fn from_v1(response: api_v1::SumResponse) -> SumResponse {
        SumResponse { sum: response.sum }
    }
    pub fn from_v2(response: api_v2::SumResponse) -> SumResponse {
        SumResponse { sum: response.sum }
    }
}

#[async_trait]
pub trait VectorService<const SV: u8> {
    async fn print(&self, request: PrintRequest) -> PrintResponse;
    async fn sum(&self, request: SumRequest) -> SumResponse;
}

#[async_trait]
impl<T> api_v1::vector_service_server::VectorService for T
where
    T: VectorService<1> + std::marker::Sync + std::marker::Send + 'static,
{
    async fn print(
        &self,
        request: tonic::Request<api_v1::PrintRequest>,
    ) -> Result<tonic::Response<api_v1::PrintResponse>, tonic::Status> {
        let inner_request = PrintRequest::from_v1(request.into_inner());
        let inner_response = VectorService::<1>::print(self, inner_request).await;
        Ok(tonic::Response::new(inner_response.to_v1()))
    }
    async fn sum(
        &self,
        request: tonic::Request<api_v1::SumRequest>,
    ) -> Result<tonic::Response<api_v1::SumResponse>, tonic::Status> {
        let inner_request = SumRequest::from_v1(request.into_inner());
        let inner_response = VectorService::<1>::sum(self, inner_request).await;
        Ok(tonic::Response::new(inner_response.to_v1()))
    }
}

#[async_trait]
impl VectorService<1> for Box<dyn api_v1::vector_service_server::VectorService + 'static> {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let inner = request.to_v1().into_request();
        let response = self.as_ref().print(inner).await.unwrap().into_inner();
        PrintResponse::from_v1(response)
    }

    async fn sum(&self, request: SumRequest) -> SumResponse {
        let inner = request.to_v1().into_request();
        let response = self.as_ref().sum(inner).await.unwrap().into_inner();
        SumResponse::from_v1(response)
    }
}

#[async_trait]
impl<T> api_v2::vector_service_server::VectorService for T
where
    T: VectorService<2> + std::marker::Sync + std::marker::Send + 'static,
{
    async fn print(
        &self,
        request: tonic::Request<api_v2::PrintRequest>,
    ) -> Result<tonic::Response<api_v2::PrintResponse>, tonic::Status> {
        let inner_request = PrintRequest::from_v2(request.into_inner());
        let inner_response = VectorService::<2>::print(self, inner_request).await;
        Ok(tonic::Response::new(inner_response.to_v2()))
    }
    async fn sum(
        &self,
        request: tonic::Request<api_v2::SumRequest>,
    ) -> Result<tonic::Response<api_v2::SumResponse>, tonic::Status> {
        let inner_request = SumRequest::from_v2(request.into_inner());
        let inner_response = VectorService::<2>::sum(self, inner_request).await;
        Ok(tonic::Response::new(inner_response.to_v2()))
    }
}

#[async_trait]
impl VectorService<2> for Box<dyn api_v2::vector_service_server::VectorService + 'static> {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let inner = request.to_v2().into_request();
        let response = self.as_ref().print(inner).await.unwrap().into_inner();
        PrintResponse::from_v2(response)
    }

    async fn sum(&self, request: SumRequest) -> SumResponse {
        let inner = request.to_v2().into_request();
        let response = self.as_ref().sum(inner).await.unwrap().into_inner();
        SumResponse::from_v2(response)
    }
}
