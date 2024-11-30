// storage/src/lib.rs

// dependencies
use opendal::Buffer;
use opendal::Error;
use opendal::Operator;
use opendal::Result;
use opendal::services::Fs;

// trait definitions
trait Dal {
  fn read(&self) -> Result<Buffer, Error>; 
}

// struct type to represent an OpenDal service
struct DalService {
  op: Operator,
}

// implement the Dal trait for the DalService
impl Dal for DalService {
  async fn read(&self) -> Result<Buffer> {
    let bytes = self.op.read("puzzle_data/").await?;

    Ok(bytes)
  }
}