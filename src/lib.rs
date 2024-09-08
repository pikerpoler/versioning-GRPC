use std::sync::Arc;
use anyhow::Context;

use protos::{PrintRequest, PrintResponse, SumRequest, SumResponse, VectorService, VectorServiceServerV1, VectorServiceServerV2};
use tonic::async_trait;
use tonic::transport::Server;


pub async fn serve(port: u16, inner_service: VectorHandler) -> anyhow::Result<()> {
    let bind_addr = format!("0.0.0.0:{}", port).parse()?;
    let service_arc = Arc::new(inner_service);
    let s1 = VectorServiceServerV1::from_arc(service_arc.clone());
    let s2 = VectorServiceServerV2::from_arc(service_arc);
    // let (s1, s2) = get_servers(inner_service);
    Server::builder().add_service(s1).add_service(s2).serve(bind_addr).await.context("error initializing server")
}

#[derive(Clone)]
pub struct VectorHandler {
    pub name: String,
}
#[async_trait]
impl VectorService for VectorHandler {
    async fn print(&self, request: PrintRequest) -> PrintResponse {
        let name = &self.name;
        let vector = request.vector;
        println!("{name} VectorService print: {vector:?}");
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
        println!("{name} VectorService sum: {sum:?}");
        SumResponse { sum }
    }
}
