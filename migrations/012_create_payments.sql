CREATE TYPE payment_method AS ENUM('cash', 'card', 'check', 'pix', 'bank transfer');

CREATE TYPE payment_status AS ENUM('pending', 'paid', 'refunded', 'cancelled');

CREATE TABLE payments (
  id BIGINT GENERATED ALWAYS AS IDENTITY,
  event_id BIGINT NOT NULL,
  amount DECIMAL(10, 2) NOT NULL,
  method payment_method NOT NULL,
  status TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  paid_at TIMESTAMPTZ
);
