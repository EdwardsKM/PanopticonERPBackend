
\set ON_ERROR_STOP 1
\ir create_tables.sql
\ir dummy_data.sql
\ir functions_and_triggers.sql
/* \ir materialized_views.sql */




-- The ON_ERROR_STOP psql variable then makes psql abort right then and there, so that it doesn't continue to flail about issuing doomed statements to the backend. Leveraging postgres's transactional DML is real handy here, especially when updating in-production live databases and you want to be damned sure you're not sabotaged by unexpected differences between production and development / staging databases.