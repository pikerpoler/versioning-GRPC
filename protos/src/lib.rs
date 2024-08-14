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
        let inner_request = api_v1::PrintRequest::from(request);
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
        let inner_request = api_v2::PrintRequest::from(request);
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
        let inner_request = api_v1::SumRequest::from(request);
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
        let inner_request = api_v2::SumRequest::from(request);
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


impl From<Vector> for api_v1::Vector {
    fn from(value: Vector) -> Self {
        api_v1::Vector {
            id: value.id.clone(),
            values: value.values.clone(),
        }
    }
}

impl From<Vector> for api_v2::Vector {
    fn from(value: Vector) -> Self {
        api_v2::Vector {
            id: value.id.clone(),
            values: value.values.clone(),
        }
    }
}

impl From<api_v1::Vector> for Vector {
    fn from(vector: api_v1::Vector) -> Vector {
        Vector {
            id: vector.id,
            values: vector.values,
        }
    }
}

impl From<api_v2::Vector> for Vector {
    fn from(vector: api_v2::Vector) -> Vector {
        Vector {
            id: vector.id,
            values: vector.values,
        }
    }
}


impl From<PrintRequest> for api_v1::PrintRequest {
    fn from(value: PrintRequest) -> Self {
        api_v1::PrintRequest {
            vectors: value.vector.map(|v| v.into()), // Use Into here
        }
    }
}

impl From<PrintRequest> for api_v2::PrintRequest {
    fn from(value: PrintRequest) -> Self {
        api_v2::PrintRequest {
            vectors: value.vector.map(|v| v.into()), // Use Into here
        }
    }
}

impl From<api_v1::PrintRequest> for PrintRequest {
    fn from(request: api_v1::PrintRequest) -> PrintRequest {
        PrintRequest {
            vector: request.vectors.map(Vector::from),
        }
    }
}

impl From<api_v2::PrintRequest> for PrintRequest {
    fn from(request: api_v2::PrintRequest) -> PrintRequest {
        PrintRequest {
            vector: request.vectors.map(Vector::from),
        }
    }
}



// TODO: unbreaking changes:adding fields
// TODO: breaking changes: changing integer types, fields, removing stuff.. same name different type
impl From<PrintResponse> for api_v1::PrintResponse {
    fn from(value: PrintResponse) -> Self {
        api_v1::PrintResponse {
            printed_count: value.printed_count,
        }
    }
}

impl From<PrintResponse> for api_v2::PrintResponse {
    fn from(value: PrintResponse) -> Self {
        api_v2::PrintResponse {
            printed_count: value.printed_count,
        }
    }
}

impl From<api_v1::PrintResponse> for PrintResponse {
    fn from(response: api_v1::PrintResponse) -> Self {
        PrintResponse {
            printed_count: response.printed_count,
        }
    }
}

impl From<api_v2::PrintResponse> for PrintResponse {
    fn from(response: api_v2::PrintResponse) -> Self {
        PrintResponse {
            printed_count: response.printed_count,
        }
    }
}


impl From<SumRequest> for api_v1::SumRequest {
    fn from(value: SumRequest) -> Self {
        api_v1::SumRequest {
            vector: value.vector.map(|v| v.into()), // Use Into here
        }
    }
}

impl From<SumRequest> for api_v2::SumRequest {
    fn from(value: SumRequest) -> Self {
        api_v2::SumRequest {
            vector: value.vector.map(|v| v.into()), // Use Into here
        }
    }
}

impl From<api_v1::SumRequest> for SumRequest {
    fn from(request: api_v1::SumRequest) -> Self {
        SumRequest {
            vector: request.vector.map(Vector::from),
        }
    }
}

impl From<api_v2::SumRequest> for SumRequest {
    fn from(request: api_v2::SumRequest) -> Self {
        SumRequest {
            vector: request.vector.map(Vector::from),
        }
    }
}


