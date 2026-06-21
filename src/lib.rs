use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read},
};

use geojson::{FeatureReader, GeoJson};
use turso_ext::{
    Connection, ResultCode, VTabCursor, VTabKind, VTabModule, VTabModuleDerive, VTable, Value,
    register_extension,
};

register_extension! {
    vtabs: { GeoJsonModule }
}

#[derive(Debug, VTabModuleDerive, Default)]
struct GeoJsonModule;

impl GeoJsonModule {
    fn parse_arg(arg: &Value) -> Result<(&str, &str), ResultCode> {
        if let Some(text) = arg.to_text() {
            let mut split = text.splitn(2, '=');
            let name = split.next();
            let value = split.next();
            if name.is_none() || value.is_none() {
                return Err(ResultCode::InvalidArgs);
            }
            return Ok((name.unwrap(), value.unwrap()));
        }
        Err(ResultCode::InvalidArgs)
    }
}

impl VTabModule for GeoJsonModule {
    type Table = GeoJsonTable;

    const VTAB_KIND: VTabKind = VTabKind::VirtualTable;

    const NAME: &'static str = "geojson";

    fn create(args: &[Value]) -> Result<(String, Self::Table), ResultCode> {
        if args.is_empty() {
            return Err(ResultCode::InvalidArgs);
        }
        let mut filename = None;

        for arg in args {
            let (name, value) = Self::parse_arg(arg)?;
            match name {
                "filename" => {
                    if filename.is_some() {
                        return Err(ResultCode::InvalidArgs);
                    }
                    // Todo: parse/validate input
                    filename = Some(value);
                }
                _ => return Err(ResultCode::InvalidArgs),
            }
        }

        if filename.is_none() {
            return Err(ResultCode::InvalidArgs);
        }

        let mut schema = "CREATE TABLE x(
                fid INTEGER
                geometry BLOB"
            .to_string();
        let mut table = GeoJsonTable {
            filename: Some(filename.unwrap().to_string()),
            column_count: 0,
        };

        let keys = table.get_columns()?;
        for (i, col) in keys.iter().enumerate() {
            schema.push('"');

            schema.push_str(col);
            // TODO: type inference. this lazily evaluates to text
            schema.push_str("\" TEXT");
            if i < keys.len() - 1 {
                schema.push_str(", ");
            }
        }
        schema.push(')');
        Ok((schema, table))
    }
}

struct VGeoJsonReader {
    reader: geojson::FeatureReader<BufReader<File>>,
}
struct GeoJsonTable {
    filename: Option<String>,
    column_count: u32,
}
impl GeoJsonTable {
    fn new_reader(&self) -> Result<VGeoJsonReader, ResultCode> {
        match &self.filename {
            Some(path) => {
                let file = File::open(path).map_err(|_| ResultCode::Error)?;
                let reader = BufReader::new(file);
                let geojson = FeatureReader::from_reader(reader);
                Ok(VGeoJsonReader { reader: geojson })
            }
            None => Err(ResultCode::Internal),
        }
    }

    fn get_columns(&mut self) -> Result<HashSet<String>, ResultCode> {
        let reader = self.new_reader()?;
        let mut keys: HashSet<String> = HashSet::new();
        for feature in reader.reader.features() {
            let feat = feature.map_err(|_| ResultCode::Error)?;
            if let Some(properties) = feat.properties {
                properties.keys().for_each(|k| {
                    keys.insert(k.to_owned());
                });
            }
        }
        Ok(keys)
    }
}

impl VTable for GeoJsonTable {
    type Cursor = GeoJsonCursor;

    type Error = ResultCode;

    fn open(&self, _conn: Option<std::sync::Arc<Connection>>) -> Result<Self::Cursor, Self::Error> {
        match self.new_reader() {
            Ok(reader) => Ok(GeoJsonCursor::new(reader, self)),
            Err(_) => Err(ResultCode::Error),
        }
    }
}

struct GeoJsonCursor {
    eof: bool,
    reader: VGeoJsonReader,
    row_number: usize,
    column_count: u32,
}
impl GeoJsonCursor {
    fn new(reader: VGeoJsonReader, table: &GeoJsonTable) -> Self {
        GeoJsonCursor {
            reader,
            row_number: 0,
            eof: false,
            column_count: table.column_count,
        }
    }
}
impl VTabCursor for GeoJsonCursor {
    type Error = ResultCode;

    fn filter(&mut self, args: &[Value], idx_info: Option<(&str, i32)>) -> ResultCode {
        self.row_number = 0;
        ResultCode::OK
    }

    fn rowid(&self) -> i64 {
        self.row_number as i64
    }

    fn column(&self, idx: u32) -> Result<Value, Self::Error> {
        if idx >= self.column_count  {
            return Ok(Value::null());
        }
        Ok(Value::null())
        // let value = self.current_row.get(idx as usize)
        // Ok(value)
    }

    fn eof(&self) -> bool {
        self.eof
    }

    fn next(&mut self) -> ResultCode {
        // Add logic to loop through data
        self.row_number += 1;
        ResultCode::OK
    }
}
