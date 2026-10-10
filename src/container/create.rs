use std::{fs, fs::File};

use client::{
    services::v1::{
        Container, CreateContainerRequest, CreateTaskRequest, StartRequest, container::Runtime,
        containers_client::ContainersClient, tasks_client::TasksClient,
    },
    tonic::Request,
    with_namespace,
};
use containerd_client as client;
use prost_types::Any;

use crate::image::Image;

const NAMESPACE: &str = "default";

#[tokio::main(flavor = "current_thread")]
pub async fn start(cid: String, image: Image) {
    dbg!(image.clone());
    let channel = client::connect("/run/containerd/containerd.sock")
        .await
        .expect("Connect Failed");

    let mut client = ContainersClient::new(channel.clone());

    let spec = include_str!("container_spec.json");
    let spec = spec
        .to_string()
        .replace("$ROOTFS", "/tmp/busybox/bundle/rootfs")
        .replace("$OUTPUT", "hello rust client");

    let spec = Any {
        type_url: "types.containerd.io/opencontainers/runtime-spec/1/Spec".to_string(),
        value: spec.into_bytes(),
    };

    let container = Container {
        id: cid.clone(),
        image: image.as_ref().to_string(),
        runtime: Some(Runtime {
            name: "io.containerd.runc.v2".to_string(),
            options: None,
        }),
        spec: Some(spec),
        ..Default::default()
    };

    let req = CreateContainerRequest {
        container: Some(container),
    };
    let req = with_namespace!(req, NAMESPACE);

    let _resp = client
        .create(req)
        .await
        .expect("Failed to create container");

    println!("Container: {:?} created", cid.clone());

    // create temp dir for stdin/stdout/stderr
    let tmp = std::env::temp_dir().join("containerd-client-test");
    fs::create_dir_all(&tmp).expect("Failed to create temp directory");
    let stdin = tmp.join("stdin");
    let stdout = tmp.join("stdout");
    let stderr = tmp.join("stderr");
    File::create(&stdin).expect("Failed to create stdin");
    File::create(&stdout).expect("Failed to create stdout");
    File::create(&stderr).expect("Failed to create stderr");

    // creat and start task
    let mut client = TasksClient::new(channel.clone());

    let req = CreateTaskRequest {
        container_id: cid.clone(),
        stdin: stdin.to_str().unwrap().to_string(),
        stdout: stdout.to_str().unwrap().to_string(),
        stderr: stderr.to_str().unwrap().to_string(),
        ..Default::default()
    };
    let req = with_namespace!(req, NAMESPACE);

    let _resp = client.create(req).await.expect("Failed to create task");

    println!("Task: {:?} created", cid.clone());

    let req = StartRequest {
        container_id: cid.clone(),
        ..Default::default()
    };
    let req = with_namespace!(req, NAMESPACE);

    let _resp = client.start(req).await.expect("Failed to start task");

    println!("Task: {:?} started", cid);
}
