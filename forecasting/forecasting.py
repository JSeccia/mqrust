import sys
import json
import os
import numpy as np
import pandas as pd
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import load_model
import joblib

# Function to preprocess API data
def preprocess_api_data(data):
    openings = []
    for f in data:
        split_data = f.split(",")
        opening = float(split_data[4].split(":")[1].strip()[:-1].replace(",", "."))
        openings.append(opening)
    return pd.DataFrame({"opening": openings})

# Read input from stdin
api_data = json.load(sys.stdin)

# Check if the model path exists
model_path = '/home/waul/Documents/GitHub/mqrust/forecasting/forecasting_model.h5'
if not os.path.exists(model_path):
    print(json.dumps({"error": f"Model file not found in {model_path}"}))
    sys.exit(1)

# Load the model
model = load_model(model_path)

# Preprocess the API data
processed_data = preprocess_api_data(api_data)

# Load the scaler
scaler_path = '/home/waul/Documents/GitHub/mqrust/forecasting/scaler.save'
if not os.path.exists(scaler_path):
    print(json.dumps({"error": f"Scaler file not found in {scaler_path}"}))
    sys.exit(1)

scaler = joblib.load(scaler_path)

# Scale the data
scaled_data = scaler.transform(processed_data)

# Prepare the input sequence for the model
n_past = len(processed_data)
input_sequence = np.array(scaled_data).reshape(1, n_past, 1)

# Set the number of days to predict
n_days = 3
predicted_prices = []

# Generate predictions
current_sequence = input_sequence
for _ in range(n_days):
    predicted_price = model.predict(current_sequence)
    predicted_price_actual = scaler.inverse_transform(predicted_price)
    predicted_prices.append(predicted_price_actual[0][0])

    current_sequence = np.roll(current_sequence, -1, axis=1)
    current_sequence[0, -1, 0] = predicted_price[0, 0]


predictions = {}
for i, price in enumerate(predicted_prices, 1):
    day_label = f"day{i}"
    predictions[day_label] = price

print(json.dumps(predictions))

