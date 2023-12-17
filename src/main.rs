use {{service_name}}_service::{{service_name}}_impl::{{service_name | pascal_case}}ServiceImpl;
use std::net::SocketAddr;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 50051));

    Server::builder()
        .add_service({{service_name | pascal_case}}ServiceImpl::attach_reflection()?)
        .add_service({{service_name | pascal_case}}ServiceImpl::new())
        .serve(addr)
        .await?;
    Ok(())
}
