use std::net::IpAddr;
use uuid::Uuid;
use time::Timespec;

use frame::frame_result::{RowsMetadata, ColType, ColSpec, BodyResResultRows, ColTypeOption,
                          ColTypeOptionValue};
use types::{CBytes, IntoRustByName, ByName, IntoRustByIndex, ByIndex};
use types::data_serialization_types::*;
use types::list::List;
use types::map::Map;
use types::udt::UDT;
use types::tuple::Tuple;
use error::{Error, Result, column_is_empty_err};

#[derive(Debug)]
pub struct Row {
    metadata: RowsMetadata,
    row_content: Vec<CBytes>,
}

impl Row {
    pub fn from_frame_body(body: BodyResResultRows) -> Vec<Row> {
        body.rows_content
            .iter()
            .map(|row| {
                     Row {
                         metadata: body.metadata.clone(),
                         row_content: row.clone(),
                     }
                 })
            .collect()
    }

    fn get_col_spec_by_name(&self, name: &str) -> Option<(&ColSpec, &CBytes)> {
        self.metadata
            .col_specs
            .iter()
            .position(|spec| spec.name.as_str() == name)
            .map(|i| {
                     let ref col_spec = self.metadata.col_specs[i];
                     let ref data = self.row_content[i];
                     (col_spec, data)
                 })
    }

    fn get_col_spec_by_index(&self, index: usize) -> Option<(&ColSpec, &CBytes)> {
        let specs = self.metadata.col_specs.iter();
        let values = self.row_content.iter();
        specs.zip(values).nth(index)
    }
}

impl IntoRustByName<Vec<u8>> for Row {
    fn get_by_name(&self, name: &str) -> Result<Option<Vec<u8>>> {
        self.get_col_spec_by_name(name)
            .ok_or(column_is_empty_err())
            .and_then(|(col_spec, cbytes)| {
                          let ref col_type = col_spec.col_type;
                          as_rust_type!(col_type, cbytes, Vec<u8>)
                      })
    }
}

impl ByName for Row {}

into_rust_by_name!(Row, String);
into_rust_by_name!(Row, bool);
into_rust_by_name!(Row, i64);
into_rust_by_name!(Row, i32);
into_rust_by_name!(Row, i16);
into_rust_by_name!(Row, i8);
into_rust_by_name!(Row, f64);
into_rust_by_name!(Row, f32);
into_rust_by_name!(Row, IpAddr);
into_rust_by_name!(Row, Uuid);
into_rust_by_name!(Row, List);
into_rust_by_name!(Row, Map);
into_rust_by_name!(Row, UDT);
into_rust_by_name!(Row, Tuple);
into_rust_by_name!(Row, Timespec);

impl ByIndex for Row {}

into_rust_by_index!(Row, String);
into_rust_by_index!(Row, bool);
into_rust_by_index!(Row, i64);
into_rust_by_index!(Row, i32);
into_rust_by_index!(Row, i16);
into_rust_by_index!(Row, i8);
into_rust_by_index!(Row, f64);
into_rust_by_index!(Row, f32);
into_rust_by_index!(Row, IpAddr);
into_rust_by_index!(Row, Uuid);
into_rust_by_index!(Row, List);
into_rust_by_index!(Row, Map);
into_rust_by_index!(Row, UDT);
into_rust_by_index!(Row, Tuple);
into_rust_by_index!(Row, Timespec);
