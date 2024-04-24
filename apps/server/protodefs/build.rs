

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //tonic_build::compile_protos("./protos/adventures.proto")?;
    
    println!("cargo:rerun-if-changed=NULL");


    tonic_build::configure()
    .build_server(true)
    .compile(
        &["protos/fight.proto"],
        &["protos/"],//"protos/fight"],
    )?;
    
    Ok(())
}