create table "user"
(
    id       bigserial
        constraint user_pk
            primary key,
    email    text not null
        constraint email_check
            check (email = lower(email)),
    password text not null,
    role     text not null
);

create unique index user_email_uindex
    on "user" (email);

create table profile
(
    id              bigserial
        constraint profile_pk
            primary key,
    username        text not null,
    display_name    text,
    user_id         bigint
        constraint profile_user_id_fk
            references "user",
    profile_picture text,
    bio             text,
    banner          text
);

create unique index profile_username_uindex
    on profile (username);

create table figure
(
    id          bigserial,
    title       text    not null,
    width       integer not null,
    height      integer not null,
    profile_id  bigint  not null
        constraint figure_profile_id_fk
            references profile,
    url         text    not null,
    description text
);
