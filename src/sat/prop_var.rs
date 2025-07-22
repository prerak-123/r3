use super::decls::VarID;

#[derive(Debug)]
struct PropVar {
    id: VarID,
}

impl PropVar {
    fn new(id: VarID) -> Self {
        PropVar { id }
    }

    fn id(&self) -> VarID {
        self.id
    }
}
