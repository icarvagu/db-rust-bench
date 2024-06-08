use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Clone)] 
pub struct Record {
    pub FID: String,
    pub gid: String,
    pub UF: String,
    pub nome: String, 
    pub Censo: String,
    pub PIB: String,
    pub Pop_est_2009: String,
    pub PIB_percapita: String,
    pub Descricao: String,
    pub legenda: String, 
    pub classe: String,
    pub geom: String
}
