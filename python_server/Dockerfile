# Step 1: Build the React application
# Use a Node.js base image to build the React app
FROM node:latest AS react-build
WORKDIR /app
# Copy the React app files into the image
COPY datavis/package.json datavis/package-lock.json ./
# Install React app dependencies
RUN npm install --force
COPY datavis/ ./
# Build the React app
RUN npm run build

# Step 2: Set up the Flask application
FROM python:3.11-slim
WORKDIR /app
# Copy the Flask app files into the image
COPY . .
# Copy the React build from the previous stage
COPY --from=react-build /app/build ./static
# Install Flask app dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Expose the port Flask will listen on
EXPOSE 8000

# Run the Flask app using Gunicorn in production
#CMD ["gunicorn", "-w", "1", "-k", "eventlet", "-b", "0.0.0.0:8000", "app:app"]
CMD ["flask", "run", "--host=0.0.0.0", "--port=8000"]
