create table "user"
(
	id bigserial
		constraint user_pk
			primary key,
	email text not null
		constraint email_check
			check (email = lower(email)),
	password text not null,
	role text not null
);

create unique index user_email_uindex
	on "user" (email);

create table profile
(
	id bigserial
		constraint profile_pk
			primary key,
	username text not null,
	display_name text,
	user_id bigint
		constraint profile_user_id_fk
			references "user"
);

create table figure
(
	id bigserial,
	title text not null,
	width integer,
	height integer,
	profile_id bigint not null
		constraint figure_profile_id_fk
			references profile
);


