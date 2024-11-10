use versioning_grpc::{serve, VectorHandler};

#[tokio::main]
async fn main() {
    let port = 1620;
    let inner_service = VectorHandler {
        name: "my name".to_string(),
    };
    serve(port, inner_service).await.unwrap()
}

#[cfg(test)]
mod tests {
    use protos::vector_service::vector_service_client::{SupportedVersion, VectorServiceClient};
    use protos::vector_service::{PrintRequest, SumRequest, Vector};
    use std::time::Duration;
    use tokio::time::sleep;
    use tonic::transport::Uri;
    use versioning_grpc::{serve, VectorHandler};

    #[tokio::test]
    async fn simple_test() {
        let port = 1818;
        let address: Uri = format!("https://0.0.0.0:{}", port).parse().unwrap();
        println!("Connecting to {}", address);
        let inner_service = VectorHandler {
            name: "my name".to_string(),
        };
        let server_handle = tokio::spawn(async move {
            let _ = serve(port, inner_service.clone()).await;
        });
        sleep(Duration::from_secs(1)).await;

        let mut client_v1 =
            VectorServiceClient::connect_versioned(address.clone(), SupportedVersion::V1)
                .await
                .unwrap();
        let mut client_v2 = VectorServiceClient::connect_versioned(address, SupportedVersion::V2)
            .await
            .unwrap();
        let vec1 = Vector {
            id: "id1".parse().unwrap(),
            values: vec![1., 1., 1.],
        };
        let vec2 = Vector {
            id: "id2".parse().unwrap(),
            values: vec![2., 2., 2.],
        };

        let print_request_1 = PrintRequest {
            vector: Some(vec1.clone()),
        };
        let print_request_2 = PrintRequest {
            vector: Some(vec2.clone()),
        };
        let sum_request_1 = SumRequest {
            vectors: vec![vec1],
        };
        let sum_request_2 = SumRequest {
            vectors: vec![vec2],
        };

        let print_result1 = client_v1.print(print_request_1).await; // VectorService::<api_versions::V1>::print(&client,print_request_1).await;
        println!("print result 1: {print_result1:?}");
        let print_result2 = client_v2.print(print_request_2).await;
        println!("print result 2: {print_result2:?}");

        let sum_result1 = client_v1.sum(sum_request_1).await;
        println!("sum result 1: {sum_result1:?}");
        let sum_result2 = client_v2.sum(sum_request_2).await;
        println!("sum result 2: {sum_result2:?}");

        server_handle.abort();
        // Wait for the server to shut down
        let _ = server_handle.await;
    }

    use protos::actual_clients::v1::Vector as Vector_V1;
    use protos::actual_clients::v1::{
        vector_service_client::VectorServiceClient as VectorServiceClient_V1,
        SumRequest as SumRequest_V1,
    };
    use protos::actual_clients::v2::{
        vector_service_client::VectorServiceClient as VectorServiceClient_V2,
        SumRequest as SumRequest_V2,
    };
    #[tokio::test]
    // in this test we will use the actual clients the users will be using
    async fn actual_client_test() {
        let port = 1818;

        let address: Uri = format!("https://0.0.0.0:{}", port).parse().unwrap();
        println!("Connecting to {}", address);
        let inner_service = VectorHandler {
            name: "actual_input".to_string(),
        };
        let server_handle = tokio::spawn(async move {
            let _ = serve(port, inner_service.clone()).await;
        });
        sleep(Duration::from_secs(1)).await;

        let mut client_v1 = VectorServiceClient_V1::connect(address.clone())
            .await
            .unwrap();
        let mut client_v2 = VectorServiceClient_V2::connect(address).await.unwrap();

        let vec1 = Vector_V1 {
            id: "id1".parse().unwrap(),
            values: vec![1., 1., 1.],
        };
        let vec2 = Vector_V1 {
            id: "id2".parse().unwrap(),
            values: vec![2., 2., 2.],
        };
        let sum_request_1 = SumRequest_V1 {
            vector: Some(vec1.clone()),
        };
        let sum_request_2 = SumRequest_V2 {
            vectors: vec![vec1, vec2],
        };

        let sum_result1 = client_v1.sum(sum_request_1.clone()).await;
        println!("sum result 1: {sum_result1:?}");
        let sum_result2 = client_v2.sum(sum_request_2).await;
        println!("sum result 2: {sum_result2:?}");

        server_handle.abort();
        // Wait for the server to shut down
        let _ = server_handle.await;
    }

    #[tokio::test]
    // in this test we will rely on the server running in a different terminal.
    // this may help simplify what happens on the which end (client/server)
    async fn detached_test() {
        let port = 1620;
        let address: Uri = format!("https://0.0.0.0:{}", port).parse().unwrap();
        let mut client_v1 = VectorServiceClient_V1::connect(address.clone())
            .await
            .unwrap();
        let mut client_v2 = VectorServiceClient_V2::connect(address).await.unwrap();

        let vec1 = Vector_V1 {
            id: "id1".parse().unwrap(),
            values: vec![1., 1., 1.],
        };
        let vec2 = Vector_V1 {
            id: "id2".parse().unwrap(),
            values: vec![2., 2., 2.],
        };
        let sum_request_1 = SumRequest_V1 {
            vector: Some(vec1.clone()),
        };
        let sum_request_2 = SumRequest_V2 {
            vectors: vec![vec1, vec2],
        };

        let sum_result1 = client_v1.sum(sum_request_1).await;
        println!("sum result 1: {sum_result1:?}");
        let sum_result2 = client_v2.sum(sum_request_2).await;
        println!("sum result 2: {sum_result2:?}");
    }
}
