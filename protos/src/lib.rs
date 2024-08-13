use tonic::async_trait;
use tonic::transport::Channel;

#[path = "api.v1.rs"]
#[rustfmt::skip]
mod api_v1;

#[path = "api.v2.rs"]
#[rustfmt::skip]
mod api_v2;

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
        VectorService::<api_versions::V1>::print(self, request).await
    }
    pub async fn print_v2(&self, request: PrintRequest) -> PrintResponse {
        VectorService::<api_versions::V2>::print(self, request).await
    }
    pub async fn sum_v1(&self, request: SumRequest) -> SumResponse {
        VectorService::<api_versions::V1>::sum(self, request).await
    }
    pub async fn sum_v2(&self, request: SumRequest) -> SumResponse {
        VectorService::<api_versions::V2>::sum(self, request).await
    }
}

#[async_trait]
impl VectorService<api_versions::V1> for MultiVersionedClient {
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
impl VectorService<api_versions::V2> for MultiVersionedClient {
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
    pub struct V1;
    pub struct V2;
}
pub enum VersionedVectorServiceServer {
    V1(api_v1::vector_service_server::VectorServiceServer<VectorHandler>),
    V2(api_v2::vector_service_server::VectorServiceServer<VectorHandler>),
}

impl VersionedVectorServiceServer {
    pub fn new_from(service: VectorHandler, version: SupportedVersion) -> Self {
        // let boxed_service= Box::new(service);
        match version {
            SupportedVersion::V1 => VersionedVectorServiceServer::V1(
                api_v1::vector_service_server::VectorServiceServer::new(service),
            ),
            SupportedVersion::V2 => VersionedVectorServiceServer::V2(
                api_v2::vector_service_server::VectorServiceServer::new(service),
            ),
        }
    }
}

#[derive(Clone)]
pub struct VectorHandler {
    pub name: String,
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
pub trait VectorService<V> {
    async fn print(&self, request: PrintRequest) -> PrintResponse;
    async fn sum(&self, request: SumRequest) -> SumResponse;
}

#[async_trait]
impl VectorService<api_versions::V1> for VectorHandler {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let name = &self.name;
        let vector = request.vector;
        println!("{name} V1 print: {vector:?}");
        let result = api_v1::PrintResponse {
            printed_count: vector.iter().len() as u32,
        };
        PrintResponse::from_v1(result)
    }
    async fn sum(&self, request: SumRequest) -> SumResponse {
        let name = &self.name;
        let vector = request.vector;
        let sum: f32 = match vector {
            Some(vector) => vector.values.into_iter().sum(),
            None => 0_f32,
        };
        println!("{name} V1 sum: {sum:?}");
        let result = api_v1::SumResponse { sum };
        SumResponse::from_v1(result)
    }
}

#[async_trait]
impl VectorService<api_versions::V2> for VectorHandler {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let name = &self.name;
        let vector = request.vector;
        println!("{name} V2 print: {vector:?}");
        let result = api_v2::PrintResponse {
            printed_count: vector.iter().len() as u32,
        };
        PrintResponse::from_v2(result)
    }
    async fn sum(&self, request: SumRequest) -> SumResponse {
        let name = &self.name;
        let vector = request.vector;
        let sum: f32 = match vector {
            Some(vector) => vector.values.into_iter().sum(),
            None => 0_f32,
        };
        println!("{name} V2 sum: {sum:?}");
        let result = api_v2::SumResponse { sum };
        SumResponse::from_v2(result)
    }
}

#[async_trait]
impl api_v1::vector_service_server::VectorService for VectorHandler {
    async fn print(
        &self,
        request: tonic::Request<api_v1::PrintRequest>,
    ) -> Result<tonic::Response<api_v1::PrintResponse>, tonic::Status> {
        let inner_request = PrintRequest::from_v1(request.into_inner());
        let inner_response = VectorService::<api_versions::V1>::print(self, inner_request).await;
        Ok(tonic::Response::new(inner_response.to_v1()))
    }
    async fn sum(
        &self,
        request: tonic::Request<api_v1::SumRequest>,
    ) -> Result<tonic::Response<api_v1::SumResponse>, tonic::Status> {
        let inner_request = SumRequest::from_v1(request.into_inner());
        let inner_response = VectorService::<api_versions::V1>::sum(self, inner_request).await;
        Ok(tonic::Response::new(inner_response.to_v1()))
    }
}

#[async_trait]
impl api_v2::vector_service_server::VectorService for VectorHandler {
    async fn print(
        &self,
        request: tonic::Request<api_v2::PrintRequest>,
    ) -> Result<tonic::Response<api_v2::PrintResponse>, tonic::Status> {
        let inner_request = PrintRequest::from_v2(request.into_inner());
        let inner_response = VectorService::<api_versions::V2>::print(self, inner_request).await;
        Ok(tonic::Response::new(inner_response.to_v2()))
    }
    async fn sum(
        &self,
        request: tonic::Request<api_v2::SumRequest>,
    ) -> Result<tonic::Response<api_v2::SumResponse>, tonic::Status> {
        let inner_request = SumRequest::from_v2(request.into_inner());
        let inner_response = VectorService::<api_versions::V2>::sum(self, inner_request).await;
        Ok(tonic::Response::new(inner_response.to_v2()))
    }
}
