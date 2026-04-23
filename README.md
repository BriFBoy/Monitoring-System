# Monitoring System

A monitoring platform that lets you monitor remote machines in real-time.

- **Agent** - A lightweight program that runs on machines you want to monitor. It collects system info like CPU usage, memory, disk space, hostname, and Linux distro
- **Backend** - An API server that handles communication between the dashboard and agents, stores the list of machines you're monitoring in a database
- **Frontend** - A web dashboard that displays the monitored machines and their stats with charts

## Running Locally with mprocs

1. Ensure PostgreSQL is running and set the `DATABASE_URL` environment variable:
   ```
   export DATABASE_URL=url_to_postgres_database
   ```

2. Build the agent:
   ```
   cd agent && make
   ```

3. Install [miniweb](https://github.com/BriFBoy/MiniWeb) and [mprocs](https://github.com/pvolok/mprocs) if not already installed

4. Run all components:
   ```
   mprocs
   ```

The following services will start:
- Frontend: http://localhost:8080
- Backend: http://localhost:8081
- Agent: UDP listener on port 8080

## Running with Docker Compose

1. Set the `DATABASE_URL` environment variable. It is recommended to create a .env file in the root folder where the docker-compose file is located.
   ```.env
   DATABASE_URL=url_to_postgres_database
   ```

2. Start the containers:
   ```
   docker compose up
   ```

This starts the frontend and backend containers. The backend connects to your PostgreSQL instance via `DATABASE_URL`.

## Database

The backend uses PostgreSQL to store monitored machine IP addresses. Migrations are in `backend/migrations/001_tables.sql`. If you encounter any error when trying to compile the backend, it is recommended to install the sqlx-cli and run the migrations manually.
```
sqlx migrate run
```
