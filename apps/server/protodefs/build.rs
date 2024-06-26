

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tonic_build::compile_protos("./protos/adventures.proto")?;
    
    println!("cargo:rerun-if-changed=NULL");


    tonic_build::configure()
    .build_server(true)
    .compile(
        &["../proto/fight/v1/fight.proto"],
        &["../proto/"],//"protos/fight"],
    )?;
    
    Ok(())
}