INSERT INTO accounts
  (id, name, account_type, is_built_in)
VALUES (:id, :name, :account_type, :is_built_in)
ON CONFLICT (id) DO UPDATE SET id = excluded.id;
