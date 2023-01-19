fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
         .build_server(cfg!(feature = "grpc-server"))
         .build_client(cfg!(feature = "grpc-client"))
         .compile(
             &["../../proto/service/v2/loper_db_service.proto"],
             &["../../proto/"]
         )?;
    Ok(())
 }