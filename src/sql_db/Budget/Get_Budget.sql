SELECT 
    public."Presupuesto"."ID", 
    public."Presupuesto"."Month", 
    public."Presupuesto"."Year", 
    public."FlujoIngreso"."Tipo", 
    public."Categoria"."Categoria", 
    public."Presupuesto"."Concepto", 
    public."Presupuesto"."PresupuestoUSD", 
    public."Presupuesto"."PresupuestoBS", 
    public."Tasa"."Promedio"

FROM public."Presupuesto"

INNER JOIN public."FlujoIngreso" ON public."Presupuesto"."Tipo" = public."FlujoIngreso"."ID"
INNER JOIN public."Categoria" ON public."Presupuesto"."Categoria" = public."Categoria"."ID"
INNER JOIN public."Tasa" ON public."Presupuesto"."Tasa" = public."Tasa"."ID"

WHERE public."Presupuesto"."Month" = $1 AND public."Presupuesto"."Year" = $2;