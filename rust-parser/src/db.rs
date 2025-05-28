use postgres::{Client, NoTls, Error};

pub struct Db {
    client: Client,
}

impl Db {
    pub fn connect(conn_str: &str) -> Result<Self, Error> {
        let client = Client::connect(conn_str, NoTls)?;
        Ok(Db { client })
    }

    pub fn execute_query(&mut self, query: &str, params: &[&(dyn postgres::types::ToSql + Sync)]) -> Result<u64, Error> {
        self.client.execute(query, params)
    }

    pub fn query_rows(&mut self, query: &str, params: &[&(dyn postgres::types::ToSql + Sync)]) -> Result<Vec<postgres::Row>, Error> {
        let rows = self.client.query(query, params)?;
        Ok(rows)
    }
}

