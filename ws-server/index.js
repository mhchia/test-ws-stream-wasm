import { WebSocketServer } from 'ws';

const wss = new WebSocketServer({ port: 12345 });

wss.on('connection', function connection(ws) {
  ws.on('message', function message(data) {
    console.log('received: %s', data);
  });

  ws.send('something');
});
console.log("ws server started")
