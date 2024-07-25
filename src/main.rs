use swc_core::SWC_CORE_VERSION;
use anyhow::Result;

fn main() -> Result<()> {
  println!("{:?}", SWC_CORE_VERSION);
  Ok(())
}
