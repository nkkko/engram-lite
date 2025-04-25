fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile protocol buffers when the "grpc" feature is enabled
    #[cfg(feature = "grpc")]
    {
        println!("cargo:rerun-if-changed=proto/engram.proto");
        tonic_build::compile_protos("proto/engram.proto")?;
    }
    Ok(())
}