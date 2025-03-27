-- Add migration script here
CREATE TABLE IF NOT EXISTS sensor_data (
    id SERIAL PRIMARY KEY,
    eui TEXT NOT NULL,
    ack BOOLEAN NOT NULL,
    bat INTEGER NOT NULL,
    cmd TEXT NOT NULL,
    confirmed BOOLEAN NOT NULL,
    data TEXT NOT NULL,
    devaddr TEXT NOT NULL,
    dr TEXT NOT NULL,
    fcnt INTEGER NOT NULL,
    freq BIGINT NOT NULL,
    gws JSONB NOT NULL,
    offline BOOLEAN NOT NULL,
    port INTEGER NOT NULL,
    seqno INTEGER NOT NULL,
    toa INTEGER NOT NULL,
    ts BIGINT NOT NULL,
    received_at TIMESTAMPTZ DEFAULT NOW()
);
