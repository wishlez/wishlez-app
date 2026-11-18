CREATE TABLE IF NOT EXISTS transactions_tags
(
  tag            VARCHAR(100) NOT NULL,
  transaction_id VARCHAR(36)  NOT NULL,

  PRIMARY KEY (tag, transaction_id),

  CONSTRAINT fk_tag
    FOREIGN KEY (tag)
      REFERENCES tags (name)
      ON UPDATE CASCADE ON DELETE CASCADE,

  CONSTRAINT fk_transaction
    FOREIGN KEY (transaction_id)
      REFERENCES transactions (id)
      ON UPDATE CASCADE ON DELETE CASCADE
);
