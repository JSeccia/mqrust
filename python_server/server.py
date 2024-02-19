from flask import Flask, request, jsonify
from flask_cors import CORS
from flask_socketio import SocketIO, join_room, leave_room, send, emit
from kafka import KafkaConsumer
import os
import json
import re

app = Flask(__name__)
CORS(app)
app.config['SECRET_KEY'] = 'secret!'
socketio = SocketIO(app, cors_allowed_origins="*")
port = os.environ.get('PORT', 5001)
kafka_broker = os.environ.get('KAFKA_BROKER', 'localhost:9092')

# Kafka setup
consumer = KafkaConsumer(
    'stocks',
    bootstrap_servers=['localhost:9092'],
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


# Kafka consumer loop
def kafka_consumer_loop():
    print("consumer loop")
    for msg in consumer:
        message_value = msg.value  # This is already a dict because of your deserializer
        message_key = msg.key.decode('utf-8')  # Decoding the key from bytes to string
        room = message_key  # You can use message_key directly if it's a simple string
        toto = parse_raw_message(message_value)
        print(json.dumps(toto))
        socketio.emit('stock-update', toto, room=room)



if __name__ == '__main__':
    socketio.start_background_task(kafka_consumer_loop)
    socketio.run(app, host='0.0.0.0', port=port)
