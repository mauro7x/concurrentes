use crate::types::BoxResult;

// ----------------------------------------------------------------------------

pub struct DataPlane {}

impl DataPlane {
    pub fn new() -> BoxResult<Self> {
        let ret = DataPlane {};
        Ok(ret)
    }

    pub fn run_iteration(&self) -> BoxResult<()> {
        println!("[INFO] (Data) Working...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("[INFO] (Data) Finished work iteration");
        Ok(())
    }
}
