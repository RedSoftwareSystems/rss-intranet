#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "pgactix" --password "pgactix" <<-EOSQL
--
-- PostgreSQL database dump
--

-- Dumped from database version 10.4 (Debian 10.4-2.pgdg90+1)
-- Dumped by pg_dump version 10.4 (Debian 10.4-2.pgdg90+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: users; Type: TABLE; Schema: public; Owner: pgactix
--

CREATE TABLE public.users (
    id uuid NOT NULL,
    data jsonb NOT NULL
);


ALTER TABLE public.users OWNER TO pgactix;

--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: pgactix
--

COPY public.users (id, data) FROM stdin;
8294293f-4d8c-5306-80fd-09fab26b6a38	{"email": "user1@test.com", "last_name": "Chiumenti", "user_name": "user1", "first_name": "Andrea"}
a4923514-8cc7-11e8-bfe6-0242ac110003	{"email": "user2@test.com", "last_name": "Chiumenti", "user_name": "user2", "first_name": "Andrea"}
e1ee1ad4-cd4e-5889-962a-4f605a68d94e	{"email": "user3@test.com", "last_name": "Chiumenti", "user_name": "user3", "first_name": "Andrea"}
\.


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: pgactix
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: user_data_ix; Type: INDEX; Schema: public; Owner: pgactix
--

CREATE INDEX user_data_ix ON public.users USING gin (data);


--
-- Name: user_email_ix; Type: INDEX; Schema: public; Owner: pgactix
--

CREATE UNIQUE INDEX user_email_ix ON public.users USING btree (((data -> 'email'::text)));


--
-- Name: user_id_ix; Type: INDEX; Schema: public; Owner: pgactix
--

CREATE UNIQUE INDEX user_id_ix ON public.users USING btree (id);


--
-- Name: user_username_ix; Type: INDEX; Schema: public; Owner: pgactix
--

CREATE UNIQUE INDEX user_username_ix ON public.users USING btree (((data -> 'user_name'::text)));


--
-- PostgreSQL database dump complete
--

EOSQL
