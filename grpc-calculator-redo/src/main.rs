use proto::admin_server::{Admin, AdminServer};
use proto::calculator_server::{Calculator, CalculatorServer};

use crate::proto::{CalculationRequest, CalculationResponse};

mod proto {
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct CalculatorService {
    pub state: State,
}

#[derive(Debug, Default)]
struct AdminService {
    state: State,
}

impl CalculatorService {
    async fn increment_sounter(&self) {
        let mut count = self.state.write().await;

        *count += 1;
        println!("Request Count: {}", *count);
    }
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_count(
        &self,
        _request: tonic::Request<proto::GetCountRequest>,
    ) -> Result<tonic::Response<proto::GetCountResponse>, tonic::Status> {
        let count = self.state.read().await;
        Ok(tonic::Response::new(proto::GetCountResponse {
            count: *count as i64,
        }))
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: tonic::Request<CalculationRequest>,
    ) -> Result<tonic::Response<CalculationResponse>, tonic::Status> {
        let req = request.get_ref();

        self.increment_sounter().await;

        let result = req.a + req.b;

        Ok(tonic::Response::new(CalculationResponse { result: result }))
    }

    async fn say_age(
        &self,
        request: tonic::Request<proto::CalculationRequest>,
    ) -> Result<tonic::Response<proto::HelloResponse>, tonic::Status> {
        let req = request.get_ref();
        self.increment_sounter().await;

        let response = proto::HelloResponse {
            response: format!("Hello your age is: {}{}", req.a, req.b),
        };

        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;

    let state = State::default();

    let calculator = CalculatorService {
        state: state.clone(),
    };
    let admin = AdminService {
        state: state.clone(),
    };

    let calculator_service = CalculatorServer::new(calculator);
    let admin_service = AdminServer::new(admin);
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    tonic::transport::Server::builder()
        .add_service(reflection_service)
        .add_service(calculator_service)
        .add_service(admin_service)
        .serve(addr)
        .await?;

    Ok(())
}
