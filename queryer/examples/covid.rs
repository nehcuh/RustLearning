use anyhow::Result;
use polars::{io::SerReader, prelude::CsvReadOptions, series::ChunkCompare};
use queryer::query;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // let url = "https://raw.githubusercontent.com/owid/owid-datasets/master/datasets/COVID-2019%20-%20ECDC%20(2020)/COVID-2019%20-%20ECDC%20(2020).csv";
    /*
    let data = reqwest::get(url).await?.text().await?;

    // 指定 column 使用的数据类型
    // let schema = Schema::new().set_dtype(
    //     "Total confirmed cases of COVID-19",
    //     polars::datatypes::DataType::Float64,
    // );

    // 使用 polars 直接请求
    // let mut df = CsvReader::new(Cursor::new(data)).finish()?;
    let csv_options = CsvReadOptions {
        n_rows: Some(10000),
        infer_schema_length: Some(1000),
        ..Default::default()
    };
    let df = csv_options
        .into_reader_with_file_handle(Cursor::new(data))
        .finish()?;
    // df.try_apply("Total confirmed cases of COVID-19", |s: &Series| {
    //     s.cast(&polars::datatypes::DataType::Float32)
    // })?;
    // println!("{:?}", df);
    let mask = df.column("Total confirmed cases of COVID-19")?.gt(500)?;
    let filtered = df.filter(&mask)?;
    println!(
        "{:?}",
        filtered.select([
            "Entity",
            "Year",
            "Daily new confirmed cases of COVID-19",
            "Daily new confirmed deaths due to COVID-19",
            "Total confirmed cases of COVID-19",
            "Total confirmed deaths due to COVID-19",
            "Daily new confirmed cases of COVID-19 per million people",
            "Daily new confirmed deaths due to COVID-19 per million people",
            "Total confirmed cases of COVID-19 per million people",
            "Total confirmed deaths due to COVID-19 per million people",
            "Days since the total confirmed cases of COVID-19 reached 100",
            "Days since the total confirmed deaths of COVID-19 reached 5",
            "Days since the total confirmed cases of COVID-19 per million people reached 1",
            "Days since the total confirmed deaths of COVID-19 per million people reached 0.1",
            "Case fatality rate of COVID-19 (%)",
            "Case fatality rate of COVID-19 (%) (Only observations with ≥100 cases)",
            "Days since 30 daily new confirmed cases recorded",
        ])
    );
    */
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";
    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
        FROM {} where new_deaths >= 500 ORDER BY new_cases DESC",
        url
    );
    let df1 = query(sql).await?;
    println!("{:?}", df1);
    Ok(())
}
