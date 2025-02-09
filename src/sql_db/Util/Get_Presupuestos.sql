SELECT 
    "Presupuesto"."ID", 
    "FlujoIngreso"."Tipo", 
    "Categoria"."Categoria", 
    "Presupuesto"."Concepto"
FROM "Presupuesto"
INNER JOIN "FlujoIngreso" on "FlujoIngreso"."ID" = "Presupuesto"."Tipo"
INNER JOIN "Categoria" on "Categoria"."ID" = "Presupuesto"."Categoria"
WHERE "Month" = $1 and "Year" = $2
