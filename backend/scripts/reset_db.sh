
dropdb locationless
createdb locationless
diesel migration run
psql locationless < data/seed.sql
