import ws from 'k6/ws';
import { check, sleep } from 'k6';

export const options = {
  vus: 20,
  duration: '30s',
  thresholds: {
    checks: ['rate>0.95'],
    'ws_connecting': ['p(95)<1500'],
  },
};

const base = __ENV.REALTIME_WS_URL || 'ws://127.0.0.1:8085/realtime/servers/00000000-0000-0000-0000-000000000000/events';
const token = __ENV.REALTIME_TOKEN || '';

export default function () {
  const target = token.length > 0 ? `${base}?token=${token}` : `${base}?token=invalid`;
  const response = ws.connect(target, {}, (socket) => {
    socket.on('open', () => {
      socket.send(JSON.stringify({
        event: {
          type: 'action',
          kind: 'move',
          payload: { x: 1, y: 1 },
        },
      }));
    });

    socket.on('message', () => {
      socket.close();
    });

    socket.on('error', () => {
      socket.close();
    });

    socket.setTimeout(() => {
      socket.close();
    }, 1000);
  });

  check(response, {
    'ws upgraded': (r) => r && r.status === 101,
  });

  sleep(0.2);
}
