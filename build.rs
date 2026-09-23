use std::process::Command;

fn check_deps() {
    match Command::new("protoc").arg("--version").status() {
        Ok(status) if status.success() => {
            println!("protoc found");
        }
        Ok(_) => {
            eprintln!("protoc is installed but failed to run");
            std::process::exit(1);
        }
        Err(_) => {
            eprintln!("protoc not found");
            eprintln!("Install it with: \x1b[31mapt install -y protobuf-compiler\x1b[0m");
            std::process::exit(1);
        }
    }
}

fn main() {
    check_deps();
}
