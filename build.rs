
fn main(){
    let proto_file = "./config/login.proto";
    tonic_build::configure()
    .build_server(true)
    .out_dir("./src/grpc_models")
    .compile(&[proto_file], &["."])
    .unwrap_or_else(|e| panic!("protobuf compile error: {}", e))
}