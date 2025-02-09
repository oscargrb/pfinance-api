use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Response {
    pub ok: bool,
    pub info: String,
}

#[derive(Serialize, Deserialize)]
pub struct Budget {
    pub ID: i32,
    pub month: i32,
    pub year: i32,
    pub tipo: String,
    pub categoria: String,
    pub concepto: String,
    pub presupuesto_usd: f64,
    pub presupuesto_bs: f64,
    pub promedio: f64, // Añade más campos según tu tabla
}

#[derive(Serialize, Deserialize)]
pub struct BudgetForm {
    pub month: i32,
    pub year: i32,
    pub tipo: i32,
    pub categoria: i32,
    pub concepto: String,
    pub presupuesto_usd: f64, // Añade más campos según tu tabla
}

#[derive(Serialize, Deserialize)]
pub struct Detail {
    pub fecha: String,
    pub month: i32,
    pub year: i32,
    pub nombre_presupuesto: String,
    pub detalle: String,
    pub monto_usd: f64,
    pub monto_bs: f64,
    pub promedio: f64, // Añade más campos según tu tabla
}

#[derive(Serialize, Deserialize)]
pub struct DetailForm {
    pub fecha: String,
    pub presupuesto: i32,
    pub detalle: String,
    pub monto_bs: f64, // Añade más campos según tu tabla
}

#[derive(Serialize, Deserialize)]
pub struct Resumen {
    pub categoria: String,
    pub tipo: String,
    pub monto_usd: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Dashboard {
    pub total_ingresos: f64,
    pub total_egresos: f64,
    pub total_ingresos_pre: f64,
    pub total_egresos_pre: f64,
    pub ingresos: Vec<Resumen>,
    pub egresos: Vec<Resumen>,
    pub ingresos_pre: Vec<Resumen>,
    pub egresos_pre: Vec<Resumen>,
    pub ing_vs_egr: Vec<Resumen>, // Añade más campos según tu tabla
}

#[derive(Serialize, Deserialize)]
pub struct Tipo {
    pub id: i32,
    pub tipo: String,
}

#[derive(Serialize, Deserialize)]
pub struct Categoria {
    pub id: i32,
    pub categoria: String,
    pub tipo: i32,
}

#[derive(Serialize, Deserialize)]
pub struct FormDataBudget {
    pub categorias: Vec<Categoria>,
    pub tipos: Vec<Tipo>,
}

#[derive(Serialize, Deserialize)]
pub struct BudgetSendToForm {
    pub id: i32,
    pub categoria: String,
    pub tipo: String,
    pub concepto: String,
}
