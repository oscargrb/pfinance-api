use crate::db::DbClient;
use crate::model::{
    Budget, BudgetForm, BudgetSendToForm, Categoria, Dashboard, Detail, DetailForm, FormDataBudget,
    Response, Resumen, Tipo,
};
use crate::read_query::read_query;
use rocket::serde::json::Json;
use rocket::State;

#[get("/")]
pub async fn index(_db: &State<DbClient>) -> Json<Response> {
    Json(Response {
        ok: true,
        info: String::from("Welcome to Personal Finance API!"),
    })
}

// BUDGET

#[post("/budget", data = "<budget>")]
pub async fn submit_budget(budget: Json<BudgetForm>, db: &State<DbClient>) -> Json<Response> {
    println!("{}", format!(
        "Received budget for month: {:?}, year: {:?}, tipo: {:?}, categoria: {:?}, concepto: {:?}, presupuesto_usd: {:?}",
        budget.month, budget.year, budget.tipo, budget.categoria, budget.concepto, budget.presupuesto_usd
    ));

    let findTasa = get_current_tasa(budget.month, budget.year, &db).await;

    let bs = budget.presupuesto_usd * findTasa.1;

    let _save_budged =
        db.0.query(
            read_query("Budget/Submit_Budget").as_str(),
            &[
                &budget.month,
                &budget.year,
                &budget.tipo,
                &budget.categoria,
                &budget.concepto,
                &budget.presupuesto_usd,
                &bs,
                &findTasa.0,
            ],
        )
        .await
        .unwrap();

    //println!("{}", find_flujo_trabajo.first());

    Json(Response {
        ok: true,
        info: String::from("recibido budged!"),
    })
}

#[get("/budget/<year>/<month>")]
pub async fn get_budget(year: i32, month: i32, db: &State<DbClient>) -> Json<Vec<Budget>> {
    let get_budget =
        db.0.query(read_query("Budget/Get_Budget").as_str(), &[&month, &year])
            .await
            .unwrap();

    let budgets: Vec<Budget> = get_budget
        .iter()
        .map(|row| Budget {
            ID: row.get("ID"),
            month: row.get("Month"),
            year: row.get("Year"),
            tipo: row.get("Tipo"),
            categoria: row.get("Categoria"),
            concepto: row.get("Concepto"),
            presupuesto_usd: row.get("PresupuestoUSD"),
            presupuesto_bs: row.get("PresupuestoBS"),
            promedio: row.get("Promedio"),
        })
        .collect();

    Json(budgets)
}

// DETAILS

#[post("/detail", data = "<detail>")]
pub async fn submit_detail(detail: Json<DetailForm>, db: &State<DbClient>) -> Json<Response> {
    println!(
        "{}",
        format!(
            "Received detail for date: {:?}, budged: {:?}, details: {:?}, montoUSD: {:?}",
            detail.fecha, detail.presupuesto, detail.detalle, detail.monto_bs
        )
    );

    let dolar = detail.monto_bs / 61.04;

    let _save_detail =
        db.0.query(
            read_query("Detail/Submit_Detail").as_str(),
            &[
                &detail.fecha,
                &detail.presupuesto,
                &detail.detalle,
                &dolar,
                &detail.monto_bs,
            ],
        )
        .await
        .unwrap();

    //println!("{}", find_flujo_trabajo.first());

    Json(Response {
        ok: true,
        info: String::from("recibido Detail!"),
    })
}

#[get("/detail/<year>/<month>")]
pub async fn get_detail(year: i32, month: i32, db: &State<DbClient>) -> Json<Vec<Detail>> {
    let get_detail =
        db.0.query(read_query("Detail/Get_Detail").as_str(), &[&month, &year])
            .await
            .unwrap();

    let details: Vec<Detail> = get_detail
        .iter()
        .map(|row| Detail {
            fecha: row.get("Fecha"),
            month: row.get("Month"),
            year: row.get("Year"),
            detalle: row.get("Detalle"),
            nombre_presupuesto: row.get("Concepto"),
            monto_usd: row.get("MontoUSD"),
            monto_bs: row.get("MontoBS"),
            promedio: row.get("Promedio"),
        })
        .collect();

    Json(details)
}

