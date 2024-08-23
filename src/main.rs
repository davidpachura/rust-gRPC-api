use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder;
use tonic_reflection::pb::v1::FILE_DESCRIPTOR_SET;
use api::my_service_server::{MyService, MyServiceServer};
use api::{HelloRequest, HelloResponse};

pub mod api {
    tonic::include_proto!("api");
}

#[derive(Default)]
pub struct MyServiceImpl{}

#[tonic::async_trait]
impl MyService for MyServiceImpl {
    async fn greet(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let reply = HelloResponse {
            message: format!("Hello {}", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

const DESCRIPTOR_SET: &[u8] = include_bytes!("descriptor.pb");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:50051".parse()?;
    let my_service = MyServiceImpl::default();

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(MyServiceServer::new(my_service))
        .add_service(reflection_service)
        .serve(address)
        .await?;

    Ok(())
}
