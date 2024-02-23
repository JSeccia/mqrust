import React, {useState, useEffect, useCallback} from 'react';
import {Line, Bar} from 'react-chartjs-2';
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
import io from 'socket.io-client';
import {socket} from './socket';

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

const companyNames = [
    "AIR LIQUIDE",
    "AIRBUS GROUP",
    "ALSTOM",
    "ARCELORMITTAL",
    "AXA",
    "BNP PARIBAS",
    "BOUYGUES",
    "CAPGEMINI",
    "CARREFOUR",
    "CREDIT AGRICOLE",
    "DANONE",
    "DASSAULT SYSTEMES",
    "EDENRED",
    "ENGIE",
    "ESSILORLUXOTTICA",
    "EUROFINS SCIENTIFIC",
    "HERMES INTERNATIONAL",
    "KERING",
    "LEGRAND",
    "L'OREAL",
    "LVMH",
    "MICHELIN",
    "ORANGE",
    "PERNOD RICARD",
    "PUBLICIS",
    "RENAULT",
    "SAFRAN",
    "SAINT-GOBAIN",
    "SANOFI",
    "SCHNEIDER ELECTRIC",
    "SOCIETE GENERALE",
    "STELLANTIS",
    "STMICROELECTRONICS",
    "TELEPERFORMANCE",
    "THALES",
    "TOTALENERGIES",
    "UNIBAIL-RODAMCO-WESTFIELD",
    "VEOLIA ENVIRONNEMENT",
    "VINCI",
    "VIVENDI SE"
];

const colors = ['#FF6384', '#36A2EB', '#FFCE56', '#4BC0C0', '#9966FF', '#FF9F40'];

const subscribeToStock = (companyName) => {
    socket?.emit('subscribe-stock', companyName.replace(/\s+/g, ''));
};

const unsubscribeFromStock = (companyName) => {
    socket?.emit('unsubscribe-stock', companyName);
}

const DataVisualization = () => {
    const [stockData, setStockData] = useState(new Map());
    const [chartData, setChartData] = useState({datasets: []});
    const [loading, setLoading] = useState(true); // Initialize loading state
    const [subscriptions, setSubscriptions] = useState(companyNames.reduce((acc, stock) => {
        acc[stock] = true; // Initialize all stocks as subscribed
        return acc;
    }, {}));

    const [isDropdownVisible, setIsDropdownVisible] = useState(false);

    const processChartData = (useCallback((data) => {
        const datasets = [];
        let colorIndex = 0;

        Object.keys(data).forEach(companyName => {
            const companyData = data[companyName] || [];

            console.log(companyData)

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

        setChartData({datasets});
    }, []));

    const barChartData = {
        labels: Array.from(stockData.values()).map((item) => item.name),
        datasets: [
            {
                label: 'rates',
                data: Array.from(stockData.values()).map((item) => item.rate),
                backgroundColor: colors, // array of colors for each bar
                minBarLength: 2, // Minimum length of each bar to ensure visibility
            },
        ],
    };

    const updateChartDataForSingleCompany = (companyName, companyData) => {
        setChartData(prevChartData => {
            const existingDatasetIndex = prevChartData.datasets.findIndex(dataset => dataset.label === companyName);
            const combinedData = companyData.map(point => ({
                x: point.date,
                y: point.opening !== undefined ? point.opening : point.predicted_opening
            }));

            if (existingDatasetIndex !== -1) {
                // Update existing dataset
                const updatedDatasets = prevChartData.datasets.slice(); // Create a shallow copy of datasets
                updatedDatasets[existingDatasetIndex].data = combinedData;
                return {...prevChartData, datasets: updatedDatasets};
            } else {
                // Add new dataset for the company
                const newDataset = {
                    label: companyData.name,
                    data: combinedData,
                    borderColor: colors[prevChartData.datasets.length % colors.length], // Assuming 'colors' array exists
                    fill: false,
                };
                return {...prevChartData, datasets: [...prevChartData.datasets, newDataset]};
            }
        });
    };

    useEffect(() => {
        const fetchPredictions = async () => {
            setLoading(true); // Start loading
            try {
                const response = await axios.get('/predict');
                processChartData(response.data);
                setLoading(false); // Data fetched, stop loading
            } catch (error) {
                console.error('Error fetching predictions:', error);
                setLoading(false); // Error occurred, stop loading
            }
        };

        fetchPredictions();
    }, [processChartData]);

    useEffect(() => {
        socket.on("stock-update", (newData) => {
            setStockData(currentData => {
                const updatedData = new Map(currentData);
                updatedData.set(newData.name, newData);
                return updatedData;
            });
        });
        return () => {
            socket.off("stock-update");
        }
    }, []);

    // useEffect(() => {
    //   const fetchData = async () => {
    //     try {
    //       const response = await axios.get('/data');
    //       setStockData(response.data);
    //     } catch (error) {
    //       console.error('Error fetching stock data:', error);
    //     }
    //   };
    //
    //   fetchData();
    //   const interval = setInterval(fetchData, 5 * 1000);
    //
    //   return () => clearInterval(interval);
    // }, []);

    useEffect(() => {
        for (let company of companyNames) {
            subscribeToStock(company);
        }
        return () => {
            for (let company of companyNames) {
                unsubscribeFromStock(company);
            }
        }
    }, []);


    const handleSubscriptionChange = (stock) => {
        const newSubscriptions = {...subscriptions, [stock]: !subscriptions[stock]};
        setSubscriptions(newSubscriptions);
        // Call subscribe or unsubscribe based on newSubscriptions[stock]
        if (newSubscriptions[stock]) {
            subscribeToStock(stock); // Assuming this subscribes the user
        } else {
            unsubscribeFromStock(stock); // Assuming this unsubscribes the user
        }
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
                        <Line data={chartData} options={chartOptions}/> // Render line chart when data is ready
                    )}
                </div>
            </div>
            <div className="charts-row">
                <div className="chart-container">
                    <Bar data={barChartData} options={barChartOptions}/>
                </div>
            </div>
            <div className="table-container">
                <button onClick={() => setIsDropdownVisible(!isDropdownVisible)}>
                    Manage Subscriptions
                </button>
                {isDropdownVisible && (
                    <div className="subscription-dropdown-content">
                        {companyNames.map((stock, index) => (
                            <div key={index} className="subscription-checkbox">
                                <input
                                    type="checkbox"
                                    id={stock}
                                    name={stock}
                                    checked={subscriptions[stock]}
                                    onChange={() => handleSubscriptionChange(stock)}
                                />
                                <label htmlFor={stock}>{stock}</label>
                            </div>
                        ))}
                    </div>
                )}
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
                    {Array.from(stockData.values()).map((item, index) => (
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
