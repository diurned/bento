use containerd_client::Client;

/// Make sure you run containerd before running this example.
#[tokio::main(flavor = "current_thread")]
pub async fn version() {
    #[cfg(unix)]
    let path = "/var/run/containerd/containerd.sock";

    #[cfg(windows)]
    let path = r"\\.\pipe\containerd-containerd";

    let client = Client::from_path(path).await.expect("Connect failed");

    let resp = client
        .version()
        .version(())
        .await
        .expect("Failed to query version");

    println!("{:?}", resp.get_ref());
}
