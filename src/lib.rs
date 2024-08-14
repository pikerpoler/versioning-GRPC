use anyhow::Context;

use protos::{PrintRequest, PrintResponse, SumRequest, SumResponse, VectorService, VersionedVectorServiceServer};
use std::net::SocketAddr;
use tonic::async_trait;
use tonic::transport::Server;
use tonic_reflection::server::{ServerReflection, ServerReflectionServer};

//TODO: figure out what is reflection server and wether we need it. if not, remove this code as it causes an error in this crate.
// const VECTOR_DESCRIPTOR_SET_V1: &[u8] = tonic::include_file_descriptor_set!("api.V1");
// const VECTOR_DESCRIPTOR_SET_V2: &[u8] = tonic::include_file_descriptor_set!("api.V2");

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
        // .register_encoded_file_descriptor_set(VECTOR_DESCRIPTOR_SET_V1)
        // .register_encoded_file_descriptor_set(VECTOR_DESCRIPTOR_SET_V2)
        .build()?;

    let service_server_list = vec![
        VersionedVectorServiceServer::new_v1(Box::new(inner_service.clone())),
        VersionedVectorServiceServer::new_v2(Box::new(inner_service.clone())),
    ];
    export_grpc_services(bind_addr, reflection_server, service_server_list).await
}

#[derive(Clone)]
pub struct VectorHandler {
    pub name: String,
}
#[async_trait]
impl VectorService<1> for VectorHandler {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let name = &self.name;
        let vector = request.vector;
        println!("{name} V1 print: {vector:?}");
        PrintResponse {
            printed_count: vector.iter().len() as u32,
        }
    }
    async fn sum(&self, request: SumRequest) -> SumResponse {
        let name = &self.name;
        let vector = request.vector;
        let sum: f32 = match vector {
            Some(vector) => vector.values.into_iter().sum(),
            None => 0_f32,
        };
        println!("{name} V1 sum: {sum:?}");
        SumResponse { sum }
    }
}

#[async_trait]
impl VectorService<2> for VectorHandler {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let name = &self.name;
        let vector = request.vector;
        println!("{name} V2 print: {vector:?}");
        PrintResponse {
            printed_count: vector.iter().len() as u32,
        }
    }
    async fn sum(&self, request: SumRequest) -> SumResponse {
        let name = &self.name;
        let vector = request.vector;
        let sum: f32 = match vector {
            Some(vector) => vector.values.into_iter().sum(),
            None => 0_f32,
        };
        println!("{name} V2 sum: {sum:?}");
        SumResponse { sum }
    }
}
