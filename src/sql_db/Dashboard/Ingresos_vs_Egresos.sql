SELECT public."Categoria"."Categoria" AS Categoria,
    public."FlujoIngreso"."Tipo",
    SUM(
        CASE
            WHEN public."FlujoIngreso"."Tipo" = 'Ingreso' THEN public."Detalle"."MontoUSD"
            ELSE - public."Detalle"."MontoUSD"
        END
    ) AS MontoUSD,
    SUM(
        CASE
            WHEN public."FlujoIngreso"."Tipo" = 'Ingreso' THEN public."Presupuesto"."PresupuestoUSD"
            ELSE - public."Presupuesto"."PresupuestoUSD"
        END
    ) AS PresupuestoUSD
FROM public."Detalle"
    INNER JOIN public."Presupuesto" ON public."Presupuesto"."ID" = public."Detalle"."Presupuesto"
    INNER JOIN public."Categoria" ON public."Categoria"."ID" = public."Presupuesto"."Categoria"
    INNER JOIN public."FlujoIngreso" ON public."FlujoIngreso"."ID" = public."Presupuesto"."Tipo"
WHERE public."Presupuesto"."Month" = $1
    and public."Presupuesto"."Year" = $2
GROUP BY public."Categoria"."Categoria",
    public."FlujoIngreso"."Tipo"