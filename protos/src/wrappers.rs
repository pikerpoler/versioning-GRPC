macro_rules! add_versions {
    ($(($version:ident, $variant:ident)),*) => {
        use tonic::{async_trait, Request, Response, Status};
        use crate::api::{inner, $($version,)*};
        $(
        impl_vector_service!($version);
        )*
        define_server!($($version),*);
        define_client!($(($version, $variant)),*);
    };
}

// implements a versions VectorService to just use the inner VectorService instead
macro_rules! impl_vector_service {
    ($version:ident) => {
        #[async_trait]
        impl<T> $version::vector_service_server::VectorService for T
        where
            T: inner::VectorService,
        {
            async fn print(
                &self,
                request: Request<$version::PrintRequest>,
            ) -> Result<Response<$version::PrintResponse>, Status> {
                let tmp = $version::VERSION_NAME;
                println!("rerouting print from {tmp:?}");
                let (metadata, extensions, inner_request) = request.into_parts();
                println!("original request recived in server: {inner_request:?}");
                let inner_request = inner::PrintRequest::from(inner_request);
                let request = Request::from_parts(metadata, extensions, inner_request);
                let (metadata, response, extensions) = inner::VectorService::print(self, request)
                    .await?
                    .into_parts();

                Ok(Response::from_parts(
                    metadata,
                    $version::PrintResponse::from(response),
                    extensions,
                ))
            }

            async fn sum(
                &self,
                request: Request<$version::SumRequest>,
            ) -> Result<Response<$version::SumResponse>, Status> {
                let tmp = $version::VERSION_NAME;
                println!("rerouting sum from {tmp:?}");
                let (metadata, extensions, inner_request) = request.into_parts();
                println!("original request recived in server: {inner_request:?}");
                let inner_request = inner::SumRequest::from(inner_request);
                let request = Request::from_parts(metadata, extensions, inner_request);
                let (metadata, response, extensions) =
                    inner::VectorService::sum(self, request).await?.into_parts();

                Ok(Response::from_parts(
                    metadata,
                    $version::SumResponse::from(response),
                    extensions,
                ))
            }
        }
    };
}

macro_rules! define_client {
    ($(($version:ident, $variant:ident)),*) => {
        pub mod vector_service_client {
            use tonic::codegen::*;
            use tonic::{Request, Response};

            use crate::api::{self, inner, $($version,)*};

            #[derive(Debug, Clone)]
            pub enum VectorServiceClient<T> {
                $(
                    $variant(api::$version::vector_service_client::VectorServiceClient<T>),
                )*
            }

            pub enum SupportedVersion {
                $(
                    $variant,
                )*
            }

            impl VectorServiceClient<tonic::transport::Channel> {
            pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
                where
                    D: TryInto<tonic::transport::Endpoint>,
                    D::Error: Into<StdError>,
                {
                    let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                    Ok(Self::new(conn))
                }

                pub async fn connect_versioned<D>(dst: D, version: SupportedVersion) -> Result<Self, tonic::transport::Error>
                where
                    D: TryInto<tonic::transport::Endpoint>,
                    D::Error: Into<StdError>,
                {
                    let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
                    Ok(Self::new_versioned(conn, version))
                }
            }
            impl<T> VectorServiceClient<T>
            where
                T: tonic::client::GrpcService<tonic::body::BoxBody>,
                T::Error: Into<StdError>,
                T::ResponseBody: Body<Data = Bytes> + Send + 'static,
                <T::ResponseBody as Body>::Error: Into<StdError> + Send,
            {

                pub fn new(inner: T) -> Self {
                    Self::new_versioned(inner, SupportedVersion::V1)
                }

                pub fn new_versioned(inner: T, version: SupportedVersion) -> Self{
                    match version {
                        $(
                            SupportedVersion::$variant =>{
                                let client = api::$version::vector_service_client::VectorServiceClient::new(inner);
                                Self::$variant(client)
                            }
                        )*
                    }
                }

                pub fn with_interceptor<F>(
                    inner: T,
                    interceptor: F,
                ) -> VectorServiceClient<InterceptedService<T, F>>
                where
                    F: tonic::service::Interceptor,
                    T::ResponseBody: Default,
                    T: tonic::codegen::Service<
                        http::Request<tonic::body::BoxBody>,
                        Response = http::Response<
                            <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                        >,
                    >,
                    <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                        Into<StdError> + Send + Sync,
                {

                    VectorServiceClient::new(InterceptedService::new(inner, interceptor))
                }

                #[must_use]
                pub fn send_compressed(self, encoding: CompressionEncoding) -> Self {
                    call_variant_method!(self, send_compressed, encoding, $($variant),*)
                }

                #[must_use]
                pub fn accept_compressed(self, encoding: CompressionEncoding) -> Self {
                    call_variant_method!(self, accept_compressed, encoding, $($variant),*)
                }

                #[must_use]
                pub fn max_decoding_message_size(self, limit: usize) -> Self {
                    call_variant_method!(self, max_decoding_message_size, limit, $($variant),*)
                }

                #[must_use]
                pub fn max_encoding_message_size(self, limit: usize) -> Self {
                    call_variant_method!(self, max_encoding_message_size, limit, $($variant),*)
                }

                delegate_client_call!(print, PrintRequest, PrintResponse, $(($version ,$variant)),*);
                delegate_client_call!(sum, SumRequest, SumResponse, $(($version ,$variant)),*);
            }
        }
    };
}

