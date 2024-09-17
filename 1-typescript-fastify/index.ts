import fastify from 'fastify';
import { createCommercetoolsClient } from './ctClient';

const server = fastify();

const clientId = '';
const clientSecret = '';
const projectKey = 'test-marcelo-soria';
const oauthHost = 'https://auth.us-central1.gcp.commercetools.com';
const host = 'https://api.us-central1.gcp.commercetools.com';

const ctClient = createCommercetoolsClient({
  clientId,
  clientSecret,
  projectKey,
  oauthHost,
  host,
});

server.get('/', async (request, reply) => {
  const cart = await ctClient
    .carts()
    .post({ body: { currency: 'USD', country: 'US' } })
    .execute();

  const createdCartId = cart.body.id;

  const updatedCart = await ctClient
    .carts()
    .withId({ ID: createdCartId })
    .post({
      body: {
        version: cart.body.version,
        actions: [{ action: 'addLineItem', sku: 'RCC-09' }],
      },
    })
    .execute();

  const updatedCartId = updatedCart.body.id;

  console.log(`Cart ID: ${updatedCartId} Updated Cart ID: ${updatedCartId}`);

  return `Cart ID: ${updatedCartId} Updated Cart ID: ${updatedCartId}`;
});

server.listen({ host: '0.0.0.0', port: 8001 }, (err, address) => {
  if (err) {
    console.error(err);
    process.exit(1);
  }
  console.log(`Server listening at ${address}`);
});
