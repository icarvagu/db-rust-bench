use postgres::{Client, NoTls};
use std::error::Error;
use crate::modal::csvStruct::Record;

pub fn insert(csv_content: Vec<Record>) -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect("postgresql://exampleuser:examplepass@localhost:5432/exampledb", NoTls)?;

    client.batch_execute(
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
            geom TEXT
        )",
    )?;

    for r in csv_content {
        client.execute(
            r"INSERT INTO records (FID, gid, UF, nome, Censo, PIB, Pop_est_2009, PIB_percapita, Descricao, legenda, classe, geom) 
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            &[&r.FID, &r.gid, &r.UF, &r.nome, &r.Censo, &r.PIB, &r.Pop_est_2009, &r.PIB_percapita, &r.Descricao, &r.legenda, &r.classe, &r.geom],
        )?;
    }

    Ok(())
}

pub fn queryAll() -> Result<(), postgres::Error> {
    let mut client = Client::connect("host=localhost user=exampleuser password=examplepass dbname=exampledb", NoTls)?;

    let statement = client.prepare("SELECT FID, gid, UF, nome, Censo, PIB, Pop_est_2009, PIB_percapita, Descricao, legenda, classe, geom FROM records")?;

    for row in client.query(&statement, &[])? {
        let FID: String = row.get(0);
        let gid: String = row.get(1);
        let UF: String = row.get(2);
        let nome: String = row.get(3);
        let Censo: String = row.get(4);
        let PIB: String = row.get(5);
        let Pop_est_2009: String = row.get(6);
        let PIB_percapita: String = row.get(7);
        let Descricao: String = row.get(8);
        let legenda: String = row.get(9);
        let classe: String = row.get(10);
        let geom: String = row.get(11);
    }

    Ok(())
}

pub fn deleteAll() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect("postgresql://exampleuser:examplepass@localhost:5432/exampledb", NoTls)?;

    client.execute("DELETE FROM records", &[])?;

    Ok(())
}