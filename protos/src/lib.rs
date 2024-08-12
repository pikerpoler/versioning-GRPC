use std::net::SocketAddr;

use anyhow::Context;
use tonic::async_trait;
use tonic::transport::Server;
use tonic_reflection::server::{ServerReflection, ServerReflectionServer};

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

const VECTOR_DESCRIPTOR_SET_V1: &[u8] = tonic::include_file_descriptor_set!("vector_service.V1");
const VECTOR_DESCRIPTOR_SET_V2: &[u8] = tonic::include_file_descriptor_set!("vector_service.V2");

async fn export_grpc_services(
    bind_addr: SocketAddr,
    reflection_server: ServerReflectionServer<impl ServerReflection + Sized>,
    service_server_list: Vec<VersionedVectorServiceServer>,
) -> anyhow::Result<()> {
    let mut server = Server::builder();
    let mut router = Box::new(server.add_service(reflection_server));
    for service in service_server_list {
        router = match service {
            VersionedVectorServiceServer::V1(s) => Box::new(router.add_service(s)),
            VersionedVectorServiceServer::V2(s) => Box::new(router.add_service(s)),
        };
    }
    router
        .serve(bind_addr)
        .await
        .context("error running gRPC server")
}

pub async fn serve(port: u16, inner_service: VectorHandler) -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:{}", port).parse()?;

    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(VECTOR_DESCRIPTOR_SET_V1)
        .register_encoded_file_descriptor_set(VECTOR_DESCRIPTOR_SET_V2)
        .build()?;

    let service_server_list = vec![
        VersionedVectorServiceServer::new_from(inner_service.clone(), SupportedVersion::V1),
        VersionedVectorServiceServer::new_from(inner_service, SupportedVersion::V2),
    ];
    export_grpc_services(bind_addr, reflection_server, service_server_list).await
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