macro_rules! delegate_client_call {
    ($function:ident, $request_type:ident, $response_type: ident, $(($version:ident, $variant:ident)),*) => {
        pub async fn $function(
            &mut self,
            request: impl tonic::IntoRequest<inner::$request_type>,
        ) -> Result<Response<inner::$response_type>, tonic::Status> {
            let request = request.into_request();
            let (metadata, extensions, inner_request) = request.into_parts();
            
            match self {
                $(
                VectorServiceClient::$variant(client) => {
                    let inner_request = $version::$request_type::from(inner_request);
                    println!("request sent from client: {inner_request:?}");
                    let request = Request::from_parts(metadata, extensions, inner_request);
                    let (metadata, inner_response, extensions) =
                        client.$function(request).await?.into_parts();
                    Ok(Response::from_parts(
                        metadata,
                        inner::$response_type::from(inner_response),
                        extensions,
                    ))
                }
                )*

            }
        }
    };
}
macro_rules! call_variant_method {
    ($self:ident, $method:ident, $arg:ident, $($variant:ident),*) => {
        match $self {
            $(
                VectorServiceClient::$variant(client) => {
                    VectorServiceClient::$variant(client.$method($arg))
                }
            )*
        }
    };
}

macro_rules! define_server {
    ($($version:ident),*) => {
        pub mod vector_service_server {
            // VectorService can also be accessed directly from pinecone_service,
            // but exported here to keep the structure similar to the tonic-generated code
            pub use crate::api::inner::VectorService;

            use std::sync::Arc;
            use tonic::transport::server::Router;
            use tonic::transport::Server;

            pub fn add_services_to_router<T, R>(service: T) -> impl FnOnce(Router<R>) -> Router<R>
            where
                T: VectorService + Send + Sync,
                R:  Sized,
            {
                let service_arc = Arc::new(service);
                move |server| {
                    server
                        $(
                            .add_service(crate::api::$version::vector_service_server::VectorServiceServer::from_arc(service_arc.clone()))
                        )*
                }
            }

            // this function acts similarly to the previous one, but its ment to add services to a brand new Server which wasn't converted into a Router yet.
            // it is currently used only in pinecone-sim.
            // consider adding a middleware layer to the pinecone-sim services, and use the same function as in the real pinecone.
            pub fn add_services_to_server<T, R>(service: T) -> impl FnOnce(Server<R>) -> Router<R>
            where
                T: VectorService + Send + Sync,
                R:  Sized + Clone,
            {
                let service_arc = Arc::new(service);
                move |mut server| {
                    server
                        $(
                            .add_service(crate::api::$version::vector_service_server::VectorServiceServer::from_arc(service_arc.clone()))
                        )*
                }
            }
        }
    };
}

// Maybe we can use #[allow(non_camel_case_types)] and get rid of $variant?
// if we do so, this code will be a bit less readable,
// but we can have all the changes for adding a new version happen in outer-protos/src/lib.rs
// and here we can import a list of "user facing apis" and pass it to the macro
add_versions!((v1, V1), (v2, V2));
