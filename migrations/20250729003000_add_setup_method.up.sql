-- Add setup_method column to config_records table
ALTER TABLE config_records ADD COLUMN setup_method TEXT DEFAULT 'config_file';