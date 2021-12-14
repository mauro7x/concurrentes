use crate::{
    control_plane::Control,
    data_plane::DataPlane,
    types::common::{BoxResult, Id},
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
        let mut finished = false;
        while !finished {
            if self.control.am_i_leader()? {
                finished = self.run_as_leader()?;
            } else {
                self.control.healthcheck_leader()?;
            }
        }

        Ok(())
    }

    fn run_as_leader(&mut self) -> BoxResult<bool> {
        let mut data_plane = DataPlane::new()?;
        let mut transactions_pending = true;

        while self.control.am_i_leader()? && transactions_pending {
            transactions_pending = data_plane.process_transaction()?;
        }

        Ok(!transactions_pending)
    }
}
