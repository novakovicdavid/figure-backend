--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

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
-- Name: public; Type: SCHEMA; Schema: -; Owner: figure
--

-- *not* creating schema, since initdb creates it


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: figures; Type: TABLE; Schema: public; Owner: figure
--

CREATE TABLE public.figures (
    id bigint NOT NULL,
    title text NOT NULL,
    width integer NOT NULL,
    height integer NOT NULL,
    profile_id bigint NOT NULL,
    url text NOT NULL,
    description text
);

--
-- Name: figure_id_seq; Type: SEQUENCE; Schema: public; Owner: figure
--

CREATE SEQUENCE public.figure_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: figure_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: figure
--

ALTER SEQUENCE public.figure_id_seq OWNED BY public.figures.id;

--
-- Name: profiles; Type: TABLE; Schema: public; Owner: figure
--

CREATE TABLE public.profiles (
    id bigint NOT NULL,
    username text NOT NULL,
    display_name text,
    user_id bigint,
    profile_picture text,
    bio text,
    banner text
);

--
-- Name: profile_id_seq; Type: SEQUENCE; Schema: public; Owner: figure
--

CREATE SEQUENCE public.profile_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: profile_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: figure
--

ALTER SEQUENCE public.profile_id_seq OWNED BY public.profiles.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: figure
--

CREATE TABLE public.users (
    id bigint NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    role text NOT NULL,
    CONSTRAINT email_check CHECK ((email = lower(email)))
);

--
-- Name: user_id_seq; Type: SEQUENCE; Schema: public; Owner: figure
--

CREATE SEQUENCE public.user_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: user_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: figure
--

ALTER SEQUENCE public.user_id_seq OWNED BY public.users.id;


--
-- Name: figures id; Type: DEFAULT; Schema: public; Owner: figure
--

ALTER TABLE ONLY public.figures ALTER COLUMN id SET DEFAULT nextval('public.figure_id_seq'::regclass);


--
-- Name: profiles id; Type: DEFAULT; Schema: public; Owner: figure
--

