use client::{
    services::v1::{ListImagesRequest, images_client::ImagesClient},
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

// pub fn container() {}
