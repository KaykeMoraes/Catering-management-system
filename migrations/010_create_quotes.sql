CREATE TYPE quote_status AS ENUM(
  'draft',
  'sent',
  'accepted',
  'rejected',
  'expired'
);

CREATE TABLE IF NOT EXISTS quotes (
  id BIGINT GENERATED ALWAYS AS IDENTITY,
  customer_id BIGINT NOT NULL,
  status quote_status NOT NULL,
  subtotal DECIMAL(10, 2),
  discount DECIMAL(10, 2),
  total DECIMAL(10, 2),
  notes TEXT,
  expires_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
