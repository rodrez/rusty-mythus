#[derive(Queryable)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub publish: bool
}