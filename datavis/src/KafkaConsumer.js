import React, { useEffect, useState } from 'react';
import { Kafka } from 'kafkajs';
import DataVisualization from './DataVisualization';

const KafkaConsumerComponent = () => {
  const [data, setData] = useState([]);

  useEffect(() => {
    const kafka = new Kafka({
        clientId: '1',
	      brokers: ['localhost:9092'],
    });

    const consumer = kafka.consumer({ groupId: 'consumer-group' });

    const runConsumer = async () => {
      await consumer.connect();
      await consumer.subscribe({ topic: 'test-group', fromBeginning: true });

      await consumer.run({
        eachMessage: async ({ topic, partition, message }) => {
          const receivedData = JSON.parse(message.value.toString('utf8'));
          console.log(receivedData)
          setData((prevData) => [...prevData, receivedData]);
        },
      });
    };

    runConsumer();

    return () => {
      consumer.disconnect();
    };
  }, [data]);

  return (
    <div>
        <h2>Kafka Consumer</h2>
        <DataVisualization data={data} />
    </div>
  );
};

export default KafkaConsumerComponent;