impl From<SumResponse> for api_v1::SumResponse {
    fn from(value: SumResponse) -> Self {
        api_v1::SumResponse {
            sum: value.sum,
        }
    }
}

impl From<SumResponse> for api_v2::SumResponse {
    fn from(value: SumResponse) -> Self {
        api_v2::SumResponse {
            sum: value.sum,
        }
    }
}

impl From<api_v1::SumResponse> for SumResponse {
    fn from(response: api_v1::SumResponse) -> Self {
        SumResponse {
            sum: response.sum,
        }
    }
}

impl From<api_v2::SumResponse> for SumResponse {
    fn from(response: api_v2::SumResponse) -> Self {
        SumResponse {
            sum: response.sum,
        }
    }
}


#[async_trait]
pub trait VectorService {
    async fn print(&self, request: PrintRequest) -> PrintResponse;
    async fn sum(&self, request: SumRequest) -> SumResponse;
}

#[async_trait]
impl<T> api_v1::vector_service_server::VectorService for T
where
    T: VectorService + std::marker::Sync + std::marker::Send + 'static,
{
    async fn print(
        &self,
        request: tonic::Request<api_v1::PrintRequest>,
    ) -> Result<tonic::Response<api_v1::PrintResponse>, tonic::Status> {
        let inner_request = PrintRequest::from(request.into_inner());
        let inner_response = VectorService::print(self, inner_request).await;
        Ok(tonic::Response::new(api_v1::PrintResponse::from(inner_response)))
    }
    async fn sum(
        &self,
        request: tonic::Request<api_v1::SumRequest>,
    ) -> Result<tonic::Response<api_v1::SumResponse>, tonic::Status> {
        let inner_request = SumRequest::from(request.into_inner());
        let inner_response = VectorService::sum(self, inner_request).await;
        Ok(tonic::Response::new(api_v1::SumResponse::from(inner_response)))
    }
}

#[async_trait]
impl VectorService for Box<dyn api_v1::vector_service_server::VectorService + 'static> {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let inner = api_v1::PrintRequest::from(request).into_request();
        let response = self.as_ref().print(inner).await.unwrap().into_inner();
        PrintResponse::from(response)
    }

    async fn sum(&self, request: SumRequest) -> SumResponse {
        let inner = api_v1::SumRequest::from(request).into_request();
        let response = self.as_ref().sum(inner).await.unwrap().into_inner();
        SumResponse::from(response)
    }
}

#[async_trait]
impl<T> api_v2::vector_service_server::VectorService for T
where
    T: VectorService + std::marker::Sync + std::marker::Send + 'static,
{
    async fn print(
        &self,
        request: tonic::Request<api_v2::PrintRequest>,
    ) -> Result<tonic::Response<api_v2::PrintResponse>, tonic::Status> {
        let inner_request = PrintRequest::from(request.into_inner());
        let inner_response = VectorService::print(self, inner_request).await;
        Ok(tonic::Response::new(api_v2::PrintResponse::from(inner_response)))
    }
    async fn sum(
        &self,
        request: tonic::Request<api_v2::SumRequest>,
    ) -> Result<tonic::Response<api_v2::SumResponse>, tonic::Status> {
        let inner_request = SumRequest::from(request.into_inner());
        let inner_response = VectorService::sum(self, inner_request).await;
        Ok(tonic::Response::new(api_v2::SumResponse::from(inner_response)))
    }
}

#[async_trait]
impl VectorService for Box<dyn api_v2::vector_service_server::VectorService + 'static> {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let inner = api_v2::PrintRequest::from(request).into_request();
        let response = self.as_ref().print(inner).await.unwrap().into_inner();
        PrintResponse::from(response)
    }

    async fn sum(&self, request: SumRequest) -> SumResponse {
        let inner = api_v2::SumRequest::from(request).into_request();
        let response = self.as_ref().sum(inner).await.unwrap().into_inner();
        SumResponse::from(response)
    }
}
