import { Kafka } from 'kafkajs';

const kafka = new Kafka({
	clientId: '1',
	brokers: ['localhost:9092'],
});

async function run() {

	const consumer = kafka.consumer({groupId: 'test-group'})

	await consumer.connect()
	await consumer.subscribe({topic: 'test-topic', fromBeginning: true})

	await consumer.run({
		eachMessage: async ({topic, partition, message}) => {
			console.log({
				value: message.value.toString(),
			})
		},
	})
}

run();