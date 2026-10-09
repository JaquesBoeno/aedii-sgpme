use crate::models::DataBase;

pub fn run(db: &DataBase) {
    crate::missao::planejar_missao_extracao(db);
}
