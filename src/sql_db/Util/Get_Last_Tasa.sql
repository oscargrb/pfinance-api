SELECT "ID", "Promedio"
FROM "Tasa"
WHERE "Month" = $1 and "Year" = $2
ORDER BY "ID" DESC
LIMIT 1;