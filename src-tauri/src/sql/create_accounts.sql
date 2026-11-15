CREATE TABLE IF NOT EXISTS accounts
(
  id           VARCHAR(36) PRIMARY KEY,
  name         VARCHAR(100) NOT NULL,
  account_type TEXT         NOT NULL,
  is_built_in  BOOLEAN      NOT NULL DEFAULT FALSE,

  CONSTRAINT validate_account_type CHECK (account_type IN ('Asset', 'Liability', 'Equity', 'Income', 'Expense'))
);
