use anyhow;
use anyhow::Context;

use protos::{SupportedVersion, VectorHandler, VersionedVectorServiceServer};
use std::net::SocketAddr;
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
        VersionedVectorServiceServer::new_from(inner_service.clone(), SupportedVersion::V1),
        VersionedVectorServiceServer::new_from(inner_service, SupportedVersion::V2),
    ];
    export_grpc_services(bind_addr, reflection_server, service_server_list).await
}
