use protoc_rust::{Codegen, Customize};

fn main() {
   // Build our realtime feed structure
   Codegen::new()
      .out_dir("src/realtime")
      .inputs(&["src/realtime/data.proto"])
      .includes(&[ "src" ])
      .customize(Customize { ..Default::default() })
      .run()
      .expect("protoc");
}