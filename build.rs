
use io::EdgewareDaoMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;


fn main(){

   WasmBuilder::with_meta(EdgewareDaoMetadata::repr()).exclude_features(vec!["binary-vendor"]).build();
}