// https://www.sqlite.org/datatype3.html

/// SQL statements used in the tenant manager.
pub(crate) enum SqlStatement
{
    CreateMasterDb,
    SelectTenantsOnLoad,
    InsertAddTenant,
    DeleteRemoveTenant,
}

impl SqlStatement
{
    pub(crate) fn as_str(&self) -> &'static str
    {
        match self {
            SqlStatement::CreateMasterDb => {
                "
                CREATE TABLE IF NOT EXISTS tenants (
                    id TEXT PRIMARY KEY,
                    tenant_id TEXT,
                    tenant_path TEXT,
                    tenant_has_path INTEGER,
                    created_at TEXT
                )"
            }
            SqlStatement::SelectTenantsOnLoad => "SELECT id, tenant_path, tenant_has_path FROM tenants",
            SqlStatement::InsertAddTenant => {
                "INSERT INTO tenants (tenant_id, tenant_path, tenant_has_path, created_at) VALUES (?1, ?2, ?3, \
                 CURRENT_TIMESTAMP)"
            }
            SqlStatement::DeleteRemoveTenant => "DELETE FROM tenants WHERE id = ?1",
        }
    }
}
