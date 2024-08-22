use tonic::{transport::Server, Request, Response, Status};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse()?;
    let my_service = MyServiceImpl::default();

    Server::builder()
        .add_service(MyServiceServer::new(my_service))
        .serve(address)
        .await?;

    Ok(())
}
