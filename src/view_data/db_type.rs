#[derive(Debug, Clone)]
pub enum DbType<T>
where
    T: Clone,
{
    /// For entries already in the db
    InDb(T),
    /// For futures that are currently awaited to be pushed
    DbActionPending(usize, T),
}

impl<T> DbType<T>
where
    T: Clone,
{
    pub const fn get_entry(&self) -> &T {
        match self {
            Self::InDb(ref x) | Self::DbActionPending(_, ref x) => x,
        }
    }

    pub fn get_entry_mut(&mut self) -> &mut T {
        match self {
            Self::InDb(ref mut x) | Self::DbActionPending(_, ref mut x) => x,
        }
    }

    /// Converts self to a database entry.
    /// This happens unchecked and the id associated with it will be forgotten
    pub fn convert_to_db(&mut self) {
        if let Self::DbActionPending(_, x) = self {
            *self = Self::InDb(x.clone());
        }
    }

    pub const fn new_future(id: usize, idea: T) -> Self {
        Self::DbActionPending(id, idea)
    }

    pub fn convert_to_db_action(&mut self, id: usize) -> Result<(), ()> {
        if let Self::InDb(x) = self {
            *self = Self::DbActionPending(id, x.clone());
            Ok(())
        } else {
            Err(())
        }
    }
}
