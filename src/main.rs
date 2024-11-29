use std::sync::Arc;

use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use server::{
    business_server::YelpBusinessService, yelp::business_service_server::BusinessServiceServer,
};
use tokio_postgres::NoTls;
use tonic_reflection::server::Builder;

pub mod postgres;
pub mod server;

mod pb {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("yelp");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("password".to_string());
    cfg.dbname = Some("yelp".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    let addr = "[::1]:50051".parse()?;
    let business_service = YelpBusinessService::new(Arc::new(pool));

    let reflection_service = Builder::configure()
        .register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(BusinessServiceServer::new(business_service))
        .serve(addr)
        .await?;

    Ok(())
}
