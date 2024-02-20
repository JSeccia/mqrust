from flask import Flask, request, send_from_directory
from flask_cors import CORS
from flask_socketio import SocketIO, join_room, leave_room, send
from kafka import KafkaConsumer
import os
import json
import re

app = Flask(__name__, static_folder="./static", static_url_path='')
socketio = SocketIO(app, cors_allowed_origins="*")
CORS(app)
app.config['SECRET_KEY'] = 'secret!'
port = os.environ.get('PORT', 5001)
kafka_broker = os.environ.get('KAFKA_BROKER', 'localhost:9092')

# Kafka setup
consumer = KafkaConsumer(
    'stocks',
    bootstrap_servers=[kafka_broker],
    auto_offset_reset='earliest',
    group_id='test-group',
    value_deserializer=lambda x: json.loads(x.decode('utf-8'))
)


# For parsing and cleaning the message data
def parse_raw_message(raw_message):
    print(raw_message)
    jsonData = {}
    for key, value in raw_message.items():  # Use .items() to get both keys and values
        clean_value = value.replace('€', '').replace(',', '.').strip()
        # Use a regular expression to replace non-alphanumeric characters with nothing
        json_key = re.sub(r'[^a-zA-Z0-9]+(.)', lambda m: m.group(1).upper(), key.lower())
        if key != "name":
            # Ensure to replace non-breaking space and standard spaces
            clean_value = re.sub(r'\s+', '', clean_value.replace('\xa0', ''))
        jsonData[json_key] = clean_value

    return jsonData


# Socket.IO events
@socketio.on('connect')
def handle_connect():
    print(f'Client connected: {request.sid}')


@socketio.on('disconnect')
def handle_disconnect():
    print(f'Client disconnected: {request.sid}')


@socketio.on('subscribe-stock')
def handle_subscribe_stock(data):
    join_room(data)
    send("Subscribed to stock updates", room=data)


@socketio.on('unsubscribe-stock')
def handle_unsubscribe_stock(data):
    leave_room(data)
    send("Unsubscribed from stock updates", room=data)


@app.route('/', defaults={'path': ''})
@app.route('/<path:path>')
def serve(path):
    if path != "" and os.path.exists(os.path.join(app.static_folder, path)):
        return send_from_directory(app.static_folder, path)
    else:
        return send_from_directory(app.static_folder, 'index.html')


# Kafka consumer loop
def kafka_consumer_loop():
    print("consumer loop")
    for msg in consumer:
        message_value = msg.value
        message_key = msg.key.decode('utf-8')
        room = message_key
        socketio.emit('stock-update', parse_raw_message(message_value), room=room)


def create_app():
    # Initialize your Flask app here
    # Start Kafka consumer loop as a background task
    socketio.start_background_task(kafka_consumer_loop)
    return app


app = create_app()
