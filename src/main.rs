mod db;
mod utils;
mod modal;
use std::time::Instant;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Escolha o banco de dados: (mysql, postgres, redis ou mongodb)");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let start = Instant::now();
    process(&input.trim()).await?;
    println!("Tempo total de processamento: {:?}", start.elapsed());
    Ok(())
}


async fn process(database: &str) -> Result<(), Box<dyn Error>> {
    let records = utils::csvUtils::read_csv("netflix_titles.csv")?;
    
    match database {
        "mysql" => process_mysql(records)?,
        "postgres" => process_postgres(records)?,
        "mongodb" => process_mongodb(records).await?,
        "redis" => process_redis(records)?,
        _ => return Err("Banco de dados não suportado".into()),
    }
    
    Ok(())
}

fn process_mysql(records: Vec<modal::csvStruct::Record>) -> Result<(), Box<dyn Error>> {
    let startInsert = Instant::now();
    db::mysql::insert(records)?;
    println!("Tempo de processamento do Insert MySQL: {:?}", startInsert.elapsed());

    let startQuery = Instant::now();
    db::mysql::queryAll()?;
    println!("Tempo de processamento da Query MySQL: {:?}", startQuery.elapsed());
    
    let startDelete = Instant::now();
    db::mysql::deleteAll()?;
    println!("Tempo de processamento do Delete MySQL: {:?}", startDelete.elapsed());
    
    Ok(())
}

fn process_postgres(records: Vec<modal::csvStruct::Record>) -> Result<(), Box<dyn Error>> {
    let startInsert = Instant::now();
    db::postgree::insert(records)?;
    println!("Tempo de processamento do Insert PostgreSQL: {:?}", startInsert.elapsed());

    let startQuery = Instant::now();
    db::postgree::queryAll()?;
    println!("Tempo de processamento da Query PostgreSQL: {:?}", startQuery.elapsed());
    
    let startDelete = Instant::now();
    db::postgree::queryAll()?;
    println!("Tempo de processamento do Delete PostgreSQL: {:?}", startDelete.elapsed());
    Ok(())
}

async fn process_mongodb(records: Vec<modal::csvStruct::Record>) -> Result<(), Box<dyn Error>> {
    let startInsert = Instant::now();
    match db::mongo::insert(records.clone()).await {
        Ok(()) => println!("Tempo de processamento do Insert MongoDB: {:?}", startInsert.elapsed()),
        Err(err) => return Err(err.into()),
    }

    let startQuery = Instant::now();
    match db::mongo::queryAll().await {
        Ok(()) => println!("Tempo total de processamento da Query MongoDB:: {:?}", startQuery.elapsed()),
        Err(err) => return Err(err.into()),
    }
    
    let startUpdate = Instant::now();
    match db::mongo::updateAll(records.clone()).await {
        Ok(()) => println!("Tempo total de processamento do Update MongoDB:: {:?}", startUpdate.elapsed()),
        Err(err) => return Err(err.into()),
    }

    let startDelete = Instant::now();
    match db::mongo::deleteAll().await {
        Ok(()) => println!("Tempo total de processamento do Delete MongoDB:: {:?}", startDelete.elapsed()),
        Err(err) => return Err(err.into()),
    }
    Ok(())
}

fn process_redis(records: Vec<modal::csvStruct::Record>) -> Result<(), Box<dyn Error>> {
    let startInsert = Instant::now();
    db::redis::insert(records.clone());
    println!("Tempo de processamento do Insert Redis: {:?}", startInsert.elapsed());
    
    let startQuery = Instant::now();
    db::redis::queryAll();
    println!("Tempo de processamento da Query Redis: {:?}", startQuery.elapsed());

    let startUpdate = Instant::now();
    db::redis::updateAll(records.clone());
    println!("Tempo de processamento do Update Redis: {:?}", startUpdate.elapsed());

    let startDelete = Instant::now();
    db::redis::deleteAll();
    println!("Tempo de processamento do Delete Redis: {:?}", startDelete.elapsed());
    Ok(())
}

