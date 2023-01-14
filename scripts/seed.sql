--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1

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
    user_id bigint
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

INSERT INTO public.figures (id, title, width, height, profile_id, url, description) VALUES (1, 'first', 1268, 951, 1, 'https://i.imgur.com/XpNCV7a.jpg', NULL);


--
-- Data for Name: profiles; Type: TABLE DATA; Schema: public; Owner: figure
--

INSERT INTO public.profiles (id, username, display_name, user_id) VALUES (DEFAULT, 'one', NULL, 1);
INSERT INTO public.profiles (id, username, display_name, user_id) VALUES (DEFAULT, 'two', NULL, 2);
INSERT INTO public.profiles (id, username, display_name, user_id) VALUES (DEFAULT, 'three', NULL, 3);


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: figure
--

INSERT INTO public.users (id, email, password, role) VALUES (DEFAULT, 'one@one.one', '$argon2id$v=19$m=8192,t=5,p=1$TAhaSdMMlP1AFODrj1Gvaw$lOhoEGfmQe2+SEDjjDlLU4EI/n3ij/vGRlUgUvED8Uc', 'user');
INSERT INTO public.users (id, email, password, role) VALUES (DEFAULT, 'two@two.two', '$argon2id$v=19$m=8192,t=5,p=1$jY4QkgqqJTkt0blQ6TR/tg$XjA5OaSMuNmK98BdbW/UcDzRcD8q90Nailgoi9IP490', 'user');
INSERT INTO public.users (id, email, password, role) VALUES (DEFAULT, 'three@three.three', '$argon2id$v=19$m=8192,t=5,p=1$ZHpPqdueTGnLg4eCP86j0Q$M4Wa686IxTfHV/ZvGDe6vwra1eeoRshviVxXxUTE2L4', 'user');


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

