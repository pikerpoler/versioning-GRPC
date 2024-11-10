mod wrappers;

mod api {
    pub(crate) mod v1 {
        pub const VERSION_NAME: &str = "V1";
        include!("api.v1.rs");
    }

    pub(crate) mod v2 {
        pub const VERSION_NAME: &str = "V2";
        pub use super::v1::*;
        include!("api.v2.rs");

        impl From<super::v1::SumRequest> for SumRequest {
            fn from(value: super::v1::SumRequest) -> Self {
                match value.vector {
                    Some(vector) => SumRequest {
                        vectors: vec![vector],
                    },
                    None => SumRequest { vectors: vec![] },
                }
            }
        }
        impl From<SumRequest> for super::v1::SumRequest {
            fn from(value: SumRequest) -> Self {
                assert_eq!(value.vectors.len(), 1);
                super::v1::SumRequest {
                    vector: Some(value.vectors[0].clone()),
                }
            }
        }

        impl From<super::v1::SumResponse> for SumResponse {
            fn from(value: super::v1::SumResponse) -> Self {
                SumResponse {
                    sum: vec![value.sum],
                }
            }
        }
        impl From<SumResponse> for super::v1::SumResponse {
            fn from(value: SumResponse) -> Self {
                assert_eq!(value.sum.len(), 1);
                super::v1::SumResponse { sum: value.sum[0] }
            }
        }
    }

    pub(crate) mod inner {
        pub use super::v1::Vector;
        pub use super::v2::{PrintRequest, PrintResponse, SumRequest, SumResponse};

        include!("api.inner.rs");

        pub use vector_service_server::VectorService;
    }
}

pub mod vector_service {
    pub use crate::api::inner::*;
    pub use crate::wrappers::{vector_service_client, vector_service_server};
}

// these clients are what the client will actually use, and intended for showcasing.
// actual tests should use the inner clients
pub mod actual_clients{
    pub mod v1{
        include!("api.v1.rs");
    }
    pub mod v2{
        pub use super::v1::*;
        include!("api.v2.rs");
    }
}