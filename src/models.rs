use chrono::naive;

#[derive(Queryable)]
pub struct Massa {
    pub id: Option<i32>,
    pub ts: Option<naive::NaiveDateTime>,
    pub kg: Option<f32>,
    pub note_txt: Option<String>,
}
