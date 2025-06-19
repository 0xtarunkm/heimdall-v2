// build.rs
use std::io::Result;

fn main() -> Result<()> {
    // Compile Protobuf definitions
    prost_build::compile_protos(&["proto/events.proto"], &["proto/"])?;

    Ok(())
}