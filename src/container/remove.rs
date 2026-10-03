use containerd_client::Client;
use containerd_client::services::v1::DeleteImageRequest;
use containerd_client::tonic::Request;
use containerd_client::with_namespace;

const NAMESPACE: &str = "default";

#[tokio::main(flavor = "current_thread")]
pub async fn remove(image: &str) {
    let client = Client::from_path("/run/containerd/containerd.sock")
        .await
        .expect("Connect Failed");

    // TODO: Add a progressbar
    println!("Remove image {}", image);

    let req = with_namespace!(
        DeleteImageRequest {
            name: image.to_string(),
            sync: true,
            ..Default::default()
        },
        NAMESPACE
    );

    client
        .images()
        .delete(req)
        .await
        .expect("unable to remove image");
}
