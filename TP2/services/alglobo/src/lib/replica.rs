use crate::{
    control_plane::ControlPlane,
    data_plane::DataPlane,
    types::common::{BoxResult, Id},
    utils::fail_randomly,
};

// ----------------------------------------------------------------------------

pub struct Replica {
    id: Id,
    control: ControlPlane,
}

impl Replica {
    pub fn new() -> BoxResult<Self> {
        println!("[DEBUG] (ID: -) (Replica) Creating Control...");
        let control = ControlPlane::new()?;
        let id = control.get_my_id()?;
        fail_randomly()?;

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
        let mut finished = false;
        while !finished {
            fail_randomly()?;

            if self.control.am_i_leader()? {
                finished = self.run_as_leader()?;
            } else {
                self.control.healthcheck_leader()?;
            }
        }

        Ok(())
    }

    fn run_as_leader(&mut self) -> BoxResult<bool> {
        let mut data_plane = DataPlane::new(false)?;
        let mut transactions_pending = true;
        fail_randomly()?;

        while self.control.am_i_leader()? && transactions_pending {
            fail_randomly()?;
            transactions_pending = data_plane.process_transaction_from_file()?;
        }

        Ok(!transactions_pending)
    }
}
