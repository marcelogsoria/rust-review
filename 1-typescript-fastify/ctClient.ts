import { createApiBuilderFromCtpClient } from '@commercetools/platform-sdk';
import {
  AuthMiddlewareOptions,
  ClientBuilder,
  HttpMiddlewareOptions,
} from '@commercetools/sdk-client-v2';

const createCommercetoolsClient = ({
  clientId,
  clientSecret,
  projectKey,
  oauthHost,
  host,
}: {
  clientId: string;
  clientSecret: string;
  projectKey: string;
  oauthHost: string;
  host: string;
}) => {
  const authOptions: AuthMiddlewareOptions = {
    host: oauthHost,
    credentials: { clientId, clientSecret },
    projectKey,
    fetch,
  };

  const httpOptions: HttpMiddlewareOptions = {
    host,
    fetch,
  };

  const client = new ClientBuilder()
    .withClientCredentialsFlow(authOptions)
    .withHttpMiddleware(httpOptions)
    .build();

  const apiRoot = createApiBuilderFromCtpClient(client).withProjectKey({
    projectKey,
  });

  return apiRoot;
};

export { createCommercetoolsClient };
