use crate::{
    control_plane::Control,
    data_plane::DataPlane,
    types::{BoxResult, Id},
};

// ----------------------------------------------------------------------------

pub struct Replica {
    id: Id,
    control: Control,
}

impl Replica {
    pub fn new() -> BoxResult<Self> {
        println!("[DEBUG] (ID: -) (Replica) Creating Control...");
        let control = Control::new()?;
        let id = control.get_my_id()?;

        let ret = Replica { id, control };
        println!("[DEBUG] (ID: {}) (Replica) Created successfully", id);

        Ok(ret)
    }

    pub fn run_iteration(&mut self) -> BoxResult<()> {
        println!("[INFO] (ID: {}) Replica started", self.id);
        self.inner_run()?;
        self.control.finish()?;
        println!("[INFO] (ID: {}) Terminated gracefully", self.id);

        Ok(())
    }

    // Private

    fn inner_run(&mut self) -> BoxResult<()> {
        loop {
            if self.control.am_i_leader()? {
                self.run_as_leader()?;
                break;
                // TODO: detect when there
                // is not more work to do
                // and finish gracefully
            } else {
                self.control.healthcheck_leader()?;
            }
        }

        // TEMP:
        Ok(())
    }

    fn run_as_leader(&self) -> BoxResult<()> {
        // let mut failed_requests_logger = FileLogger::new(failed_reqs_filename);

        let mut data_plane = DataPlane::new()?;

        while self.control.am_i_leader()? {
            data_plane.run_iteration()?;
        }

        // TODO: differentiate if we leave because
        // we are not leaders anymore of
        // because there is no more job to do
        // (¿maybe using a bool ret value?)

        Ok(())
    }
}