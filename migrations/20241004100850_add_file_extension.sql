-- Add file_extension column to file_chunks table
ALTER TABLE file_chunks
ADD COLUMN file_extension VARCHAR(10);

-- Update existing rows to have a default extension if needed
-- You may want to adjust or remove this based on your needs
UPDATE file_chunks
SET file_extension = 'bin'
WHERE file_extension IS NULL;

-- Make file_extension NOT NULL for future inserts
ALTER TABLE file_chunks
ALTER COLUMN file_extension SET NOT NULL;

-- Add an index on file_id and file_extension for faster lookups
CREATE INDEX idx_file_chunks_file_id_extension ON file_chunks (file_id, file_extension);