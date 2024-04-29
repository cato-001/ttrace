use std::rc::Rc;

use rusqlite::{Connection, OpenFlags};

use crate::config::Config;

pub fn open_database_connection(config: &Config) -> eyre::Result<Rc<Connection>> {
    let path = config.database_path();
    let flags = OpenFlags::default();
    let connection = Connection::open_with_flags(path, flags)?;
    Ok(connection.into())
}
