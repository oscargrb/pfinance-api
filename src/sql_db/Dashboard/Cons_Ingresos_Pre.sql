select sum("PresupuestoUSD") as "TotalIngresosPre"
from (
		SELECT public."Presupuesto"."Month",
			public."FlujoIngreso"."Tipo",
			public."Presupuesto"."PresupuestoUSD"
		FROM public."Presupuesto"
			inner join public."FlujoIngreso" on public."FlujoIngreso"."ID" = public."Presupuesto"."Tipo"
		where public."Presupuesto"."Month" = $1
			and public."Presupuesto"."Year" = $2
			and public."FlujoIngreso"."Tipo" = 'Ingreso'
	)