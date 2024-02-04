import React, { useEffect, useState } from 'react';
import { Line, Bar } from 'react-chartjs-2';

const DataVisualization = ({ data }) => {
  const timestamps = data.map((item) => item.timestamp); 
  const values = data.map((item) => item.value); 

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

  // Bar Chart Configuration
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
    <div>
      <h2>Data Visualization</h2>
      <div className="line-chart">
        <Line data={lineChartData} />
      </div>
      <div className="bar-chart">
        <Bar data={barChartData} />
      </div>
    </div>
  );
};

export default DataVisualization;
