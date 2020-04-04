extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
   // Build our realtime feed structure
   protoc_rust::run(protoc_rust::Args {
      out_dir: "src/realtime",
      input: &[ "src/realtime/data.proto" ],
      includes: &[ "src" ],
      customize: Customize { ..Default::default() },
   })
   .expect("protoc");
}