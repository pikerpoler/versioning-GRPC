use tonic::async_trait;

#[path = "api.v1.rs"]
#[rustfmt::skip]
pub mod api_v1;

#[path = "api.v2.rs"]
#[rustfmt::skip]
pub mod api_v2;

pub enum SupportedVersion {
    V1,
    V2,
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

#[async_trait]
impl api_v1::vector_service_server::VectorService for VectorHandler {
    async fn print(
        &self,
        request: tonic::Request<api_v1::PrintRequest>,
    ) -> std::result::Result<tonic::Response<api_v1::PrintResponse>, tonic::Status> {
        let name = &self.name;
        let vector = request.into_inner();

        println!("{name} V1 print: {vector:?}");
        let result = api_v1::PrintResponse {
            printed_count: vector.vectors.iter().len() as u32,
        };
        Ok(tonic::Response::new(result))
    }
    async fn sum(
        &self,
        request: tonic::Request<api_v1::SumRequest>,
    ) -> std::result::Result<tonic::Response<api_v1::SumResponse>, tonic::Status> {
        let name = &self.name;
        let vector = request.into_inner().vector;
        // let vc = vector.clone();
        let sum: f32 = match vector {
            Some(vector) => vector.values.into_iter().sum(),
            None => 0_f32,
        };
        println!("{name} V1 sum: {sum:?}");
        let result = api_v1::SumResponse { sum };
        Ok(tonic::Response::new(result))
    }
}

#[async_trait]
impl api_v2::vector_service_server::VectorService for VectorHandler {
    async fn print(
        &self,
        request: tonic::Request<api_v2::PrintRequest>,
    ) -> std::result::Result<tonic::Response<api_v2::PrintResponse>, tonic::Status> {
        let name = &self.name;
        let vector = request.into_inner();

        println!("{name} V2 print: {vector:?}");
        let result = api_v2::PrintResponse {
            printed_count: vector.vectors.iter().len() as u32,
        };
        Ok(tonic::Response::new(result))
    }
    async fn sum(
        &self,
        request: tonic::Request<api_v2::SumRequest>,
    ) -> std::result::Result<tonic::Response<api_v2::SumResponse>, tonic::Status> {
        let name = &self.name;
        let vector = request.into_inner().vector;
        // let vc = vector.clone();
        let sum: f32 = match vector {
            Some(vector) => vector.values.into_iter().sum(),
            None => 0_f32,
        };
        println!("{name} V2 sum: {sum:?}");
        let result = api_v2::SumResponse { sum };
        Ok(tonic::Response::new(result))
    }
}
