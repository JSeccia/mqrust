import './App.css';
import DataVisualization from './DataVisualization';


function App() {


  const sampleData = [
    { timestamp: '2023-01-01', openingPrice: 50 },
    { timestamp: '2023-01-02', openingPrice: 55 },
    { timestamp: '2023-01-03', openingPrice: 53 },
    { timestamp: '2023-01-04', openingPrice: 58 },
    { timestamp: '2023-01-05', openingPrice: 62 },
    { timestamp: '2023-01-06', openingPrice: 60 },
    { timestamp: '2023-01-07', openingPrice: 65 },
    { timestamp: '2023-01-08', openingPrice: 67 },
    { timestamp: '2023-01-09', openingPrice: 66 },
    { timestamp: '2023-01-10', openingPrice: 69 },
    { timestamp: '2023-01-11', openingPrice: 72 },
    { timestamp: '2023-01-12', openingPrice: 70 },
    { timestamp: '2023-01-13', openingPrice: 74 },
    { timestamp: '2023-01-14', openingPrice: 77 },
    { timestamp: '2023-01-15', openingPrice: 75 },
    { timestamp: '2023-01-16', openingPrice: 78 },
    { timestamp: '2023-01-17', openingPrice: 80 },
    { timestamp: '2023-01-18', openingPrice: 82 },
    { timestamp: '2023-01-19', openingPrice: 81 },
    { timestamp: '2023-01-20', openingPrice: 83 },
  ];
  

  return (
    <div>
      <header className="App-header">
        <DataVisualization data={sampleData} />
      </header>
    </div>
  );
};

export default App;
