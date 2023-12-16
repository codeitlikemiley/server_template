use {{service_name}}_service::{{service_name}}::{{service_name}}_service_server::{{service_name | pascal_case}}ServiceServer;
use {{service_name}}_service::{{service_name}}_impl::{{service_name | pascal_case}}ServiceImpl;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let {{service_name}}_service = {{service_name | pascal_case}}ServiceImpl::default();

    Server::builder()
        .add_service({{service_name | pascal_case}}ServiceServer::new({{service_name}}_service))
        .serve(addr)
        .await?;
    Ok(())
}