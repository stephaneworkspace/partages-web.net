-- This file should undo anything in `up.sql`
DROP TABLE db01_quote;
ALTER TABLE db01_quote DROP CONSTRAINT db01_quote_da01_user_id_fkey;
