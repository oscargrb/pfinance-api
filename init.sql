CREATE SEQUENCE public."FlujoIngreso_ID_seq" START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;
CREATE SEQUENCE public."Categoria_ID_seq" START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;
CREATE SEQUENCE public."Tasa_ID_seq" START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;
CREATE SEQUENCE public."Presupuesto_ID_seq" START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;
CREATE SEQUENCE public."Detalle_ID_seq" START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1;
-- Table: public.FlujoIngreso
-- DROP TABLE IF EXISTS public."FlujoIngreso";
CREATE TABLE IF NOT EXISTS public."FlujoIngreso" (
    "ID" integer NOT NULL DEFAULT nextval('"FlujoIngreso_ID_seq"'::regclass),
    "Tipo" character varying(255) COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT "FlujoIngreso_pkey" PRIMARY KEY ("ID")
) TABLESPACE pg_default;
ALTER TABLE IF EXISTS public."FlujoIngreso" OWNER to postgres;
-- Table: public.Categoria
-- DROP TABLE IF EXISTS public."Categoria";
CREATE TABLE IF NOT EXISTS public."Categoria" (
    "ID" integer NOT NULL DEFAULT nextval('"Categoria_ID_seq"'::regclass),
    "Categoria" character varying(255) COLLATE pg_catalog."default" NOT NULL,
    "Tipo" integer,
    CONSTRAINT "Categoria_pkey" PRIMARY KEY ("ID"),
    CONSTRAINT "Tipo" FOREIGN KEY ("Tipo") REFERENCES public."FlujoIngreso" ("ID") MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION NOT VALID
) TABLESPACE pg_default;
ALTER TABLE IF EXISTS public."Categoria" OWNER to postgres;
-- Table: public.Tasa
-- DROP TABLE IF EXISTS public."Tasa";
CREATE TABLE IF NOT EXISTS public."Tasa" (
    "ID" integer NOT NULL DEFAULT nextval('"Tasa_ID_seq"'::regclass),
    "Fecha" character varying(255) COLLATE pg_catalog."default" NOT NULL,
    "Month" integer,
    "Year" integer,
    "TasaBCV" double precision,
    "TasaParalela" double precision,
    "Promedio" double precision,
    CONSTRAINT "Tasa_pkey" PRIMARY KEY ("ID")
) TABLESPACE pg_default;
ALTER TABLE IF EXISTS public."Tasa" OWNER to postgres;
-- Table: public.Presupuesto
-- DROP TABLE IF EXISTS public."Presupuesto";
CREATE TABLE IF NOT EXISTS public."Presupuesto" (
    "ID" integer NOT NULL DEFAULT nextval('"Presupuesto_ID_seq"'::regclass),
    "Month" integer,
    "Year" integer,
    "Tipo" integer,
    "Categoria" integer,
    "Concepto" character varying(255) COLLATE pg_catalog."default",
    "PresupuestoUSD" double precision,
    "PresupuestoBS" double precision,
    "Tasa" integer,
    CONSTRAINT "Presupuesto_pkey" PRIMARY KEY ("ID"),
    CONSTRAINT "Categoria" FOREIGN KEY ("Categoria") REFERENCES public."Categoria" ("ID") MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT "Tasa" FOREIGN KEY ("Tasa") REFERENCES public."Tasa" ("ID") MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION NOT VALID,
    CONSTRAINT "Tipo" FOREIGN KEY ("Tipo") REFERENCES public."FlujoIngreso" ("ID") MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION
) TABLESPACE pg_default;
ALTER TABLE IF EXISTS public."Presupuesto" OWNER to postgres;
-- Table: public.Detalle
-- DROP TABLE IF EXISTS public."Detalle";
CREATE TABLE IF NOT EXISTS public."Detalle" (
    "ID" integer NOT NULL DEFAULT nextval('"Detalle_ID_seq"'::regclass),
    "Presupuesto" integer,
    "Detalle" character varying(255) COLLATE pg_catalog."default",
    "MontoUSD" double precision,
    "MontoBS" double precision,
    "Tasa" integer,
    "Fecha" character varying(255) COLLATE pg_catalog."default",
    CONSTRAINT "Detalle_pkey" PRIMARY KEY ("ID"),
    CONSTRAINT "Presupuesto" FOREIGN KEY ("Presupuesto") REFERENCES public."Presupuesto" ("ID") MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION NOT VALID,
    CONSTRAINT "Tasa" FOREIGN KEY ("Tasa") REFERENCES public."Tasa" ("ID") MATCH SIMPLE ON UPDATE NO ACTION ON DELETE NO ACTION NOT VALID
) TABLESPACE pg_default;
ALTER TABLE IF EXISTS public."Detalle" OWNER to postgres;