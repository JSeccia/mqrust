import sys
import json
import os

import numpy as np
import pandas as pd
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import load_model
from datetime import datetime, timedelta
import joblib

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'


def preprocess_api_data(data):
    # Convert the list of dictionaries to a DataFrame
    df = pd.DataFrame(data)

    # Initialize a list to store preprocessed data for all stocks
    preprocessed_data = []
    current_date = pd.Timestamp.now()

    for name, group in df.groupby('name'):
        # Ensure the last 10 entries are taken if no date column is available
        group = group.sort_values(by='date') if 'date' in group.columns else group.tail(10)

        # Take only the last 10 data points for each stock
        last_10 = group.tail(10)

        # Generate dates for the last 10 days if no date info is present
        dates = [(current_date - pd.Timedelta(days=x)).date() for x in reversed(range(10))]

        # Preprocess the 'opening' values
        openings = last_10['opening'].apply(lambda x: str(x).replace('\xa0', '').replace(',', '.')).astype(float)

        # Store the preprocessed data along with the stock name and dates
        preprocessed_data.append({
            'name': name,
            'data': openings.values.reshape(-1, 1),  # Reshape for compatibility with the scaler
            'dates': dates  # Associate each data point with a date
        })

    return preprocessed_data


def make_predictions_for_stock(stock_data, model, scaler, n_days=3):
    scaled_data = scaler.transform(stock_data)

    # Prepare the input sequence for the model, assuming the model expects a 2D input shape
    input_sequence = np.array(scaled_data).reshape(1, -1)

    predicted_prices = []
    current_sequence = input_sequence.copy()

    # Generate predictions for the specified number of days
    for _ in range(n_days):
        predicted_price = model.predict(current_sequence, verbose=0)
        # Adjusted to handle 2D predicted_price
        predicted_price_actual = scaler.inverse_transform(predicted_price)  # Assuming the model outputs a 2D array
        predicted_prices.append(predicted_price_actual.flatten()[0])

        # Update the current sequence for the next prediction
        # We need to ensure the sequence remains 2D
        current_sequence = np.roll(current_sequence, -1)
        current_sequence[0, -1] = predicted_price[0, -1]

    return predicted_prices


api_data = json.load(sys.stdin)

if not api_data:
    print(json.dumps({"error": "No data received"}))
    sys.exit(1)

processed_data = preprocess_api_data(api_data)

model_path = '/python_server/forecasting_model.h5'
if not os.path.exists(model_path):
    print(json.dumps({"error": f"Model file not found in {model_path}"}))
    sys.exit(1)

model = load_model(model_path)

# Load and apply the scaler
scaler_path = '/python_server/scaler.save'
if not os.path.exists(scaler_path):
    print(json.dumps({"error": f"Scaler file not found in {scaler_path}"}))
    sys.exit(1)

scaler = joblib.load(scaler_path)
predictions = {}
for stock_info in processed_data:
    stock_name = stock_info['name']
    stock_data = stock_info['data']
    stock_predictions = make_predictions_for_stock(stock_data, model, scaler)
    last_10_days = stock_info['dates']
    # Combine historical data with predictions
    combined_data = [{"date": str(date), "opening": float(stock_data[i][0])} for i, date in enumerate(last_10_days)]
    for i, pred in enumerate(stock_predictions, start=1):
        combined_data.append(
            {"date": str((datetime.now() + timedelta(days=i)).date()), "predicted_opening": float(pred)})
    predictions[stock_name] = combined_data

# Now, you can serialize 'predictions' without errors
json_predictions = json.dumps(predictions)
print(json_predictions)
