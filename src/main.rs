use tonic::{transport::Server};

{% for service in service_names.split(",") %}
use {{service.trim()}}::{{service.trim()}}_service_server::{{ServiceName | pascal_case}}Service;
use {{service.trim()}}::{{service.trim()}}_impl::{{ServiceName | pascal_case}}ServiceImpl;
{% endfor %}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let addr = "[::1]:50051".parse()?;
     let mut server = Server::builder();

     {% for service in service_names.split(",") %}
     let {{service.trim()}} = {{ServiceName | pascal_case}}ServiceImpl{};
     server = server.add_service({{ServiceName | pascal_case}}ServiceServer::new({{service.trim()}}));
     {% endfor %}

     server.serve(addr).await?;

     Ok(())
}
