use std::error::Error;
use redis::Commands;
use crate::modal::csvStruct::Record;

pub fn insert(csv_content: Vec<Record>) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let mut id = 0;
    for r in csv_content {
        con.hset_multiple(id, &[("FID", &r.FID), ("gid", &r.gid), ("UF", &r.UF), ("nome", &r.nome), ("Censo", &r.Censo), ("PIB", &r.PIB), ("Pop_est_2009", &r.Pop_est_2009), ("PIB_percapita", &r.PIB_percapita), ("Descricao", &r.Descricao), ("legenda", &r.legenda), ("classe", &r.classe), ("geom", &r.geom)])?;
        id = id + 1;
    }
    Ok(())
}

pub fn queryAll() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut all_values: Vec<String> = Vec::new();

    for i in 0..5566 {
        let values: Vec<String> = con.hvals(i)?;
        all_values.extend(values);
    }
    
    Ok(())
}

pub fn deleteAll() -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    let _: () = redis::cmd("FLUSHALL").query(&mut con)?;

    Ok(())
}

pub fn updateAll(records: Vec<Record>) -> Result<(), Box<dyn Error>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    for record in records {
        let uf: String = con.hget(&record.gid.to_string(), "UF")?;
        if uf == "SP" || uf == "SÃO PAULO" {
            let _: () = con.hset_multiple(
                &record.gid.to_string(),
                &[
                    ("FID", record.FID),
                    ("gid", record.gid),
                    ("UF", "IGAO".to_string()),
                    ("nome", record.nome.clone()),
                    ("Censo", record.Censo),
                    ("PIB", record.PIB),
                    ("Pop_est_2009", record.Pop_est_2009),
                    ("PIB_percapita", record.PIB_percapita),
                    ("Descricao", record.Descricao.clone()),
                    ("legenda", record.legenda.clone()),
                    ("classe", record.classe.clone()),
                    ("geom", record.geom.clone()),
                ],
            )?;
        }
    }

    Ok(())
}
