import React, { useState, useEffect } from 'react';
import { Line, Bar } from 'react-chartjs-2';
import axios from 'axios';
import './DataVisualization.css';
import 'chartjs-adapter-date-fns';
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
  TimeScale // Import TimeScale here
} from 'chart.js';

ChartJS.register(
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  BarElement,
  Title,
  Tooltip,
  Legend,
  TimeScale // Add TimeScale to the register call
);


const DataVisualization = () => {
  const [stockData, setStockData] = useState([]);
  const [chartData, setChartData] = useState({ datasets: [] });
  const [loading, setLoading] = useState(true); // Initialize loading state

  const colors = ['#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF', '#FF9F40'];

  useEffect(() => {
    const fetchPredictions = async () => {
      setLoading(true); // Start loading
      try {
        const response = await axios.get('http://localhost:5001/predict');
        processChartData(response.data);
        setLoading(false); // Data fetched, stop loading
      } catch (error) {
        console.error('Error fetching predictions:', error);
        setLoading(false); // Error occurred, stop loading
      }
    };

    fetchPredictions();
  }, []);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await axios.get('http://localhost:5001/data');
        setStockData(response.data);
      } catch (error) {
        console.error('Error fetching stock data:', error);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 5 * 1000);

    return () => clearInterval(interval);
  }, []);

  const processChartData = (data) => {
    const datasets = [];
    let colorIndex = 0;

    Object.keys(data).forEach(companyName => {
      const companyData = data[companyName] || [];

      // Combine actual and predicted data into a single array
      const combinedData = companyData.map(point => ({
        x: point.date,
        y: point.opening !== undefined ? point.opening : point.predicted_opening
      }));

      datasets.push({
        label: companyName,
        data: combinedData,
        borderColor: colors[colorIndex % colors.length],
        fill: false,
      });

      colorIndex++;
    });

    setChartData({ datasets });
  };

  const barChartData = {
    labels: stockData.map((item) => item.name),
    datasets: [
      {
        label: 'rates',
        data: stockData.map((item) => item.rate),
        backgroundColor: colors, // array of colors for each bar
        minBarLength: 2, // Minimum length of each bar to ensure visibility
      },
    ],
  };



  const barChartOptions = {
    scales: {
      x: {
        beginAtZero: true,
        ticks: {
          callback: () => ''
        },
        offset: true,
      },
      y: {
        beginAtZero: false,
      }
    },
    plugins: {
      title: {
        display: true,
        text: 'Company Rates',
        padding: {
          top: 10,
          bottom: 30
        },
        font: {
          size: 18
        }
      },
      legend: {
        display: false,
      }
    },
    maintainAspectRatio: true,
    responsive: true,
  };


  const chartOptions = {
    scales: {
      x: {
        type: 'time',
        time: {
          unit: 'day',
        },
      },
      y: {
        beginAtZero: true,
      },
    },
    plugins: {
      title: {
      display: true,
      text: 'Company Stock Prices - Actual and Predicted',
      padding: {
        top: 10,
        bottom: 30
      },
      font: {
        size: 18
      }
    },
      legend: {
        display: true,
      },
    },
  };

  return (
    <div className="data-visualization-container">
      <h2>CyptoViz</h2>
      <div className="charts-row"> {/* This div wraps the charts and places them in a row */}
        <div className="chart-container">
          {loading ? ( // Check if line chart is still loading
            <div>Predicting the future</div> // Loading message for line chart
          ) : (
            <Line data={chartData} options={chartOptions} /> // Render line chart when data is ready
          )}
        </div>
        <div className="chart-container">
          <Bar data={barChartData} options={barChartOptions} />
        </div>
      </div>
      <div className="table-container">
        <h3>Live Data</h3>
        <table>
          <thead>
            <tr>
              <th>Name</th>
              <th>Rate</th>
              <th>Variation</th>
              <th>High</th>
              <th>Opening Price</th>
              <th>Low</th>
              <th>Volume</th>
            </tr>
          </thead>
          <tbody>
            {stockData.map((item, index) => (
              <tr key={index}>
                <td>{item.name}</td>
                <td>{item.rate}</td>
                <td>{item.variation}</td>
                <td>€{item.high}</td>
                <td>€{item.opening}</td>
                <td>€{item.low}</td>
                <td>{item.volume}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default DataVisualization;
