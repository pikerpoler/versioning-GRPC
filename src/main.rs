use protos::VectorHandler;
use versioning_grpc::serve;

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

    use protos::{MultiVersionedClient, VectorService};
    use std::time::Duration;
    use tokio::time::sleep;

    use protos::api_versions;
    use protos::VectorHandler;
    use versioning_grpc::serve;

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

        let client = MultiVersionedClient::new(port).await.unwrap();
        let vec1 = protos::Vector {
            id: "id1".parse().unwrap(),
            values: vec![1., 1., 1.],
        };
        let vec2 = protos::Vector {
            id: "id2".parse().unwrap(),
            values: vec![2., 2., 2.],
        };

        let print_request_1 = protos::PrintRequest {
            vector: Some(vec1.clone()),
        };
        let print_request_2 = protos::PrintRequest {
            vector: Some(vec2.clone()),
        };
        let sum_request_1 = protos::SumRequest { vector: Some(vec1) };
        let sum_request_2 = protos::SumRequest { vector: Some(vec2) };

        // note that we can call a method for client in two ways.
        // VectorService::<api_versions::V2>::print  or client.print_v2(...
        // should we support both?
        let print_result1 = client.print_v1(print_request_1).await; // VectorService::<api_versions::V1>::print(&client,print_request_1).await;
        println!("print result 1: {print_result1:?}");
        let print_result2 =
            VectorService::<api_versions::V2>::print(&client, print_request_2).await;
        println!("print result 2: {print_result2:?}");

        let sum_result1 = VectorService::<api_versions::V1>::sum(&client, sum_request_1).await;
        println!("sun result 1: {sum_result1:?}");
        let sum_result2 = client.sum_v2(sum_request_2).await;
        println!("sun result 2: {sum_result2:?}");

        server_handle.abort();
        // Wait for the server to shut down
        let _ = server_handle.await;
    }
}
