use random;
use random::Source;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::result::Result;
use tensorflow::Code;
use tensorflow::Graph;
use tensorflow::ImportGraphDefOptions;
use tensorflow::Session;
use tensorflow::SessionOptions;
use tensorflow::SessionRunArgs;
use tensorflow::Status;
use tensorflow::Tensor;


#[cfg_attr(feature = "examples_system_alloc", global_allocator)]
#[cfg(feature = "examples_system_alloc")]
static ALLOCATOR: std::alloc::System = std::alloc::System;


fn main() -> Result<(), Box<dyn Error>> {
    let filename = "ml/model/saved_model.pb"; // y = w * x + b
    if !Path::new(filename).exists() {
        return Err(Box::new(
            Status::new_set(
                Code::NotFound,
                &format!(
                    "Run 'python regression.py' to generate \
                     {} and try again.",
                    filename
                ),
            )
                .unwrap(),
        ));
    }

    Ok(())
}
