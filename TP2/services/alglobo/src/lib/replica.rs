use crate::{
    control::Control,
    leader::Leader,
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

    pub fn run(&mut self) -> BoxResult<()> {
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
                // TODO: detect when there
                // is not more work to do
                // and finish gracefully
            } else {
                self.control.healthcheck_leader()?;
            }
        }

        // TEMP:
        // Ok(())
    }

    fn run_as_leader(&self) -> BoxResult<()> {
        let leader = Leader::new()?;
        while self.control.am_i_leader()? {
            leader.run_iteration()?;
        }

        // TODO: differentiate if we leave because
        // we are not leaders anymore of
        // because there is no more job to do
        // (¿maybe using a bool ret value?)

        Ok(())
    }
}
