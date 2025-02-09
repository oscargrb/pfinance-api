SELECT 
    public."Presupuesto"."Concepto",
    public."Presupuesto"."Month",
    public."Presupuesto"."Year",
    public."Detalle"."Detalle",
    public."Detalle"."MontoUSD",
    public."Detalle"."MontoBS",
    public."Tasa"."Promedio",
    public."Detalle"."Fecha"

FROM public."Detalle"
INNER JOIN public."Presupuesto" ON public."Presupuesto"."ID" = public."Detalle"."Presupuesto"
INNER JOIN public."Tasa" ON public."Tasa"."ID" = public."Detalle"."Tasa" 

where public."Presupuesto"."Month" = $1 and public."Presupuesto"."Year" = $2
ORDER BY public."Detalle"."ID" DESC;