
#[derive(Serialize, Deserialize)]
pub struct Status {
    pub name: String,
    pub hostname: String,
    pub is_up: bool
}

#[derive(Serialize)]
pub struct StatusFE {
    pub name: String,
    pub hostname: String,
    pub is_up: bool,
    pub leaf: Vec<Status>
}