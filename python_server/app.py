from flask import Flask, jsonify, request, send_from_directory
from flask_cors import CORS
from flask_socketio import SocketIO, join_room, leave_room, send
from flask_migrate import Migrate
from flask_sqlalchemy import SQLAlchemy

from kafka import KafkaConsumer
import os
import json
import re
import sys

current_dir = os.path.dirname(os.path.abspath(__file__))
if current_dir not in sys.path:
    sys.path.append(current_dir)

from forecasting import make_forecasts

stock_data = []

db_host = os.environ.get('DB_HOST', 'localhost')
db_port = os.environ.get('DB_PORT', '3306')
db_user = os.environ.get('DB_USER', 'stock_user')
db_password = os.environ.get('DB_PASSWORD', 'stock_user')

db = SQLAlchemy()
app = Flask(__name__, static_folder="./static", static_url_path='')
app.config[
    'SQLALCHEMY_DATABASE_URI'] = 'mysql+pymysql://' + db_user + ':' + db_password + '@' + db_host + ':' + db_port + '/stocks'


class StockOpening(db.Model):
    __tablename__ = 'stock_openings'

    id = db.Column(db.Integer, primary_key=True)
    stock_name = db.Column(db.String(80), nullable=False)
    date = db.Column(db.Date, nullable=False)
    opening = db.Column(db.Float, nullable=False)

    def __repr__(self):
        return f'<StockOpening {self.id} {self.opening}>'


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
    value_deserializer=lambda x: safe_json_deserialize(x)
)


# Kafka consumer loop
def kafka_consumer_loop():
    for msg in consumer:
        message_value = msg.value
        message_key = msg.key.decode('utf-8')
        room = message_key
        parsed_message = parse_raw_message(message_value)
        socketio.emit('stock-update', parsed_message, room=room)
        stock_data.append(parsed_message)


def create_app():
    app = Flask(__name__)

    # Configure your Flask app (e.g., from config files or environment variables)
    app.config[
        'SQLALCHEMY_DATABASE_URI'] = 'mysql+pymysql://' + db_user + ':' + db_password + '@' + db_host + ':' + db_port + '/stocks'

    # Initialize plugins
    db.init_app(app)
    migrate = Migrate(app, db)

    # Other app setup (e.g., register blueprints, socketio, etc.)
    socketio.init_app(app)

    return app


app = create_app()


def safe_json_deserialize(x):
    try:
        return json.loads(x.decode('utf-8'))
    except json.JSONDecodeError:
        # Handle the error, for example, by returning None or logging an error message
        print("Received a message that is not valid JSON.")
        return None


# For parsing and cleaning the message data
def parse_raw_message(raw_message):
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


@app.route('/predict', methods=['GET'])
def predict():
    # Use the global stock_data for predictions
    if not stock_data:
        return jsonify({"error": "No stock data available"}), 400

    try:
        # Call the forecasting function with the global stock_data
        predictions = make_forecasts(stock_data)
        # Clear stock_data after making predictions
        return jsonify(predictions)
    except Exception as e:
        print(e)
        return jsonify({"error": "Error processing the forecast"}), 500


@app.route('/', defaults={'path': ''})
@app.route('/<path:path>')
def serve(path):
    if path != "" and os.path.exists(os.path.join(app.static_folder, path)):
        return send_from_directory(app.static_folder, path)
    else:
        return send_from_directory(app.static_folder, 'index.html')


if __name__ == "__main__":
    app.run()
