import React from 'react';
import { Line, Bar } from 'react-chartjs-2';
import './DataVisualization.css'; // Assuming you have a CSS file for styling
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js';

// Registering the components necessary for Line and Bar charts
ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend
);

const DataVisualization = ({ data }) => {
  const timestamps = data.map((item) => item.timestamp);
  const values = data.map((item) => item.openingPrice);

  const lineChartData = {
    labels: timestamps,
    datasets: [
      {
        label: 'Line Chart Data',
        data: values,
        fill: false,
        borderColor: 'rgb(75, 192, 192)',
        tension: 0.1,
      },
    ],
  };

  const barChartData = {
    labels: timestamps,
    datasets: [
      {
        label: 'Bar Chart Data',
        data: values,
        backgroundColor: 'rgba(75, 192, 192, 0.2)',
        borderColor: 'rgba(75, 192, 192, 1)',
        borderWidth: 1,
      },
    ],
  };

  return (
    <div className="data-visualization-container">
      <h2>Data Visualization</h2>
      <div className="chart-container line-chart">
        <Line data={lineChartData} />
      </div>
      <div className="chart-container bar-chart">
        <Bar data={barChartData} />
      </div>
      <div className="table-container">
        <h3>Raw Data</h3>
        <table>
          <thead>
            <tr>
              <th>Timestamp</th>
              <th>Opening Price</th>
            </tr>
          </thead>
          <tbody>
            {data.map((item, index) => (
              <tr key={index}>
                <td>{item.timestamp}</td>
                <td>{item.openingPrice}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default DataVisualization;
