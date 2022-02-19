#![allow(clippy::module_inception)]
pub use hardware_service_client::HardwareServiceClient;
tonic::include_proto!("github.com.tinkerbell.tink.protos.hardware");
