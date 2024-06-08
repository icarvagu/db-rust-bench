use mongodb::{bson::doc, options::ClientOptions, Client, Collection};
use std::error::Error;
use crate::modal::csvStruct::Record;
use futures_util::stream::TryStreamExt;

pub async fn insert(csv_content: Vec<Record>) -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("exampledb");
    let coll = db.collection("records");

    for record in csv_content {
        coll.insert_one(doc! { 
            "FID": record.FID,
            "gid": record.gid,
            "UF": record.UF,
            "nome": record.nome,
            "Censo": record.Censo,
            "PIB": record.PIB,
            "Pop_est_2009": record.Pop_est_2009,
            "PIB_percapita": record.PIB_percapita,
            "Descricao": record.Descricao,
            "legenda": record.legenda,
            "classe": record.classe,
            "geom": record.geom
        }, None).await?;
    }

    Ok(())
}

pub async fn queryAll() -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("exampledb");
    let coll = db.collection("records");

    let mut cursor = coll.find(None, None).await?;
    let mut records = Vec::new();

    while let Some(result) = cursor.try_next().await? {
        let record = bson::from_document::<Record>(result)?;
        records.push(record);
    }

    Ok(())
}

pub async fn deleteAll() -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;

    client.database("exampledb").drop(None).await?;

    Ok(())
}

pub async fn updateAll(csv_content: Vec<Record>) -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let client_options = ClientOptions::parse(client_uri).await?;
    let client = Client::with_options(client_options)?;

    let db = client.database("exampledb");
    let coll: mongodb::Collection<bson::Document> = db.collection("records");

    for record in csv_content {
        coll.update_many(
            doc! { "UF": { "$in": ["SÃO PAULO", "SP"] } },
            doc! { 
                "$set": {
                    "FID": record.FID,
                    "gid": record.gid,
                    "UF": "IGÃO",
                    "nome": record.nome,
                    "Censo": record.Censo,
                    "PIB": record.PIB,
                    "Pop_est_2009": record.Pop_est_2009,
                    "PIB_percapita": record.PIB_percapita,
                    "Descricao": record.Descricao,
                    "legenda": record.legenda,
                    "classe": record.classe,
                    "geom": record.geom
                }
            },
            None
        ).await?;
    }
    Ok(())
}