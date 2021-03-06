use crate::{
    control_plane::ControlPlane,
    data_plane::DataPlane,
    types::common::{BoxResult, Id},
};

use log::*;

// ----------------------------------------------------------------------------

pub struct Replica {
    id: Id,
    control: ControlPlane,
}

impl Replica {
    pub fn new() -> BoxResult<Self> {
        debug!("(ID: -) (Replica) Creating Control...");
        let control = ControlPlane::new()?;
        let id = control.get_my_id()?;

        let ret = Replica { id, control };
        debug!("(ID: {}) (Replica) Created successfully", id);

        Ok(ret)
    }

    pub fn run(&mut self) -> BoxResult<()> {
        info!("(ID: {}) Replica started", self.id);
        self.inner_run()?;
        self.control.finish()?;
        info!("(ID: {}) Terminated gracefully", self.id);

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
        let mut data_plane = DataPlane::new(false)?;
        let mut transactions_pending = true;

        while self.control.am_i_leader()? && transactions_pending {
            transactions_pending = data_plane.process_transaction_from_file()?;
        }

        Ok(!transactions_pending)
    }
}
