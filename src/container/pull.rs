use std::env::consts;

use client::{
    services::v1::{TransferOptions, TransferRequest, transfer_client::TransferClient},
    to_any,
    types::{
        Platform,
        transfer::{ImageStore, OciRegistry, UnpackConfiguration},
    },
    with_namespace,
};
use containerd_client as client;
use containerd_client::tonic::Request;

const NAMESPACE: &str = "default";

/// Make sure you run containerd before running this example.
/// NOTE: to run this example, you must prepare a rootfs.
#[tokio::main(flavor = "current_thread")]
pub async fn pull(image: &str) {
    let arch = match consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => consts::ARCH,
    };

    let channel = client::connect("/run/containerd/containerd.sock")
        .await
        .expect("Connect Failed");
    let mut client = TransferClient::new(channel.clone());

    // Create the source (OCIRegistry)
    let source = OciRegistry {
        reference: image.to_string(),
        resolver: Default::default(),
    };

    let platform = Platform {
        os: "linux".to_string(),
        architecture: arch.to_string(),
        variant: "".to_string(),
        os_version: "".to_string(),
        // os_features: vec![],
    };

    // Create the destination (ImageStore)
    let destination = ImageStore {
        name: image.to_string(),
        platforms: vec![platform.clone()],
        unpacks: vec![UnpackConfiguration {
            platform: Some(platform),
            ..Default::default()
        }],
        ..Default::default()
    };

    let anys = to_any(&source);
    let anyd = to_any(&destination);

    // TODO: Add a progressbar
    println!("Pulling image for linux/{} from source: {:?}", arch, source);

    // Create the transfer request
    let request = TransferRequest {
        source: Some(anys),
        destination: Some(anyd),
        options: Some(TransferOptions {
            ..Default::default()
        }),
    };
    // Execute the transfer (pull)
    client
        .transfer(with_namespace!(request, NAMESPACE))
        .await
        .expect("unable to transfer image");
}
