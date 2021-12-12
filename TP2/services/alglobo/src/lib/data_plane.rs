use crate::{config::data::Config, types::common::BoxResult};

// ----------------------------------------------------------------------------

pub struct DataPlane {}

impl DataPlane {
    pub fn new() -> BoxResult<Self> {
        println!("[DEBUG] (Data) Creating Config...");
        let Config { port: _ } = Config::new()?;

        // check config/control.rs code style
        // and try to use similar conventions
        // in both files

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
