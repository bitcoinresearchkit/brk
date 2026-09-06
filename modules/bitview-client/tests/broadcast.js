// Offline transport regression: run with node tests/broadcast.js.
import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { once } from 'node:events';
import { BitviewClient } from '../index.js';

const calls = [];
const server = createServer(async (request, response) => {
  const chunks = [];
  for await (const chunk of request) chunks.push(chunk);
  calls.push([request.method, request.url, Buffer.concat(chunks).toString()]);
  if (calls.length === 3) {
    request.socket.destroy(); // Outcome lost after receipt: must not replay.
    return;
  }
  response.writeHead(calls.length === 2 ? 400 : 200, {
    'Content-Type': calls.length === 2 ? 'application/json' : 'text/plain',
    'Cache-Control': 'no-store',
  });
  response.end(calls.length === 2 ? '{"error":"rejected"}' : 'a'.repeat(64));
});
server.listen(0, '127.0.0.1');
await once(server, 'listening');
try {
  const client = new BitviewClient(`http://127.0.0.1:${server.address().port}`);
  assert.equal(await client.postTx('aa'), 'a'.repeat(64));
  await assert.rejects(client.postTx('bb'));
  await assert.rejects(client.postTx('cc'));
  assert.deepEqual(calls, ['aa', 'bb', 'cc'].map(body => ['POST', '/api/tx', body]));
  console.log('Broadcast: plain-text success, rejection and no replay passed.');
} finally {
  server.closeAllConnections();
  await new Promise(resolve => server.close(resolve));
}
