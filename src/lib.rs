use turso_ext::{
    Connection, ResultCode, VTabCursor, VTabKind, VTabModule, VTabModuleDerive, VTable, Value,
    register_extension,
};

register_extension! {
    vtabs: { GeoJsonModule }
}

#[derive(Debug, VTabModuleDerive, Default)]
struct GeoJsonModule;

impl GeoJsonModule {}

impl VTabModule for GeoJsonModule {
    type Table = GeoJsonTable;

    const VTAB_KIND: VTabKind = VTabKind::VirtualTable;

    const NAME: &'static str = "geojson";

    fn create(args: &[Value]) -> Result<(String, Self::Table), ResultCode> {
        todo!()
    }
}

struct GeoJsonTable {}

impl VTable for GeoJsonTable {
    type Cursor = GeoJsonCursor;

    // Stub for now
    type Error = ResultCode;

    fn open(&self, _conn: Option<std::sync::Arc<Connection>>) -> Result<Self::Cursor, Self::Error> {
        todo!()
    }
}

struct GeoJsonCursor;
impl VTabCursor for GeoJsonCursor {
    type Error = ResultCode;

    fn filter(&mut self, args: &[Value], idx_info: Option<(&str, i32)>) -> ResultCode {
        todo!()
    }

    fn rowid(&self) -> i64 {
        todo!()
    }

    fn column(&self, idx: u32) -> Result<Value, Self::Error> {
        todo!()
    }

    fn eof(&self) -> bool {
        todo!()
    }

    fn next(&mut self) -> ResultCode {
        todo!()
    }
}
