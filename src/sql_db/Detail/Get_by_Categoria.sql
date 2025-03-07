SELECT SUM(public."Detalle"."MontoUSD"),
    SUM(public."Detalle"."MontoBS"),
    public."Presupuesto"."Concepto"
FROM public."Detalle"
    INNER JOIN public."Presupuesto" ON public."Presupuesto"."ID" = public."Detalle"."Presupuesto"
    INNER JOIN public."Categoria" ON public."Categoria"."ID" = public."Presupuesto"."Categoria"
    INNER JOIN public."FlujoIngreso" ON public."FlujoIngreso"."ID" = public."Presupuesto"."Tipo"
WHERE public."Presupuesto"."Month" = $1
    and public."Presupuesto"."Year" = $2
    and public."Categoria"."Categoria" = 'Servicios'
GROUP BY public."Presupuesto"."Concepto"