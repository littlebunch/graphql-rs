--
-- PostgreSQL database dump
--

-- Dumped from database version 13.0
-- Dumped by pg_dump version 13.0

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: bfpd_test_two; Type: SCHEMA; Schema: -; Owner: gmoore
--

CREATE SCHEMA bfpd_test_two;


ALTER SCHEMA bfpd_test_two OWNER TO gmoore;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: __diesel_schema_migrations; Type: TABLE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE TABLE bfpd_test_two.__diesel_schema_migrations (
    version character varying(50) NOT NULL,
    run_on timestamp with time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE bfpd_test_two.__diesel_schema_migrations OWNER TO gmoore;

--
-- Name: derivations; Type: TABLE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE TABLE bfpd_test_two.derivations (
    id bigint NOT NULL,
    code character varying(255) NOT NULL,
    description text NOT NULL
);


ALTER TABLE bfpd_test_two.derivations OWNER TO gmoore;

--
-- Name: food_groups; Type: TABLE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE TABLE bfpd_test_two.food_groups (
    id bigint NOT NULL,
    description character varying(255) DEFAULT ''::character varying NOT NULL
);


ALTER TABLE bfpd_test_two.food_groups OWNER TO gmoore;

--
-- Name: food_groups_id_seq; Type: SEQUENCE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE SEQUENCE bfpd_test_two.food_groups_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bfpd_test_two.food_groups_id_seq OWNER TO gmoore;

--
-- Name: food_groups_id_seq; Type: SEQUENCE OWNED BY; Schema: bfpd_test_two; Owner: gmoore
--

ALTER SEQUENCE bfpd_test_two.food_groups_id_seq OWNED BY bfpd_test_two.food_groups.id;


--
-- Name: foods; Type: TABLE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE TABLE bfpd_test_two.foods (
    id bigint NOT NULL,
    publication_date timestamp with time zone NOT NULL,
    modified_date timestamp with time zone NOT NULL,
    available_date timestamp with time zone NOT NULL,
    upc character varying(24) NOT NULL,
    fdc_id character varying(24) NOT NULL,
    description character varying(255) NOT NULL,
    food_group_id bigint DEFAULT '0'::bigint NOT NULL,
    manufacturer_id bigint DEFAULT '0'::bigint NOT NULL,
    datasource character varying(8) NOT NULL,
    serving_size double precision,
    serving_unit character varying(24) DEFAULT NULL::character varying,
    serving_description character varying(256) DEFAULT NULL::character varying,
    country character varying(24) DEFAULT NULL::character varying,
    ingredients text
);


ALTER TABLE bfpd_test_two.foods OWNER TO gmoore;

--
-- Name: foods_id_seq; Type: SEQUENCE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE SEQUENCE bfpd_test_two.foods_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bfpd_test_two.foods_id_seq OWNER TO gmoore;

--
-- Name: foods_id_seq; Type: SEQUENCE OWNED BY; Schema: bfpd_test_two; Owner: gmoore
--

ALTER SEQUENCE bfpd_test_two.foods_id_seq OWNED BY bfpd_test_two.foods.id;


--
-- Name: manufacturers; Type: TABLE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE TABLE bfpd_test_two.manufacturers (
    id bigint NOT NULL,
    name character varying(255) DEFAULT ''::character varying NOT NULL
);


ALTER TABLE bfpd_test_two.manufacturers OWNER TO gmoore;

--
-- Name: manufacturers_id_seq; Type: SEQUENCE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE SEQUENCE bfpd_test_two.manufacturers_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bfpd_test_two.manufacturers_id_seq OWNER TO gmoore;

--
-- Name: manufacturers_id_seq; Type: SEQUENCE OWNED BY; Schema: bfpd_test_two; Owner: gmoore
--

ALTER SEQUENCE bfpd_test_two.manufacturers_id_seq OWNED BY bfpd_test_two.manufacturers.id;


--
-- Name: nutrient_data; Type: TABLE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE TABLE bfpd_test_two.nutrient_data (
    id bigint NOT NULL,
    value double precision DEFAULT '0'::double precision NOT NULL,
    standard_error double precision,
    minimum double precision,
    maximum double precision,
    median double precision,
    derivation_id bigint DEFAULT '0'::bigint NOT NULL,
    nutrient_id bigint DEFAULT '0'::bigint NOT NULL,
    food_id bigint
);


ALTER TABLE bfpd_test_two.nutrient_data OWNER TO gmoore;

--
-- Name: nutrient_data_id_seq; Type: SEQUENCE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE SEQUENCE bfpd_test_two.nutrient_data_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE bfpd_test_two.nutrient_data_id_seq OWNER TO gmoore;

--
-- Name: nutrient_data_id_seq; Type: SEQUENCE OWNED BY; Schema: bfpd_test_two; Owner: gmoore
--

ALTER SEQUENCE bfpd_test_two.nutrient_data_id_seq OWNED BY bfpd_test_two.nutrient_data.id;


--
-- Name: nutrients; Type: TABLE; Schema: bfpd_test_two; Owner: gmoore
--

CREATE TABLE bfpd_test_two.nutrients (
    id bigint NOT NULL,
    nutrientno character varying(12) NOT NULL,
    description character varying(255) NOT NULL,
    unit character varying(24) NOT NULL
);


ALTER TABLE bfpd_test_two.nutrients OWNER TO gmoore;

--
-- Name: food_groups id; Type: DEFAULT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.food_groups ALTER COLUMN id SET DEFAULT nextval('bfpd_test_two.food_groups_id_seq'::regclass);


--
-- Name: foods id; Type: DEFAULT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.foods ALTER COLUMN id SET DEFAULT nextval('bfpd_test_two.foods_id_seq'::regclass);


--
-- Name: manufacturers id; Type: DEFAULT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.manufacturers ALTER COLUMN id SET DEFAULT nextval('bfpd_test_two.manufacturers_id_seq'::regclass);


--
-- Name: nutrient_data id; Type: DEFAULT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.nutrient_data ALTER COLUMN id SET DEFAULT nextval('bfpd_test_two.nutrient_data_id_seq'::regclass);


--
-- Name: derivations idx_16450_primary; Type: CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.derivations
    ADD CONSTRAINT idx_16450_primary PRIMARY KEY (id);


--
-- Name: foods idx_16458_primary; Type: CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.foods
    ADD CONSTRAINT idx_16458_primary PRIMARY KEY (id);


--
-- Name: food_groups idx_16472_primary; Type: CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.food_groups
    ADD CONSTRAINT idx_16472_primary PRIMARY KEY (id);


--
-- Name: manufacturers idx_16479_primary; Type: CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.manufacturers
    ADD CONSTRAINT idx_16479_primary PRIMARY KEY (id);


--
-- Name: nutrients idx_16484_primary; Type: CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.nutrients
    ADD CONSTRAINT idx_16484_primary PRIMARY KEY (id);


--
-- Name: nutrient_data idx_16489_primary; Type: CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.nutrient_data
    ADD CONSTRAINT idx_16489_primary PRIMARY KEY (id);


--
-- Name: __diesel_schema_migrations idx_16496_primary; Type: CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.__diesel_schema_migrations
    ADD CONSTRAINT idx_16496_primary PRIMARY KEY (version);


--
-- Name: idx_16458_foods_description_idx; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16458_foods_description_idx ON bfpd_test_two.foods USING btree (description);


--
-- Name: idx_16458_foods_fdc_id_idx; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16458_foods_fdc_id_idx ON bfpd_test_two.foods USING btree (fdc_id);


--
-- Name: idx_16458_foods_fk; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16458_foods_fk ON bfpd_test_two.foods USING btree (manufacturer_id);


--
-- Name: idx_16458_foods_food_group_id_idx; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16458_foods_food_group_id_idx ON bfpd_test_two.foods USING btree (food_group_id);


--
-- Name: idx_16458_foods_manufacturer_id_idx; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16458_foods_manufacturer_id_idx ON bfpd_test_two.foods USING btree (manufacturer_id);


--
-- Name: idx_16458_foods_upc_idx; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16458_foods_upc_idx ON bfpd_test_two.foods USING btree (upc);


--
-- Name: idx_16472_food_groups_description_idx; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16472_food_groups_description_idx ON bfpd_test_two.food_groups USING btree (description);


--
-- Name: idx_16479_manufacturers_name_idx; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16479_manufacturers_name_idx ON bfpd_test_two.manufacturers USING btree (name);


--
-- Name: idx_16484_nutrientno; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE UNIQUE INDEX idx_16484_nutrientno ON bfpd_test_two.nutrients USING btree (nutrientno);


--
-- Name: idx_16489_nutrient_data_fk; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16489_nutrient_data_fk ON bfpd_test_two.nutrient_data USING btree (nutrient_id);


--
-- Name: idx_16489_nutrient_data_fk_1; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16489_nutrient_data_fk_1 ON bfpd_test_two.nutrient_data USING btree (derivation_id);


--
-- Name: idx_16489_nutrient_data_food_id_idx; Type: INDEX; Schema: bfpd_test_two; Owner: gmoore
--

CREATE INDEX idx_16489_nutrient_data_food_id_idx ON bfpd_test_two.nutrient_data USING btree (food_id);


--
-- Name: foods foods_fk; Type: FK CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.foods
    ADD CONSTRAINT foods_fk FOREIGN KEY (manufacturer_id) REFERENCES bfpd_test_two.manufacturers(id) ON UPDATE RESTRICT ON DELETE RESTRICT;


--
-- Name: foods foods_fk_1; Type: FK CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.foods
    ADD CONSTRAINT foods_fk_1 FOREIGN KEY (food_group_id) REFERENCES bfpd_test_two.food_groups(id) ON UPDATE RESTRICT ON DELETE RESTRICT;


--
-- Name: nutrient_data nutrient_data_fk; Type: FK CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.nutrient_data
    ADD CONSTRAINT nutrient_data_fk FOREIGN KEY (nutrient_id) REFERENCES bfpd_test_two.nutrients(id) ON UPDATE RESTRICT ON DELETE RESTRICT;


--
-- Name: nutrient_data nutrient_data_fk_1; Type: FK CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.nutrient_data
    ADD CONSTRAINT nutrient_data_fk_1 FOREIGN KEY (derivation_id) REFERENCES bfpd_test_two.derivations(id) ON UPDATE RESTRICT ON DELETE RESTRICT;


--
-- Name: nutrient_data nutrient_data_food_fk; Type: FK CONSTRAINT; Schema: bfpd_test_two; Owner: gmoore
--

ALTER TABLE ONLY bfpd_test_two.nutrient_data
    ADD CONSTRAINT nutrient_data_food_fk FOREIGN KEY (food_id) REFERENCES bfpd_test_two.foods(id) ON UPDATE RESTRICT ON DELETE RESTRICT;


--
-- PostgreSQL database dump complete
--

