mod convert;
mod dialect;
mod fetcher;
mod loader;
use convert::Sql;
use fetcher::retrieve_data;
use loader::detect_content;

use anyhow::{anyhow, Result};
pub use dialect::example_sql;
pub use dialect::HcDialect;
use polars::frame::DataFrame;
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::io::Cursor;
use std::ops::{Deref, DerefMut};
use tracing::info;

#[derive(Debug)]
pub struct DataSet(DataFrame);

/// 让 DataSet 用起来和 DataFrame 一致
impl Deref for DataSet {
    type Target = DataFrame;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    /// 从 DataSet 转换成 csv
    pub fn to_csv(&mut self) -> Result<String> {
        let mut buf = Cursor::new(Vec::new());
        CsvWriter::new(&mut buf).finish(&mut self.0)?;

        let csv_content = String::from_utf8(buf.into_inner()).map_err(|e| {
            polars::error::PolarsError::ComputeError(format!("UTF-8 error: {}", e).into())
        })?;

        Ok(csv_content)
    }
}

/// 从 from 中获取数据，从 where 中过滤，最后选取需要返回的列
pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&HcDialect::default(), sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    let sql = &ast[0];

    let Sql {
        source,
        condition,
        selection,
        offset,
        limit,
        order_by,
    } = sql.try_into()?;

    info!("retrieving data from source: {}", source);

    // 从 source 读入一个 DataSet
    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    // filtered = order_by
    //     .into_iter()
    //     .fold(filtered, |acc, (col, desc)| acc.sort(&col, desc));

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0), limit.unwrap_or(usize::MAX) as u32);
    }

    Ok(DataSet(filtered.select(selection).collect()?))
}
