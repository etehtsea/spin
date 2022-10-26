#![allow(dead_code)]
use anyhow::{anyhow, Result};
use spin_sdk::{
    http::{Request, Response},
    http_component, mysql::{self, Decode},
};

// The environment variable set in `spin.toml` that points to the
// address of the Pg server that the component will write to
const DB_URL_ENV: &str = "DB_URL";

#[derive(Debug, Clone)]
struct NumericRow {
    tiny: i8,
    small: i16,
    medium: i32,
    iint: i32,
    big: i64,
    float: f32,
    double: f64,
    utiny: u8,
    usmall: u16,
    umedium: u32,
    uint: u32,
    ubig: u64,
    boolflag: bool,
}

#[derive(Debug, Clone)]
struct CharacterRow {
    rvarchar: String,
    rtext: String,
    rchar: String,
    rbinary: Vec<u8>,
    rvarbinary: Vec<u8>,
    rblob: Vec<u8>
}

#[http_component]
fn process(req: Request) -> Result<Response> {
    match req.uri().path() {
        "/test_character_types" => test_character_types(req),
        "/test_numeric_types" => test_numeric_types(req),
        _ => Ok(http::Response::builder()
            .status(404)
            .body(Some("Not found".into()))?),
    }
}

fn test_numeric_types(_req: Request) -> Result<Response> {
    let address = std::env::var(DB_URL_ENV)?;

    let create_table_sql = r#"
        CREATE TEMPORARY TABLE test_numeric_types (
            rtiny TINYINT NOT NULL,
            rsmall SMALLINT NOT NULL,
            rmedium MEDIUMINT NOT NULL,
            rint INT NOT NULL,
            rbig BIGINT NOT NULL,
            rfloat FLOAT NOT NULL,
            rdouble DOUBLE NOT NULL,
            rutiny TINYINT UNSIGNED NOT NULL,
            rusmall SMALLINT UNSIGNED NOT NULL,
            rumedium MEDIUMINT UNSIGNED NOT NULL,
            ruint INT UNSIGNED NOT NULL,
            rubig BIGINT UNSIGNED NOT NULL,
            rbool TINYINT NOT NULL
         );
    "#;

    mysql::execute(&address, create_table_sql, &[])
        .map_err(|e| anyhow!("Error executing MySQL command: {:?}", e))?;

    let insert_sql = r#"
        INSERT INTO test_numeric_types
            (rtiny, rsmall, rmedium, rint, rbig, rfloat, rdouble, rutiny, rusmall, rumedium, ruint, rubig, rbool)
        VALUES
            (0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1);
    "#;

    mysql::execute(&address, insert_sql, &[])
        .map_err(|e| anyhow!("Error executing MySQL command: {:?}", e))?;

    let sql = r#"
        SELECT
            rtiny,
            rsmall,
            rmedium,
            rint,
            rbig,
            rfloat,
            rdouble,
            rutiny,
            rusmall,
            rumedium,
            ruint,
            rubig,
            rbool
        FROM test_numeric_types;
    "#;

    let rowset = mysql::query(&address, sql, &[])
        .map_err(|e| anyhow!("Error executing MySQL command: {:?}", e))?;

    let column_summary = rowset
        .columns
        .iter()
        .map(format_col)
        .collect::<Vec<_>>()
        .join(", ");

    let mut response_lines = vec![];

    for row in rowset.rows {
        let tiny = i8::decode(&row[0])?;
        let small = i16::decode(&row[1])?;
        let medium = i32::decode(&row[2])?;
        let iint = i32::decode(&row[3])?;
        let big = i64::decode(&row[4])?;
        let float = f32::decode(&row[5])?;
        let double = f64::decode(&row[6])?;
        let utiny = u8::decode(&row[7])?;
        let usmall = u16::decode(&row[8])?;
        let umedium = u32::decode(&row[9])?;
        let uint = u32::decode(&row[10])?;
        let ubig = u64::decode(&row[11])?;
        let boolflag = bool::decode(&row[12])?;

        let row = NumericRow {
            tiny,
            small,
            medium,
            iint,
            big,
            float,
            double,
            utiny,
            usmall,
            umedium,
            uint,
            ubig,
            boolflag,
        };

        response_lines.push(format!("row: {:#?}", row));
    }

    let response = format!(
        "Found {} rows(s) as follows:\n{}\n\n(Column info: {})\n",
        response_lines.len(),
        response_lines.join("\n"),
        column_summary,
    );

    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn test_character_types(_req: Request) -> Result<Response> {
    let address = std::env::var(DB_URL_ENV)?;

    let create_table_sql = r#"
        CREATE TEMPORARY TABLE test_character_types (
            rvarchar varchar(40) NOT NULL,
            rtext text NOT NULL,
            rchar char(10) NOT NULL,
            rbinary binary(10) NOT NULL,
            rvarbinary varbinary(10) NOT NULL,
            rblob BLOB NOT NULL
         );
    "#;

    mysql::execute(&address, create_table_sql, &[])
        .map_err(|e| anyhow!("Error executing MySQL command: {:?}", e))?;

    let insert_sql = r#"
        INSERT INTO test_character_types
            (rvarchar, rtext, rchar, rbinary, rvarbinary, rblob)
        VALUES
            ('rvarchar', 'rtext', 'rchar', 'a', 'a', 'a');
    "#;

    mysql::execute(&address, insert_sql, &[])
        .map_err(|e| anyhow!("Error executing MySQL command: {:?}", e))?;

    let sql = r#"
        SELECT
            rvarchar, rtext, rchar, rbinary, rvarbinary, rblob
        FROM test_character_types;
    "#;

    let rowset = mysql::query(&address, sql, &[])
        .map_err(|e| anyhow!("Error executing MySQL command: {:?}", e))?;

    let column_summary = rowset
        .columns
        .iter()
        .map(format_col)
        .collect::<Vec<_>>()
        .join(", ");

    let mut response_lines = vec![];

    for row in rowset.rows {
        let rvarchar = String::decode(&row[0])?;
        let rtext = String::decode(&row[1])?;
        let rchar = String::decode(&row[2])?;
        let rbinary = Vec::<u8>::decode(&row[3])?;
        let rvarbinary = Vec::<u8>::decode(&row[4])?;
        let rblob = Vec::<u8>::decode(&row[5])?;

        let row = CharacterRow {
            rvarchar,
            rtext,
            rchar,
            rbinary,
            rvarbinary,
            rblob,
        };

        response_lines.push(format!("row: {:#?}", row));
    }

    let response = format!(
        "Found {} rows(s) as follows:\n{}\n\n(Column info: {})\n",
        response_lines.len(),
        response_lines.join("\n"),
        column_summary,
    );

    Ok(http::Response::builder()
        .status(200)
        .body(Some(response.into()))?)
}

fn format_col(column: &mysql::Column) -> String {
    format!("{}: {:?}", column.name, column.data_type)
}
