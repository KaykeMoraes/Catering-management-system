CREATE TYPE customer_type AS ENUM('person', 'company');

CREATE TABLE IF NOT EXISTS customers (
  id BIGINT GENERATED ALWAYS AS IDENTITY,
  user_id BIGINT UNIQUE,
  customer_type customer_type,
  full_name VARCHAR(155),
  company_name VARCHAR(255),
  cpf CHAR(11) UNIQUE,
  cnpj CHAR(11) UNIQUE,
  email VARCHAR(255) NOT NULL,
  phone VARCHAR(20) NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES users (id)
);
