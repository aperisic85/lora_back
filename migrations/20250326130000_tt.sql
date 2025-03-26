-- Add migration script here
CREATE TABLE IF NOT EXISTS devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id VARCHAR(50) NOT NULL UNIQUE,
    device_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    description TEXT
);

CREATE INDEX IF NOT EXISTS idx_device_id ON devices (device_id);
CREATE TABLE IF NOT EXISTS sensor_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id VARCHAR(50) NOT NULL REFERENCES devices(device_id) ON DELETE CASCADE,
    data JSONB NOT NULL,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    rssi INT,
    snr FLOAT
);

CREATE INDEX IF NOT EXISTS idx_sensor_device ON sensor_data (device_id);
CREATE INDEX IF NOT EXISTS idx_received_at ON sensor_data (received_at);

CREATE TABLE IF NOT EXISTS lora_packets (
    id SERIAL PRIMARY KEY,
    eui TEXT NOT NULL,
    devaddr TEXT NOT NULL,
    frequency BIGINT NOT NULL,
    data BYTEA,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    gateways JSONB NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_lora_eui ON lora_packets (eui);
