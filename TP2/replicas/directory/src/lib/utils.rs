use crate::types::*;

pub fn next(id: Id) -> Id {
    (id + 1) % Id::MAX
}
