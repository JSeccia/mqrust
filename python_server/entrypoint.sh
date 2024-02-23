#!/bin/bash

# Function to check database connectivity
wait_for_db() {
    echo "Waiting for database to be ready..."
    # Loop until we're able to connect to the database
    while ! flask db check; do
      sleep 1
    done
}

# Perform database readiness check
wait_for_db

# Apply database migrations
echo "Applying database migrations..."
flask db upgrade

# Start the Flask application
echo "Starting Flask application..."
exec flask run --host=0.0.0.0 --port=8000
