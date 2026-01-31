alter table auth.oauth2_temp_states
    add redirect_url text default '' not null;

alter table auth.oauth2_temp_states
    alter column redirect_url drop default;
