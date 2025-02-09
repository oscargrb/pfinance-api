select sum("MontoUSD") as "TotalEgresos"
from (
		SELECT public."Presupuesto"."Month",
			public."FlujoIngreso"."Tipo",
			public."Detalle"."MontoUSD"
		FROM public."Detalle"
			inner join public."Presupuesto" on public."Presupuesto"."ID" = public."Detalle"."Presupuesto"
			inner join public."FlujoIngreso" on public."FlujoIngreso"."ID" = public."Presupuesto"."Tipo"
		where public."Presupuesto"."Month" = $1
			and public."Presupuesto"."Year" = $2
			and public."FlujoIngreso"."Tipo" = 'Egreso'
	)