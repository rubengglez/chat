CREATE ROLE "chat-app";
ALTER USER "postgres" WITH ENCRYPTED PASSWORD 'password';
ALTER ROLE "chat-app" WITH LOGIN;
ALTER DATABASE "chat" OWNER TO "chat-app";
GRANT ALL PRIVILEGES ON DATABASE "chat" TO "postgres";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp" SCHEMA public;
