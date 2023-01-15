fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
         .build_server(false)
         .compile(
             &["../../proto/service/v2/loper_db_service.proto"],
             &["../../proto/"]
         )?;
    Ok(())
 }