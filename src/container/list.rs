use client::{
    services::v1::{
        ListContainersRequest, ListImagesRequest, containers_client::ContainersClient,
        images_client::ImagesClient,
    },
    tonic::Request,
    with_namespace,
};
use containerd_client as client;

const NAMESPACE: &str = "default";

#[tokio::main(flavor = "current_thread")]
pub async fn image() {
    let channel = client::connect("/run/containerd/containerd.sock")
        .await
        .expect("Connect failed");

    let mut client = ImagesClient::new(channel);
    let request = with_namespace!(ListImagesRequest { filters: vec![] }, NAMESPACE);
    let response = client.list(request).await.expect("Failed to list images");

    println!("IMAGES");
    for image in response.into_inner().images {
        println!("{}", image.name);
    }
}

#[tokio::main(flavor = "current_thread")]
pub async fn container() {
    let channel = client::connect("/run/containerd/containerd.sock")
        .await
        .expect("Connect failed");

    let mut client = ContainersClient::new(channel);
    let request = with_namespace!(ListContainersRequest { filters: vec![] }, NAMESPACE);
    let response = client.list(request).await.expect("Failed to list images");

    println!("CONTAINERS");
    for container in response.into_inner().containers {
        println!("{:?}", container);
    }
}
