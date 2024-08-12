use protos::{serve, VectorHandler};

#[tokio::main]
async fn main() {
    let port = 162;
    let inner_service = VectorHandler {
        name: "my name".to_string(),
    };
    serve(port, inner_service).await.unwrap()
}

#[cfg(test)]
mod tests {

    use protos::{serve, VectorHandler};
    use std::time::Duration;
    use tokio::time::sleep;
    use tonic::transport::Channel;

    #[derive(Clone)]
    // Define the Client struct with versioned clients
    struct MultiVersionedClient {
        pub v1: protos::api_v1::vector_service_client::VectorServiceClient<Channel>,
        pub v2: protos::api_v2::vector_service_client::VectorServiceClient<Channel>,
    }

    impl MultiVersionedClient {
        async fn new(port: u16) -> Result<Self, Box<dyn std::error::Error>> {
            let v1 = protos::api_v1::vector_service_client::VectorServiceClient::connect(format!(
                "http://localhost:{}",
                port
            ))
            .await?;
            let v2 = protos::api_v2::vector_service_client::VectorServiceClient::connect(format!(
                "http://localhost:{}",
                port
            ))
            .await?;

            Ok(MultiVersionedClient { v1, v2 })
        }
    }
    #[tokio::test]
    async fn simple_test() {
        let port = 1818;
        let inner_service = VectorHandler {
            name: "my name".to_string(),
        };
        let server_handle = tokio::spawn(async move {
            let _ = serve(port, inner_service.clone()).await;
        });
        sleep(Duration::from_secs(1)).await;

        let mut client = MultiVersionedClient::new(port).await.unwrap();
        let vec1 = protos::api_v1::Vector {
            id: "id1".parse().unwrap(),
            values: vec![1., 1., 1.],
        };
        let vec2 = protos::api_v2::Vector {
            id: "id2".parse().unwrap(),
            values: vec![2., 2., 2.],
        };

        let print_request_1 = protos::api_v1::PrintRequest {
            vectors: Some(vec1.clone()),
        };
        let print_request_2 = protos::api_v2::PrintRequest {
            vectors: Some(vec2.clone()),
        };
        let sum_request_1 = protos::api_v1::SumRequest { vector: Some(vec1) };
        let sum_request_2 = protos::api_v2::SumRequest { vector: Some(vec2) };

        let print_result1 = client.v1.print(print_request_1).await.unwrap();
        println!("print result 1: {print_result1:?}");
        let print_result2 = client.v2.print(print_request_2).await.unwrap();
        println!("print result 2: {print_result2:?}");

        let sum_result1 = client.v1.sum(sum_request_1).await.unwrap();
        println!("sun result 1: {sum_result1:?}");
        let sum_result2 = client.v2.sum(sum_request_2).await.unwrap();
        println!("sun result 2: {sum_result2:?}");

        server_handle.abort();
        // Wait for the server to shut down
        let _ = server_handle.await;
    }
}
