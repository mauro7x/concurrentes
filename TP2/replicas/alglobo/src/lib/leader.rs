use crate::types::BoxResult;

// ----------------------------------------------------------------------------

pub struct Leader {}

impl Leader {
    pub fn new() -> BoxResult<Self> {
        let ret = Leader {};
        Ok(ret)
    }

    pub fn run_iteration(&self) -> BoxResult<()> {
        println!("<Leader> Working...");
        std::thread::sleep(std::time::Duration::from_secs(3));
        println!("<Leader> Worked a lot!");
        Ok(())
    }
}
