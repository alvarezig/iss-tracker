use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct IssPosition {
    pub name: String,
    pub id: u64,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub velocity: f64,
    pub visibility: String,
}

pub async fn fetch_iss_position() -> Result<IssPosition, reqwest::Error> {
    let url = "https://api.wheretheiss.at/v1/satellites/25544";
    let response = reqwest::get(url).await?;
    let iss: IssPosition = response.json().await?;
    Ok(iss)
}