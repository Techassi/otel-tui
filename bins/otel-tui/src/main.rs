use async_trait::async_trait;
use clap::Parser;
use opentelemetry_proto::tonic::collector::trace::v1::{
    trace_service_server::{TraceService, TraceServiceServer},
    ExportTraceServiceRequest, ExportTraceServiceResponse,
};
use tonic::{Request, Response, Status};

use crate::cli::Cli;

mod cli;

pub(crate) mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let service = InMemoryTraceService::new();
    let service = TraceServiceServer::new(service);

    tonic::transport::Server::builder()
        .add_service(service)
        .serve(cli.address)
        .await
        .unwrap();
}

pub struct InMemoryTraceService {}

#[async_trait]
impl TraceService for InMemoryTraceService {
    async fn export(
        &self,
        request: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        dbg!(&request);

        let message = request.into_inner();

        for resource_span in message.resource_spans {
            // resource_span.
        }

        Ok(Response::new(ExportTraceServiceResponse {
            partial_success: None,
        }))
    }
}

impl InMemoryTraceService {
    pub fn new() -> Self {
        Self {}
    }
}
