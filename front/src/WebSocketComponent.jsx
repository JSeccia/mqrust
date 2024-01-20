import React, {useState, useEffect} from 'react';

const WebSocketComponent = () => {
    const [ws, setWs] = useState(null);
    const [messages, setMessages] = useState([]);

    useEffect(() => {
        // Create WebSocket connection.
        const socket = new WebSocket('ws://localhost:8000/echo'); // Adjust the URL to your Rocket server

        // Connection opened
        socket.addEventListener('open', function (event) {
            console.log('Connected to WS Server');
        });

        // Listen for messages
        socket.addEventListener('message', function (event) {
            console.log('Message from server ', event.data);
            setMessages(prevMessages => [...prevMessages, event.data]);
        });

        // Connection closed
        socket.addEventListener('close', function (event) {
            console.log('Disconnected from WS Server');
        });

        // Update the state
        setWs(socket);

        // Cleanup on unmount
        return () => {
            socket.close();
        };
    }, []);

    // Function to send a message
    const sendMessage = (message) => {
        if (ws) {
            ws.send(message);
        }
    };

    return (
        <div>
            <h2>WebSocket Test</h2>
            <button onClick={() => sendMessage('Hello, Rocket!')}>Send Message</button>
            <div>
                <h3>Messages from Server:</h3>
                <ul>
                    {messages.map((msg, index) => (
                        <li key={index}>{msg}</li>
                    ))}
                </ul>
            </div>
        </div>
    );
};

export default WebSocketComponent;
