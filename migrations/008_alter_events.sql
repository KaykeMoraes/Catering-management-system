ALTER TABLE events
ADD COLUMN customer_id BIGINT;

ALTER TABLE events
ADD CONSTRAINT fk_customer_event FOREIGN KEY (customer_id) REFERENCES customers (id) ON DELETE CASCADE;
