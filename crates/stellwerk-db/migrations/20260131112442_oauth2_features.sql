create type auth.oauth2_provider as enum ('Discord');

create table auth.oauth2_temp_states
(
    session_id    text                 not null
        constraint oauth2_temp_states_pk
            primary key,
    auth_provider auth.oauth2_provider not null,
    csrf_token    text                 not null,
    expires_at    timestamp            not null
);