// Dashboard
#[get("/dashboard/<year>/<month>")]
pub async fn get_data_dashboard(year: i32, month: i32, db: &State<DbClient>) -> Json<Dashboard> {
    let ingresos_consolidado =
        db.0.query(
            read_query("Dashboard/Cons_Ingresos").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();
    let egresos_consolidado =
        db.0.query(
            read_query("Dashboard/Cons_Egresos").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();
    let ingresos_consolidado_pre =
        db.0.query(
            read_query("Dashboard/Cons_Ingresos_Pre").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();
    let egresos_consolidado_pre =
        db.0.query(
            read_query("Dashboard/Cons_Egresos_Pre").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();
    let all_ingresos =
        db.0.query(
            read_query("Dashboard/All_Ingresos").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();
    let all_egresos =
        db.0.query(
            read_query("Dashboard/All_Egresos").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();

    let all_ingresos_pre =
        db.0.query(
            read_query("Dashboard/All_Ingresos_Pre").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();
    let all_egresos_pre =
        db.0.query(
            read_query("Dashboard/All_Egresos_Pre").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();
    let ingresos_vs_egresos =
        db.0.query(
            read_query("Dashboard/Ingresos_vs_Egresos").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();

    // mapping

    let total_ingresos: Vec<f64> = ingresos_consolidado
        .iter()
        .map(|row| row.get("TotalIngresos"))
        .collect();

    let total_egresos: Vec<f64> = egresos_consolidado
        .iter()
        .map(|row| row.get("TotalEgresos"))
        .collect();

    let total_ingresos_pre: Vec<f64> = ingresos_consolidado_pre
        .iter()
        .map(|row| row.get("TotalIngresosPre"))
        .collect();

    let total_egreso_pre: Vec<f64> = egresos_consolidado_pre
        .iter()
        .map(|row| row.get("TotalEgresosPre"))
        .collect();

    let listado_ingresos: Vec<Resumen> = all_ingresos
        .iter()
        .map(|row| Resumen {
            categoria: row.get("Categoria"),
            tipo: row.get("Tipo"),
            monto_usd: row.get("MontoUSD"),
        })
        .collect();

    let listado_egresos: Vec<Resumen> = all_egresos
        .iter()
        .map(|row| Resumen {
            categoria: row.get("Categoria"),
            tipo: row.get("Tipo"),
            monto_usd: row.get("MontoUSD"),
        })
        .collect();

    let listado_ingresos_pre: Vec<Resumen> = all_ingresos_pre
        .iter()
        .map(|row| Resumen {
            categoria: row.get("Categoria"),
            tipo: row.get("Tipo"),
            monto_usd: row.get("MontoUSD"),
        })
        .collect();

    let listado_egresos_pre: Vec<Resumen> = all_egresos_pre
        .iter()
        .map(|row| Resumen {
            categoria: row.get("Categoria"),
            tipo: row.get("Tipo"),
            monto_usd: row.get("MontoUSD"),
        })
        .collect();

    let listado_ingvsegrs: Vec<Resumen> = ingresos_vs_egresos
        .iter()
        .map(|row| Resumen {
            categoria: row.get("Categoria"),
            tipo: row.get("Tipo"),
            monto_usd: row.get("MontoUSD"),
        })
        .collect();

    Json(Dashboard {
        total_ingresos: *total_ingresos.last().unwrap(),
        total_ingresos_pre: *total_ingresos_pre.last().unwrap(),
        total_egresos: *total_egresos.last().unwrap(),
        total_egresos_pre: *total_egreso_pre.last().unwrap(),
        ingresos: listado_ingresos,
        egresos: listado_egresos,
        ingresos_pre: listado_ingresos_pre,
        egresos_pre: listado_egresos_pre,
        ing_vs_egr: listado_ingvsegrs,
    })
}

// Form Budget
#[get("/budget/form")]
pub async fn get_data_form_budget(db: &State<DbClient>) -> Json<FormDataBudget> {
    let get_tipos =
        db.0.query(read_query("Util/Get_Tipos").as_str(), &[])
            .await
            .unwrap();

    let tipos = get_tipos
        .iter()
        .map(|row| Tipo {
            id: row.get("ID"),
            tipo: row.get("Tipo"),
        })
        .collect();

    let get_categorias =
        db.0.query(read_query("Util/Get_Categorias").as_str(), &[])
            .await
            .unwrap();

    let categorias = get_categorias
        .iter()
        .map(|row| Categoria {
            id: row.get("ID"),
            categoria: row.get("Categoria"),
            tipo: row.get("Tipo"),
        })
        .collect();

    Json(FormDataBudget {
        categorias: categorias,
        tipos: tipos,
    })
}

// Form Detail
#[get("/detail/form/<year>/<month>")]
pub async fn get_data_form_detail(
    year: i32,
    month: i32,
    db: &State<DbClient>,
) -> Json<Vec<BudgetSendToForm>> {
    let get_budgets =
        db.0.query(
            read_query("Util/Get_Presupuestos").as_str(),
            &[&month, &year],
        )
        .await
        .unwrap();

    let budgets = get_budgets
        .iter()
        .map(|row| BudgetSendToForm {
            id: row.get("ID"),
            tipo: row.get("Tipo"),
            categoria: row.get("Categoria"),
            concepto: row.get("Concepto"),
        })
        .collect();

    Json(budgets)
}

async fn get_current_tasa(month: i32, year: i32, db: &State<DbClient>) -> (i32, f64) {
    let find_tasa =
        db.0.query(read_query("Util/Get_Last_Tasa").as_str(), &[&month, &year])
            .await
            .unwrap();

    let result: Vec<(i32, f64)> = find_tasa
        .iter()
        .map(|row| (row.get("ID"), row.get("Promedio")))
        .collect();

    result[0]
}
