CREATE TABLE IF NOT EXISTS transactions
(
  id          VARCHAR(36) PRIMARY KEY,
  account_id  VARCHAR(36)                                        NOT NULL,
  amount      INTEGER                                            NOT NULL,
  description TEXT,
  date        TIMESTAMP WITH TIME ZONE                           NOT NULL,
  journal_id  VARCHAR(36),
  created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,

  CONSTRAINT fk_account
    FOREIGN KEY (account_id)
      REFERENCES accounts (id)
      ON DELETE CASCADE,

  CONSTRAINT fk_journal
    FOREIGN KEY (journal_id)
      REFERENCES journals (id)
      ON DELETE SET NULL
);
