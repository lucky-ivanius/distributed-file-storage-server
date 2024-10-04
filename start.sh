#!/bin/bash
set -e

# Run database migrations
sqlx migrate run

# Start the application
exec distributed_file_storage_server