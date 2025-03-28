-- Add migration script here
ALTER TABLE sensor_data ALTER COLUMN gws TYPE jsonb;
CREATE INDEX idx_sensor_eui ON sensor_data(eui);
CREATE INDEX idx_sensor_ts ON sensor_data(ts);
CREATE INDEX IF NOT EXISTS sensor_data_eui_idx ON sensor_data (eui);
CREATE INDEX IF NOT EXISTS sensor_data_eui_received_at_idx ON sensor_data (eui, received_at DESC);
