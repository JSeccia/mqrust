import { Kafka } from 'kafkajs';
import express from 'express';
import { createServer } from 'http';
import {Socket, Server as SocketServer} from 'socket.io';
import dotenv from 'dotenv';
import cors from 'cors';
import { spawn } from 'child_process'; // For running Python script

dotenv.config();

const app = express(),
      port = process.env.PORT || '5001';

app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

function parseRawMessage(rawMessage) {
    const jsonData = {};
    const pairs = rawMessage.split(', ');

    pairs.forEach(pair => {
        const [key, value] = pair.split(': ');
        const cleanValue = value.replace('€', '').replace(',', '.').trim();
        const numericValue = isNaN(Number(cleanValue)) ? cleanValue : Number(cleanValue);
        const jsonKey = key.toLowerCase().replace(/[^a-zA-Z0-9]+(.)/g, (m, chr) => chr.toUpperCase());
        jsonData[jsonKey] = numericValue;
    });

    return jsonData;
}

const httpServer = createServer(app);
const io = new SocketServer(httpServer, {
    cors: {
        origin: "*",
        methods: ["GET", "POST"]
    }
});

const kafka = new Kafka({
    clientId: '1',
    brokers: ['localhost:9092'],
});

const stockData = [];
const socketMap = new Map();
const keyMap = new Map();

async function run() {
    const consumer = kafka.consumer({ groupId: 'test-group' });

    await consumer.connect();
    await consumer.subscribe({ topic: 'stocks', fromBeginning: true });

    await consumer.run({
        eachMessage: async ({ topic, partition, message }) => {
            const rawMessage = message.value.toString();
            const messageValue = parseRawMessage(rawMessage);
            stockData.push(messageValue);
            if (!keyMap.has(message.key.toString())) {
                keyMap.set(message.key.toString(), new Map());
            }
            keyMap.get(message.key.toString()).forEach((socket) => {
                socket.send(messageValue);
            });
            console.log({ value: messageValue });
        },
    });
}

httpServer.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});

io.on('connection', (socket) => {
    socketMap.set(socket.id, socket);

    socket.on("subscribe-stock", (data) => {
        if (!keyMap.has(data)) {
            socket.emit("Stock not found");
            return;
        }
        socket.join(data);
    });

    socket.on("disconnect", () => {
        socketMap.delete(socket.id);
        keyMap.forEach((value, key) => {
            value.delete(socket.id);
        });
    });
});


app.get('/data', (req, res) => {
    const last40Data = stockData.slice(-40);
    res.json(last40Data);
});


const axios = require('axios');

app.get('/predict', async (req, res) => {
    try {
        const flaskResponse = await axios.post('http://127.0.0.1:5000/predict', stockData);
        res.json(flaskResponse.data);
    } catch (error) {
        console.error('Error calling Flask prediction service:', error.message);
        res.status(500).send('Error calling prediction service');
    }
});


run();
