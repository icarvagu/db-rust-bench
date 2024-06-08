use mysql::*;
use mysql::prelude::*;
use std::error::Error;
use crate::modal::csvStruct::Record;

pub fn insert(csv_content: Vec<Record>) -> std::result::Result<(), Box<dyn Error>> {
    let mysql_url = "mysql://exampleuser:examplepass@localhost:3306/exampledb";
    let pool = Pool::new(mysql_url)?;
    let mut conn = pool.get_conn()?;

    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS records (
            FID VARCHAR(255),
            gid VARCHAR(255),
            UF VARCHAR(255),
            nome VARCHAR(255),
            Censo VARCHAR(255),
            PIB VARCHAR(255),
            Pop_est_2009 VARCHAR(255),
            PIB_percapita VARCHAR(255),
            Descricao VARCHAR(255),
            legenda VARCHAR(255),
            classe VARCHAR(255),
            geom LONGTEXT
        )"
    )?;

    for r in csv_content {
        conn.exec_drop(
            r"INSERT INTO records (FID, gid, UF, nome, Censo, PIB, Pop_est_2009, PIB_percapita, Descricao, legenda, classe, geom) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            (r.FID, r.gid, r.UF, r.nome, r.Censo, r.PIB, r.Pop_est_2009, r.PIB_percapita, r.Descricao, r.legenda, r.classe, r.geom)
        )?;
    }

    Ok(())
}

pub fn queryAll() -> std::result::Result<(), Box<dyn Error>> {
    let mysql_url = "mysql://exampleuser:examplepass@localhost:3306/exampledb";
    let pool = Pool::new(mysql_url)?;
    let mut conn = pool.get_conn()?;

    let _query_result = conn.query_map(
        "SELECT FID, gid, UF, nome, Censo, PIB, Pop_est_2009, PIB_percapita, Descricao, legenda, classe, geom FROM records",
        |(FID, gid, UF, nome, Censo, PIB, Pop_est_2009, PIB_percapita, Descricao, legenda, classe, geom)| {
            Record {
                FID,
                gid,
                UF,
                nome,
                Censo,
                PIB,
                Pop_est_2009,
                PIB_percapita,
                Descricao,
                legenda,
                classe,
                geom
            }
        },
    )?;

    Ok(())
}

pub fn deleteAll() -> std::result::Result<(), Box<dyn Error>> {
    let mysql_url = "mysql://exampleuser:examplepass@localhost:3306/exampledb";
    let pool = Pool::new(mysql_url)?;
    let mut conn = pool.get_conn()?;

    conn.query_drop("DELETE FROM records")?;

    Ok(())
}