ALTER TABLE ONLY public.profiles ALTER COLUMN id SET DEFAULT nextval('public.profile_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: figure
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.user_id_seq'::regclass);


--
-- Data for Name: figures; Type: TABLE DATA; Schema: public; Owner: figure
--

INSERT INTO public.figures (id, title, width, height, profile_id, url, description) VALUES (1, 'My other cat', 4128, 3096, 1, 'https://cdn.figure.novakovic.be/35357ff7-f1c0-4264-9c2a-98119ac6eaed', 'o.o');
INSERT INTO public.figures (id, title, width, height, profile_id, url, description) VALUES (2, 'naps', 1125, 1160, 1, 'https://cdn.figure.novakovic.be/c5dc0e67-2928-4693-89b4-9afd7566e8ac', 'yes');
INSERT INTO public.figures (id, title, width, height, profile_id, url, description) VALUES (3, 'leonardo', 530, 530, 2, 'https://cdn.figure.novakovic.be/e6906ab2-50b4-4a31-9dcb-96e90dcd5b41', 'dicaprio');
INSERT INTO public.figures (id, title, width, height, profile_id, url, description) VALUES (4, 'Lazy cat', 1080, 1067, 1, 'https://cdn.figure.novakovic.be/17af1c06-df69-493a-aede-c7660c4d911c', 'he''s done');
INSERT INTO public.figures (id, title, width, height, profile_id, url, description) VALUES (5, 'LMTS', 605, 605, 1, 'https://cdn.figure.novakovic.be/3ecd0561-c350-43c8-99d6-7fea4dd864b8', 'Limits Academy');
INSERT INTO public.figures (id, title, width, height, profile_id, url, description) VALUES (6, 'Kestrel', 900, 600, 1, 'https://cdn.figure.novakovic.be/fdd38177-030e-4fc8-9e46-d58886557794', 'FTL: Faster Than Light');
INSERT INTO public.figures (id, title, width, height, profile_id, url, description) VALUES (7, 'The default profile picture', 320, 320, 1, 'https://cdn.figure.novakovic.be/fbd19ed7-6f77-49e1-b54d-c913a75df62e', 'Given to every new user.');


--
-- Data for Name: profiles; Type: TABLE DATA; Schema: public; Owner: figure
--

INSERT INTO public.profiles (id, username, display_name, user_id, profile_picture, bio, banner) VALUES (4, 'four', 'Mr. Four', 4, 'https://cdn.figure.novakovic.be/profile_pictures/d6ce3718-6092-4d61-9a94-11399b915027', 'I can count to four.', 'https://cdn.figure.novakovic.be/banners/54cda2a9-aada-4d62-a2ce-cf86e3362304');
INSERT INTO public.profiles (id, username, display_name, user_id, profile_picture, bio, banner) VALUES (1, 'one', 'hi', 1, 'https://cdn.figure.novakovic.be/profile_pictures/c52b7e30-e04d-4730-bcca-263caf8e162d', 'First user on this website. Test test test!!!', 'https://cdn.figure.novakovic.be/banners/2f0a720a-6bc6-468d-b7f2-eb8642a7cf84');
INSERT INTO public.profiles (id, username, display_name, user_id, profile_picture, bio, banner) VALUES (2, 'two', NULL, 2, NULL, NULL, NULL);
INSERT INTO public.profiles (id, username, display_name, user_id, profile_picture, bio, banner) VALUES (3, 'three', NULL, 3, NULL, NULL, NULL);


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: figure
--

INSERT INTO public.users (id, email, password, role) VALUES (1, 'one@one.one', '$argon2id$v=19$m=8192,t=5,p=1$Nup7xybNf9TvQSNvHAj1ng$MYlpQForpsxSM2p8Lskcnww0gX8MKuDAjLEyedMA//8', 'user');
INSERT INTO public.users (id, email, password, role) VALUES (2, 'two@two.two', '$argon2id$v=19$m=8192,t=5,p=1$lNbswSxj/aLz6JDvSJIdOw$nCA3vNkWQrHdDODLD51M4V5uyeWObMtCrMP92wgHves', 'user');
INSERT INTO public.users (id, email, password, role) VALUES (3, 'three@three.three', '$argon2id$v=19$m=8192,t=5,p=1$8MDHvZc89+IDwJBvODtJSw$rM3OXVO+BqLD/swRIZ/y/lSh9xQ7JEA+fvcwW3ZnqGI', 'user');
INSERT INTO public.users (id, email, password, role) VALUES (4, 'four@four.four', '$argon2id$v=19$m=8192,t=5,p=1$nWYN3iWsF/RMcOjIioaSsA$q6d4D3Vj8xFgM3F9Bu7g+fZkoZctuKIqbIBJl+aPQhU', 'user');


--
-- Name: figure_id_seq; Type: SEQUENCE SET; Schema: public; Owner: figure
--

SELECT pg_catalog.setval('public.figure_id_seq', 7, true);


--
-- Name: profile_id_seq; Type: SEQUENCE SET; Schema: public; Owner: figure
--

SELECT pg_catalog.setval('public.profile_id_seq', 4, true);


--
-- Name: user_id_seq; Type: SEQUENCE SET; Schema: public; Owner: figure
--

SELECT pg_catalog.setval('public.user_id_seq', 4, true);


--
-- Name: profiles profile_pk; Type: CONSTRAINT; Schema: public; Owner: figure
--

ALTER TABLE ONLY public.profiles
    ADD CONSTRAINT profile_pk PRIMARY KEY (id);


--
-- Name: users user_pk; Type: CONSTRAINT; Schema: public; Owner: figure
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT user_pk PRIMARY KEY (id);


--
-- Name: profile_username_uindex; Type: INDEX; Schema: public; Owner: figure
--

CREATE UNIQUE INDEX profile_username_uindex ON public.profiles USING btree (username);


--
-- Name: user_email_uindex; Type: INDEX; Schema: public; Owner: figure
--

CREATE UNIQUE INDEX user_email_uindex ON public.users USING btree (email);


--
-- Name: figures figure_profile_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: figure
--

ALTER TABLE ONLY public.figures
    ADD CONSTRAINT figure_profile_id_fk FOREIGN KEY (profile_id) REFERENCES public.profiles(id);


--
-- Name: profiles profile_user_id_fk; Type: FK CONSTRAINT; Schema: public; Owner: figure
--

ALTER TABLE ONLY public.profiles
    ADD CONSTRAINT profile_user_id_fk FOREIGN KEY (user_id) REFERENCES public.users(id);

--
-- PostgreSQL database dump complete
--

