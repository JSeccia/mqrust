import numpy as np
import pandas as pd
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import load_model
from tensorflow.keras.preprocessing.sequence import pad_sequences
import joblib
import os

model_path = '/home/waul/Documents/GitHub/mqrust/forecasting/forecasting_model.h5'
if os.path.exists(model_path):
    model = load_model(model_path)
else:
    print(f"Model file not found in {model_path}")
    exit()

api_data = ['name: VIVENDI SE, rate: 9.34€, variation: -0.59%, high: 9.51€, opening: 8.66€, low: 9.21€, volume: 927 455',
 'name: VIVENDI SE, rate: 9.81€, variation: 0.18%, high: 10.13€, opening: 10.61€, low: 9.62€, volume: 605 873',
 'name: VIVENDI SE, rate: 8.21€, variation: -0.03%, high: 8.32€, opening: 11.3€, low: 7.86€, volume: 666 267',
 'name: VIVENDI SE, rate: 8.06€, variation: -0.41%, high: 8.39€, opening: 10.03€, low: 7.69€, volume: 1 732 510',
 'name: VIVENDI SE, rate: 11.6€, variation: 0.83%, high: 11.71€, opening: 9.4€, low: 11.34€, volume: 704 405',
 'name: VIVENDI SE, rate: 8.1€, variation: -0.1%, high: 8.45€, opening: 11.97€, low: 7.79€, volume: 1 862 965',
 'name: VIVENDI SE, rate: 11.82€, variation: 0.06%, high: 11.95€, opening: 10.38€, low: 11.71€, volume: 1 569 112',
 'name: VIVENDI SE, rate: 9.36€, variation: -0.65%, high: 9.7€, opening: 8.51€, low: 9.19€, volume: 912 373',
 'name: VIVENDI SE, rate: 10.21€, variation: 0.23%, high: 10.36€, opening: 10.94€, low: 9.81€, volume: 694 819',
 'name: VIVENDI SE, rate: 11.86€, variation: -0.7%, high: 12.05€, opening: 9.61€, low: 11.38€, volume: 1 111 290',
 'name: VIVENDI SE, rate: 10.14€, variation: -0.06%, high: 10.39€, opening: 11.92€, low: 9.72€, volume: 1 552 650',
 'name: VIVENDI SE, rate: 8.2€, variation: -0.81%, high: 8.6€, opening: 9.81€, low: 7.82€, volume: 821 970',
 'name: VIVENDI SE, rate: 9.39€, variation: 0.59%, high: 9.62€, opening: 11.18€, low: 9.02€, volume: 1 026 669',
 'name: VIVENDI SE, rate: 8.99€, variation: 0.7%, high: 9.48€, opening: 10.71€, low: 8.57€, volume: 524 085',
 'name: VIVENDI SE, rate: 8.67€, variation: -0.25%, high: 8.97€, opening: 8.8€, low: 8.27€, volume: 1 792 749',
 'name: VIVENDI SE, rate: 10.69€, variation: -0.95%, high: 11.02€, opening: 10.87€, low: 10.59€, volume: 1 740 140',
 'name: VIVENDI SE, rate: 11.81€, variation: 0.41%, high: 12.13€, opening: 8.53€, low: 11.32€, volume: 1 184 107',
 'name: VIVENDI SE, rate: 9.49€, variation: -0.91%, high: 9.68€, opening: 11.0€, low: 9.12€, volume: 538 327',
 'name: VIVENDI SE, rate: 11.6€, variation: -0.07%, high: 12.02€, opening: 10.74€, low: 11.31€, volume: 1 112 896',
 'name: VIVENDI SE, rate: 9.56€, variation: -0.45%, high: 9.86€, opening: 10.56€, low: 9.4€, volume: 934 093',
 'name: VIVENDI SE, rate: 10.03€, variation: 0.95%, high: 10.14€, opening: 11.66€, low: 9.79€, volume: 1 798 477',
 'name: VIVENDI SE, rate: 10.1€, variation: 0.22%, high: 10.35€, opening: 10.6€, low: 9.98€, volume: 692 530',
 'name: VIVENDI SE, rate: 10.74€, variation: -0.92%, high: 10.98€, opening: 8.2€, low: 10.59€, volume: 1 405 830',
 'name: VIVENDI SE, rate: 8.48€, variation: 0.8%, high: 8.59€, opening: 10.25€, low: 7.99€, volume: 712 474',
 'name: VIVENDI SE, rate: 10.97€, variation: -0.89%, high: 11.13€, opening: 10.23€, low: 10.63€, volume: 1 051 352',
 'name: VIVENDI SE, rate: 10.24€, variation: 0.92%, high: 10.39€, opening: 10.25€, low: 9.86€, volume: 1 639 395',
 'name: VIVENDI SE, rate: 10.2€, variation: -0.66%, high: 10.56€, opening: 8.25€, low: 9.91€, volume: 1 892 208',
 'name: VIVENDI SE, rate: 10.58€, variation: -0.56%, high: 10.75€, opening: 9.57€, low: 10.33€, volume: 908 147',
 'name: VIVENDI SE, rate: 11.14€, variation: 0.94%, high: 11.46€, opening: 11.64€, low: 10.74€, volume: 1 309 232',
 'name: VIVENDI SE, rate: 10.83€, variation: -0.93%, high: 11.27€, opening: 11.31€, low: 10.39€, volume: 1 813 281',
 'name: VIVENDI SE, rate: 11.08€, variation: -0.54%, high: 11.57€, opening: 10.03€, low: 10.78€, volume: 1 592 137',
 'name: VIVENDI SE, rate: 10.24€, variation: 0.85%, high: 10.7€, opening: 11.59€, low: 10.13€, volume: 1 256 968',
 'name: VIVENDI SE, rate: 10.75€, variation: 0.96%, high: 11.13€, opening: 9.25€, low: 10.32€, volume: 1 177 528',
 'name: VIVENDI SE, rate: 10.39€, variation: 0.92%, high: 10.6€, opening: 8.58€, low: 9.92€, volume: 761 447',
 'name: VIVENDI SE, rate: 9.07€, variation: -0.18%, high: 9.22€, opening: 10.37€, low: 8.88€, volume: 1 304 798',
 'name: VIVENDI SE, rate: 8.24€, variation: 0.87%, high: 8.53€, opening: 11.96€, low: 7.86€, volume: 1 266 654',
 'name: VIVENDI SE, rate: 11.42€, variation: 0.72%, high: 11.73€, opening: 11.81€, low: 11.07€, volume: 1 749 975',
 'name: VIVENDI SE, rate: 11.33€, variation: 0.28%, high: 11.76€, opening: 9.6€, low: 11.07€, volume: 729 679',
 'name: VIVENDI SE, rate: 8.5€, variation: 0.02%, high: 8.83€, opening: 8.32€, low: 8.1€, volume: 1 759 039',
 'name: VIVENDI SE, rate: 9.8€, variation: -0.95%, high: 10.14€, opening: 10.99€, low: 9.66€, volume: 1 595 571',
 'name: VIVENDI SE, rate: 8.09€, variation: 0.17%, high: 8.26€, opening: 8.75€, low: 7.63€, volume: 597 467',
 'name: VIVENDI SE, rate: 10.23€, variation: 0.02%, high: 10.64€, opening: 11.83€, low: 9.89€, volume: 621 092',
 'name: VIVENDI SE, rate: 8.66€, variation: 0.65%, high: 8.86€, opening: 11.78€, low: 8.23€, volume: 1 576 410',
 'name: VIVENDI SE, rate: 8.97€, variation: -0.46%, high: 9.15€, opening: 11.87€, low: 8.65€, volume: 1 812 847',
 'name: VIVENDI SE, rate: 9.97€, variation: -0.91%, high: 10.27€, opening: 10.13€, low: 9.54€, volume: 1 217 685',
 'name: VIVENDI SE, rate: 11.75€, variation: 0.31%, high: 11.91€, opening: 8.44€, low: 11.39€, volume: 1 859 149',
 'name: VIVENDI SE, rate: 9.71€, variation: 0.47%, high: 9.88€, opening: 8.58€, low: 9.27€, volume: 1 015 873',
 'name: VIVENDI SE, rate: 8.34€, variation: 0.18%, high: 8.78€, opening: 10.58€, low: 8.08€, volume: 1 620 647',
 'name: VIVENDI SE, rate: 10.79€, variation: 0.94%, high: 10.94€, opening: 9.32€, low: 10.61€, volume: 1 223 190',
 'name: VIVENDI SE, rate: 10.38€, variation: -0.07%, high: 10.72€, opening: 10.68€, low: 9.95€, volume: 946 510',
 'name: VIVENDI SE, rate: 10.6€, variation: -0.26%, high: 10.75€, opening: 11.51€, low: 10.35€, volume: 569 100',
 'name: VIVENDI SE, rate: 10.75€, variation: 0.27%, high: 11.18€, opening: 9.29€, low: 10.57€, volume: 1 116 674',
 'name: VIVENDI SE, rate: 8.65€, variation: -0.85%, high: 9.1€, opening: 9.09€, low: 8.39€, volume: 956 413',
 'name: VIVENDI SE, rate: 10.89€, variation: -0.83%, high: 11.25€, opening: 8.72€, low: 10.68€, volume: 511 514',
 'name: VIVENDI SE, rate: 10.13€, variation: -0.88%, high: 10.38€, opening: 9.83€, low: 9.66€, volume: 1 478 095',
 'name: VIVENDI SE, rate: 8.09€, variation: 0.11%, high: 8.56€, opening: 8.47€, low: 7.79€, volume: 1 093 717',
 'name: VIVENDI SE, rate: 9.45€, variation: -0.14%, high: 9.57€, opening: 11.17€, low: 9.17€, volume: 607 199',
 'name: VIVENDI SE, rate: 9.17€, variation: 0.3%, high: 9.33€, opening: 9.09€, low: 9.02€, volume: 1 730 675',
 'name: VIVENDI SE, rate: 10.7€, variation: 0.28%, high: 10.99€, opening: 9.58€, low: 10.28€, volume: 1 288 589',
 'name: VIVENDI SE, rate: 10.31€, variation: 0.49%, high: 10.63€, opening: 9.38€, low: 9.91€, volume: 1 022 559'
]

def preprocess_api_data(data):
    openings = []
    for f in data:
        split_data = f.split(",")
        opening = float(split_data[4].split(":")[1].strip()[:-1].replace(",", "."))
        openings.append(opening)
    return pd.DataFrame({"opening": openings})

processed_data = preprocess_api_data(api_data)

scaler = MinMaxScaler()

scaler = joblib.load('/home/waul/Documents/GitHub/mqrust/forecasting/scaler.save')

scaled_data = scaler.transform(processed_data)

n_past = len(processed_data)
input_sequence = np.array(scaled_data).reshape(1, n_past, 1)


n_days = 10
predicted_prices = []


current_sequence = input_sequence

for _ in range(n_days):
    predicted_price = model.predict(current_sequence)

    predicted_price_actual = scaler.inverse_transform(predicted_price)
    predicted_prices.append(predicted_price_actual[0][0])
    
    current_sequence = np.roll(current_sequence, -1, axis=1)
    current_sequence[0, -1, 0] = predicted_price[0, 0]


for i, price in enumerate(predicted_prices, 1):
    print(f"Predicted price for day {i}: {price}€")
