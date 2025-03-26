psql -U postgres -d user_db

CREATE USER users WITH PASSWORD 'far2ec4u';
GRANT CONNECT ON DATABASE userdb TO users;
GRANT USAGE ON SCHEMA public TO users;
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE users TO users;

GRANT USAGE, SELECT ON SEQUENCE users_id_seq TO users;

