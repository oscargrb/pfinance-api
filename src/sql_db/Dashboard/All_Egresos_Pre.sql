SELECT public."Categoria"."Categoria" AS Categoria,
    public."FlujoIngreso"."Tipo",
    SUM(public."Presupuesto"."PresupuestoUSD") AS MontoUSD
FROM public."Presupuesto"
    INNER JOIN public."Categoria" ON public."Categoria"."ID" = public."Presupuesto"."Categoria"
    INNER JOIN public."FlujoIngreso" ON public."FlujoIngreso"."ID" = public."Presupuesto"."Tipo"
WHERE public."Presupuesto"."Month" = $1
    and public."Presupuesto"."Year" = $2
    and public."FlujoIngreso"."Tipo" = 'Egreso'
GROUP BY public."Categoria"."Categoria",
    public."FlujoIngreso"."Tipo"