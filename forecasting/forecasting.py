import numpy as np
import pandas as pd
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import load_model
import joblib

import os

model_path = 'C:/Users/33698/Documents/Github/mqrust/forecasting/forecasting_model.h5'
if os.path.exists(model_path):
    model = load_model(model_path)
else:
    print(f"Model file not found in {model_path}")


# Example API data
api_data = "name: VIVENDI SE, rate: 10.30€, variation: 0.00%, high: 10.47€, opening: 10.35€, low: 10.30€, volume: 1 732 543"

# Function to extract numeric features from API data
def preprocess_api_data(data):
    # Extract numeric values and convert them to float
    rate = float(data.split(",")[1].split(":")[1].strip()[:-1].replace(",", "."))
    high = float(data.split(",")[3].split(":")[1].strip()[:-1].replace(",", "."))
    opening = float(data.split(",")[4].split(":")[1].strip()[:-1].replace(",", "."))
    low = float(data.split(",")[5].split(":")[1].strip()[:-1].replace(",", "."))
    
    # Return a DataFrame with the extracted features
    return pd.DataFrame({ "opening": [opening]})

# Preprocess the API data
processed_data = preprocess_api_data(api_data)


# Assume the scaler was saved previously after fitting to the training data
scaler = MinMaxScaler()
# Load the scaler (you need to implement the save/load mechanism)
scaler = joblib.load('C:/Users/33698/Documents/Github/mqrust/forecasting/scaler.save')

# Scale the processed API data
scaled_data = scaler.transform(processed_data)

# Reshape the data to fit the LSTM input requirements
# Assuming your model was trained with sequences of length 'n_past'
n_past = 10  # Example value, you should use the actual value you used during training
input_sequence = np.array(scaled_data).reshape(1, n_past, -1)



# Predict the next day's price
predicted_price = model.predict(input_sequence)

# Inverse scale the prediction to get the actual price value
predicted_price_actual = scaler.inverse_transform(predicted_price)

print(f"Predicted next day's price: {predicted_price_actual[0][0]}€")
