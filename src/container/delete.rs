use client::{
    services::v1::{DeleteContainerRequest, containers_client::ContainersClient},
    tonic::Request,
    with_namespace,
};
use containerd_client as client;

const NAMESPACE: &str = "default";

/// Delete the container identified by `cid`.
#[tokio::main(flavor = "current_thread")]
pub async fn delete(cid: String) {
    let channel = client::connect("/run/containerd/containerd.sock")
        .await
        .expect("Connect Failed");

    let mut client = ContainersClient::new(channel);

    let req = with_namespace!(DeleteContainerRequest { id: cid.clone() }, NAMESPACE);

    client
        .delete(req)
        .await
        .expect("Failed to delete container");

    println!("Container: {:?} deleted", cid);
}
