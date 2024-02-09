import {Kafka} from 'kafkajs';
import express from 'express';
import {Socket, Server as SocketServer} from 'socket.io';
import {createServer} from 'http';
import dotenv from 'dotenv';
import cors from 'cors';

dotenv.config();

const app = express(),
    port = process.env.PORT || '5001';
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({extended: true}));
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

const socketMap = new Map();
const keyMap = new Map();


async function run() {

    const consumer = kafka.consumer({groupId: 'test-group'})

    await consumer.connect()
    await consumer.subscribe({topic: 'stocks', fromBeginning: true})

    await consumer.run({
        eachMessage: async ({topic, partition, message}) => {
            if (!keyMap.has(message.key.toString())) {
                keyMap.set(message.key.toString(), new Map());
            }
            keyMap.get(message.key.toString()).forEach((socket) => {
                socket.send(message.value.toString());
            });
            console.log({
                value: message.value.toString(),
            });
        },
    })
}

httpServer.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});

io.on('connection', (socket) => {
    socketMap.set(socket.id, socket);

    socket.on("subscribe-stock", (data) => {
        if (!keyMap.has(data)) {
            socket.send("Stock not found");
            return;
        }
        keyMap.get(data).set(socket.id, socket);
    });

    socket.on("disconnect", () => {
        socketMap.delete(socket.id);
    });
});

run();
