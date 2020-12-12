CREATE TABLE derivations (
    id SERIAL PRIMARY KEY,
    code character varying(255) NOT NULL,
    description text NOT NULL
);

CREATE TABLE food_groups (
    id SERIAL PRIMARY KEY,
    description character varying(255) DEFAULT ''::character varying NOT NULL
);

CREATE TABLE foods (
    id SERIAL PRIMARY KEY,
    publication_date timestamp with time zone NOT NULL,
    modified_date timestamp with time zone NOT NULL,
    available_date timestamp with time zone NOT NULL,
    upc character varying(24) NOT NULL,
    fdc_id character varying(24) NOT NULL,
    description character varying(255) NOT NULL,
    food_group_id int DEFAULT '0' NOT NULL,
    manufacturer_id int DEFAULT '0' NOT NULL,
    datasource character varying(8) NOT NULL,
    serving_size double precision,
    serving_unit character varying(24) DEFAULT NULL::character varying,
    serving_description character varying(256) DEFAULT NULL::character varying,
    country character varying(24) DEFAULT NULL::character varying,
    ingredients text
);

CREATE TABLE manufacturers (
    id SERIAL PRIMARY KEY,
    name character varying(255) DEFAULT ''::character varying NOT NULL
);


CREATE TABLE nutrient_data (
    id SERIAL PRIMARY KEY,
    value double precision DEFAULT '0'::double precision NOT NULL,
    standard_error double precision,
    minimum double precision,
    maximum double precision,
    median double precision,
    derivation_id int DEFAULT '0' NOT NULL,
    nutrient_id int DEFAULT '0' NOT NULL,
    food_id int
);



CREATE TABLE nutrients (
    id int PRIMARY KEY,
    nutrientno character varying(12) NOT NULL,
    description character varying(255) NOT NULL,
    unit character varying(24) NOT NULL
);


CREATE INDEX idx_16458_foods_description_idx ON foods USING btree (description);


CREATE INDEX idx_16458_foods_fdc_id_idx ON foods USING btree (fdc_id);


CREATE INDEX idx_16458_foods_fk ON foods USING btree (manufacturer_id);



CREATE INDEX idx_16458_foods_food_group_id_idx ON foods USING btree (food_group_id);



CREATE INDEX idx_16458_foods_manufacturer_id_idx ON foods USING btree (manufacturer_id);



CREATE INDEX idx_16458_foods_upc_idx ON foods USING btree (upc);



CREATE INDEX idx_16472_food_groups_description_idx ON food_groups USING btree (description);



CREATE INDEX idx_16479_manufacturers_name_idx ON manufacturers USING btree (name);



CREATE UNIQUE INDEX idx_16484_nutrientno ON nutrients USING btree (nutrientno);



CREATE INDEX idx_16489_nutrient_data_fk ON nutrient_data USING btree (nutrient_id);



CREATE INDEX idx_16489_nutrient_data_fk_1 ON nutrient_data USING btree (derivation_id);



CREATE INDEX idx_16489_nutrient_data_food_id_idx ON nutrient_data USING btree (food_id);



ALTER TABLE ONLY foods
    ADD CONSTRAINT foods_fk FOREIGN KEY (manufacturer_id) REFERENCES manufacturers(id) ON UPDATE RESTRICT ON DELETE RESTRICT;



ALTER TABLE ONLY foods
    ADD CONSTRAINT foods_fk_1 FOREIGN KEY (food_group_id) REFERENCES food_groups(id) ON UPDATE RESTRICT ON DELETE RESTRICT;



ALTER TABLE ONLY nutrient_data
    ADD CONSTRAINT nutrient_data_fk FOREIGN KEY (nutrient_id) REFERENCES nutrients(id) ON UPDATE RESTRICT ON DELETE RESTRICT;



ALTER TABLE ONLY nutrient_data
    ADD CONSTRAINT nutrient_data_fk_1 FOREIGN KEY (derivation_id) REFERENCES derivations(id) ON UPDATE RESTRICT ON DELETE RESTRICT;



ALTER TABLE ONLY nutrient_data
    ADD CONSTRAINT nutrient_data_food_fk FOREIGN KEY (food_id) REFERENCES foods(id) ON UPDATE RESTRICT ON DELETE RESTRICT;



