use protobuf_codegen_pure::{ Codegen, Customize };

fn main() {
   // Build our realtime feed structure
   Codegen::new()
      .out_dir("src/yahoo")
      .inputs(&["src/yahoo/realtime.proto"])
      .includes(&[ "src" ])
      .customize(Customize { ..Default::default() })
      .run()
      .expect("Codegen failed.");
}
