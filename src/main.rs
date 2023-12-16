use {{service_name}}_service::{{service_name}}::{{service_name}}_service_server::AuthServiceServer;
use {{service_name}}_service::{{service_name}}_impl::AuthServiceImpl;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let {{service_name}}_service = AuthServiceImpl::default();

    Server::builder()
        .add_service(AuthServiceServer::new({{service_name}}_service))
        .serve(addr)
        .await?;
    Ok(())
}