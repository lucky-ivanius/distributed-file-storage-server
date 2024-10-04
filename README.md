# Distributed File Storage Server

## Installation

### Setup

1. Clone the repository:

   ```
   git clone https://github.com/lucky-ivanius/distributed-file-storage-server.git
   cd distributed-file-storage-server
   ```

2. Copy the `.env.example` file to `.env` and adjust the environment variables as needed:

   ```
   cp .env.example .env
   ```

3. Build the project:
   ```
   cargo build --release
   ```

## Running the Application

### Using Docker

1. Build and start the containers:

   ```
   docker-compose up --build
   ```

2. The server will be available at `http://localhost:3000`

### Without Docker

1. Ensure you have a PostgreSQL instance running and accessible.

2. Update the `DATABASE_URL` in your `.env` file to point to your PostgreSQL instance.

3. Run the server:

   ```
   cargo run --release
   ```

4. The server will be available at `http://localhost:3000`

## API Documentation

The OpenAPI documentation is available at `http://localhost:3000/docs` when the server is running.

## API Endpoints

- `POST /upload`: Upload a file
- `GET /download/{file_id}`: Download a file
- `GET /docs`: Access the Swagger UI for API documentation

## Usage Examples

### Uploading a File

```bash
curl -X POST -H "Content-Type: application/octet-stream" --data-binary @/path/to/your/file http://localhost:3000/upload
```

### Downloading a File

```bash
curl -OJ http://localhost:3000/download/{file_id}
```

Replace `{file_id}` with the UUID returned from the upload operation.
