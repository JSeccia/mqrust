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
        let cleanValue = value.replace('€', '').replace(',', '.').trim();
        const jsonKey = key.toLowerCase().replace(/[^a-zA-Z0-9]+(.)/g, (m, chr) => chr.toUpperCase());
       if( key !== "name") {
           cleanValue = cleanValue.replace(/\s+/g, '')
       }
        jsonData[jsonKey] = cleanValue;
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
            console.log(message.key.toString());
            io.to(message.key.toString()).emit('stock-update', messageValue);
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

    socket.on("unsubscribe-stock", (data) => {
        socket.leave(data);
    })

    socket.on("disconnect", () => {
        socketMap.delete(socket.id);
    });
});


app.get('/data', (req, res) => {
    const last40Data = stockData.slice(-40);
    res.json(last40Data);
});


app.get('/predict', (req, res) => {

    const pythonProcess = spawn('python', ['C:/Users/j.seccia/Desktop/epitech/mqrust/python_server/forecasting.py']);
    let responseSent = false;

    pythonProcess.stdin.write(JSON.stringify(stockData));
    pythonProcess.stdin.end();

    pythonProcess.stdout.on('data', (data) => {
        if (!responseSent) {
            res.json(JSON.parse(data));
            responseSent = true;
        }
    });

    pythonProcess.stderr.on('data', (data) => {
        console.error(`stderr: ${data}`);
    });

    pythonProcess.on('error', (error) => {
        console.error('Failed to start subprocess:', error);
        if (!responseSent) {
            res.status(500).send('Error starting prediction script');
            responseSent = true;
        }
    });

    pythonProcess.on('close', (code) => {
        if (!responseSent) {
            console.log(`Python script exited with code ${code}`);
            res.status(500).send('Prediction script exited unexpectedly');
            responseSent = true;
        }
    });
});


run();
