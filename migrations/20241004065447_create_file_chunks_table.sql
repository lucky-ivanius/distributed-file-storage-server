-- Create the file_chunks table
CREATE TABLE IF NOT EXISTS file_chunks (
    id SERIAL PRIMARY KEY,
    file_id UUID NOT NULL,
    chunk_index INTEGER NOT NULL,
    chunk_data BYTEA NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (file_id, chunk_index)
);

-- Create an index on file_id for faster lookups
CREATE INDEX idx_file_chunks_file_id ON file_chunks (file_id);

-- Create an index on the combination of file_id and chunk_index for efficient ordering
CREATE INDEX idx_file_chunks_file_id_chunk_index ON file_chunks (file_id, chunk_index);