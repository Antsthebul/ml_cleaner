use deadpool_postgres::Object;
use postgres::Row;

pub struct MachineEventDb{
    client: Object
}
