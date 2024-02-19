from flask import Flask, request, jsonify
import os
import numpy as np
import pandas as pd
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import load_model
from datetime import datetime, timedelta
import joblib

app = Flask(__name__)

os.environ['TF_CPP_MIN_LOG_LEVEL'] = '2'

current_script_dir = os.path.dirname(os.path.abspath(__file__))

model_path = os.path.join(current_script_dir, 'forecasting_model.h5')
scaler_path = os.path.join(current_script_dir, 'scaler.save')

if not os.path.exists(model_path) or not os.path.exists(scaler_path):
    raise Exception("Model or scaler file not found")

model = load_model(model_path)
scaler = joblib.load(scaler_path)

def preprocess_api_data(data):

    df = pd.DataFrame(data)

    preprocessed_data = []
    current_date = pd.Timestamp.now()

    for name, group in df.groupby('name'):
        group = group.sort_values(by='date') if 'date' in group.columns else group.tail(10)
        last_10 = group.tail(10)
        dates = [(current_date - pd.Timedelta(days=x)).date() for x in reversed(range(10))]
        openings = last_10['opening'].apply(lambda x: str(x).replace('\xa0', '').replace(',', '.')).astype(float)
        preprocessed_data.append({
            'name': name,
            'data': openings.values.reshape(-1, 1), 
            'dates': dates
        })

    return preprocessed_data


def make_predictions_for_stock(stock_data, model, scaler, n_days=3):

    scaled_data = scaler.transform(stock_data)

    input_sequence = np.array(scaled_data).reshape(1, -1)

    predicted_prices = []
    current_sequence = input_sequence.copy()

    for _ in range(n_days):
        predicted_price = model.predict(current_sequence, verbose = 0)
        predicted_price_actual = scaler.inverse_transform(predicted_price)  
        predicted_prices.append(predicted_price_actual.flatten()[0])

       
        current_sequence = np.roll(current_sequence, -1)
        current_sequence[0, -1] = predicted_price[0, -1]

    return predicted_prices


@app.route('/predict', methods=['POST'])
def predict():
    api_data = request.json

    if not api_data:
        return jsonify({"error": "No data received"}), 400

    processed_data = preprocess_api_data(api_data)
    predictions = {}

    for stock_info in processed_data:
        stock_name = stock_info['name']
        stock_data = stock_info['data']
        stock_predictions = make_predictions_for_stock(stock_data, model, scaler)
        last_10_days = stock_info['dates']

        combined_data = [{"date": str(date), "opening": float(stock_data[i][0])} for i, date in enumerate(last_10_days)]
        for i, pred in enumerate(stock_predictions, start=1):
            combined_data.append({"date": str((datetime.now() + timedelta(days=i)).date()), "predicted_opening": float(pred)})
        predictions[stock_name] = combined_data

    return jsonify(predictions)

if __name__ == '__main__':
    app.run(debug=True)