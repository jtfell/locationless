
dropdb locationless_test
createdb locationless_test
DATABASE_URL=postgres://localhost/locationless_test diesel migration run
psql locationless_test < data/seed.sql
