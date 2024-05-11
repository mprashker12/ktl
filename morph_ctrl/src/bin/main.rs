use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client};

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let pods = Api::<Pod>::all(client);
    Ok(())
}
