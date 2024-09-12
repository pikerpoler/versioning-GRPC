use anyhow::Context;

use protos::vector_service::vector_service_server;
use protos::vector_service::{PrintRequest, PrintResponse, SumRequest, SumResponse, VectorService};

use tonic::async_trait;
use tonic::transport::Server;
use tonic::{Request, Response};

pub async fn serve(port: u16, inner_service: VectorHandler) -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:{}", port).parse()?;

    let add_services = vector_service_server::add_services_to_server(inner_service);
    add_services(Server::builder())
        .serve(bind_addr)
        .await
        .context("error initializing server")
}

#[derive(Clone)]
pub struct VectorHandler {
    pub name: String,
}
#[async_trait]
impl VectorService for VectorHandler {
    async fn print(
        &self,
        request: Request<PrintRequest>,
    ) -> Result<Response<PrintResponse>, tonic::Status> {
        let name = &self.name;
        let vector = request.into_inner().vector;

        println!("{name} VectorService print: {vector:?}");

        Ok(Response::new(PrintResponse {
            printed_count: vector.iter().len() as u32,
        }))
    }
    async fn sum(
        &self,
        request: Request<SumRequest>,
    ) -> Result<Response<SumResponse>, tonic::Status> {
        let name = &self.name;
        let vectors = request.into_inner().vectors;
        let sum = vectors
            .iter()
            .map(|vector| vector.values.iter().sum())
            .collect();

        println!("{name} VectorService sum: {sum:?}");

        Ok(Response::new(SumResponse { sum }))
    }
}
