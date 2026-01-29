# GitPoort

Unified Git provider gateway for GitHub, GitLab, Bitbucket, and Azure DevOps.

## Features

- **Multi-provider support**: GitHub (OAuth + App), GitLab, Bitbucket, Azure DevOps
- **Single webhook endpoint**: Route webhooks from all providers through `POST /git/webhooks`
- **Snapshot resolution**: Get repository state at any point in time (commit-at-date)
- **Diff computation**: Compare two snapshots with file-level and stats
- **Multi-tenant ready**: Tenant isolation built-in with security enforcement
- **Automatic token refresh**: TokenManager handles token expiration transparently
- **Retry with backoff**: Exponential backoff for network errors and server failures
- **Rate limit handling**: Automatic retry on 429 with distributed rate limit pooling
- **Circuit breaker**: Prevent cascade failures with automatic recovery
- **Health checks**: Kubernetes-ready liveness and readiness probes
- **Graceful shutdown**: Clean process termination with ordered cleanup
- **Webhook replay**: Dead letter queue with retry for failed webhooks
- **Observability**: Hooks for monitoring requests, webhooks, and git operations

## Installation

```bash
npm install gitpoort
```

## Quick Start

```typescript
import { createGitPoort } from 'gitpoort';

const gitpoort = createGitPoort({
  providers: {
    github: {
      oauth: {
        clientId: process.env.GITHUB_CLIENT_ID,
        clientSecret: process.env.GITHUB_CLIENT_SECRET,
        redirectUri: 'https://your-app.com/auth/github/callback',
      },
    },
    gitlab: {
      clientId: process.env.GITLAB_CLIENT_ID,
      clientSecret: process.env.GITLAB_CLIENT_SECRET,
      redirectUri: 'https://your-app.com/auth/gitlab/callback',
    },
  },
  webhookSecrets: {
    github: process.env.GITHUB_WEBHOOK_SECRET,
    gitlab: process.env.GITLAB_WEBHOOK_SECRET,
  },
  cacheDir: '/var/cache/gitpoort',
});

// Get OAuth authorization URL
const githubAdapter = gitpoort.getProvider('github');
const authUrl = githubAdapter.getAuthorizationUrl('state-token');

// Exchange code for token
const tokenInfo = await githubAdapter.exchangeCodeForToken(code);

// List repositories
const repos = await githubAdapter.listRepositories({ token: tokenInfo.accessToken });
```

## Table of Contents

- [Provider Configuration](#provider-configuration)
- [Webhook Handling](#webhook-handling)
- [Webhook Replay](#webhook-replay)
- [Health Checks](#health-checks)
- [Graceful Shutdown](#graceful-shutdown)
- [Circuit Breaker](#circuit-breaker)
- [Rate Limit Pooling](#rate-limit-pooling)
- [Observability](#observability)
- [Redis Queue](#redis-queue)
- [Express Integration](#express-integration)
- [API Reference](#api-reference)

## Provider Configuration

### GitHub OAuth (Long-lived tokens)

```typescript
github: {
  oauth: {
    clientId: 'your-client-id',
    clientSecret: 'your-client-secret',
    redirectUri: 'https://your-app.com/callback',
    scopes: ['repo', 'read:user'],
  },
}
```

### GitHub App (Short-lived, auto-refresh)

```typescript
github: {
  app: {
    appId: 12345,
    privateKey: fs.readFileSync('private-key.pem', 'utf8'),
    clientId: 'your-client-id',
    clientSecret: 'your-client-secret',
    webhookSecret: 'your-webhook-secret',
  },
}
```

### GitLab

```typescript
gitlab: {
  clientId: 'your-client-id',
  clientSecret: 'your-client-secret',
  redirectUri: 'https://your-app.com/callback',
  baseUrl: 'https://gitlab.com', // or self-hosted URL
}
```

### Bitbucket

```typescript
bitbucket: {
  clientId: 'your-client-id',
  clientSecret: 'your-client-secret',
  redirectUri: 'https://your-app.com/callback',
}
```

### Azure DevOps

```typescript
azureDevops: {
  clientId: 'your-client-id',
  clientSecret: 'your-client-secret',
  redirectUri: 'https://your-app.com/callback',
  organization: 'your-org',
}
```

## Webhook Handling

Configure all providers to send webhooks to a single endpoint:

```
POST https://your-app.com/git/webhooks
```

Process incoming webhooks:

```typescript
app.post('/git/webhooks', async (req, res) => {
  try {
    const { delivery, routing } = await gitpoort.webhookRouter.processWebhook(
      req.headers,
      req.body,
      req.rawBody // for signature verification
    );

    // Enqueue job for async processing
    await gitpoort.stores.job.enqueue(
      'process_webhook',
      routing.tenantId,
      routing.connectionId,
      { deliveryId: delivery.deliveryId, eventType: delivery.eventType }
    );

    res.status(202).json({ received: true });
  } catch (error) {
    console.error('Webhook error:', error);
    res.status(400).json({ error: error.message });
  }
});
```

## Webhook Replay

Store and replay failed webhooks with dead letter queue support:

```typescript
import { WebhookReplayManager, InMemoryWebhookReplayStore } from 'gitpoort';

// Create replay manager (use Redis store in production)
const replayStore = new InMemoryWebhookReplayStore();
const replayManager = new WebhookReplayManager(replayStore, {
  maxAttempts: 3,
  retryDelayMs: 60000,
});

// Capture webhook for replay on failure
app.post('/git/webhooks', async (req, res) => {
  const webhookId = await replayManager.capture(
    detectProvider(req.headers),
    req.headers,
    req.body,
    req.rawBody
  );

  try {
    const result = await gitpoort.webhookRouter.processWebhook(
      req.headers,
      req.body,
      req.rawBody
    );
    await replayManager.markCompleted(webhookId);
    res.status(202).json({ received: true });
  } catch (error) {
    await replayManager.markFailed(webhookId, error.message);
    res.status(500).json({ error: 'Processing failed, queued for retry' });
  }
});

// Replay failed webhooks (run periodically)
async function processFailedWebhooks() {
  const results = await replayManager.replayAll(
    async (webhook) => {
      await gitpoort.webhookRouter.processWebhook(
        webhook.headers,
        webhook.payload,
        webhook.rawBody
      );
    },
    { maxRetries: 3 }
  );
  console.log(`Replayed: ${results.succeeded} succeeded, ${results.failed} failed`);
}
```

## Health Checks

Kubernetes-ready health checks for liveness and readiness probes:

```typescript
import { HealthCheck, createRedisHealthCheck, createMemoryHealthCheck } from 'gitpoort';

const health = new HealthCheck();

// Add built-in checks
health.addCheck(createRedisHealthCheck(redis, 'redis'));
health.addCheck(createMemoryHealthCheck(512, 'memory')); // 512MB threshold

// Add custom checks
health.addCheck(async () => ({
  name: 'database',
  status: await db.ping() ? 'healthy' : 'unhealthy',
}));

// Express endpoints
app.get('/health/live', async (req, res) => {
  const result = await health.liveness();
  res.status(result.alive ? 200 : 503).json(result);
});

app.get('/health/ready', async (req, res) => {
  const result = await health.readiness();
  const status = result.status === 'healthy' ? 200 : 503;
  res.status(status).json(result);
});
```

## Graceful Shutdown

Clean process termination with ordered cleanup (LIFO):

```typescript
import { GracefulShutdown, createServerShutdown, createWorkerShutdown } from 'gitpoort';

const shutdown = new GracefulShutdown({
  timeout: 30000,
  logger: console.log,
});

// Register cleanup handlers (executed in reverse order)
shutdown.register('database', async () => {
  await db.close();
});

shutdown.register('redis', async () => {
  await redis.quit();
});

shutdown.register('http-server', createServerShutdown(server));

shutdown.register('queue-workers', createWorkerShutdown(
  () => worker.stop(),
  () => worker.drain()
));

// Start listening for SIGTERM/SIGINT
shutdown.listen();

// Or trigger manually
process.on('uncaughtException', async (err) => {
  console.error('Uncaught exception:', err);
  await shutdown.shutdown(1);
});
```

## Circuit Breaker

Prevent cascade failures with automatic recovery:

```typescript
import { CircuitBreaker, CircuitBreakerRegistry, withCircuitBreaker } from 'gitpoort';

// Single circuit breaker
const circuit = new CircuitBreaker({
  failureThreshold: 5,    // Open after 5 failures
  resetTimeout: 30000,    // Try again after 30s
  halfOpenRequests: 3,    // Allow 3 test requests in half-open
});

// Execute with circuit breaker protection
try {
  const result = await circuit.execute(async () => {
    return await externalApiCall();
  });
} catch (error) {
  if (circuit.getState() === 'open') {
    console.log('Circuit is open, using fallback');
    return fallbackValue;
  }
  throw error;
}

// Registry for multiple circuits
const registry = new CircuitBreakerRegistry();
const githubCircuit = registry.get('github-api', { failureThreshold: 3 });
const gitlabCircuit = registry.get('gitlab-api', { failureThreshold: 5 });

// Wrapper function
const data = await withCircuitBreaker(githubCircuit, async () => {
  return await githubAdapter.listRepositories(token);
});
```

## Rate Limit Pooling

Share rate limit state across multiple instances via Redis:

```typescript
import { RateLimitPool, withRateLimitPool, rateLimitExtractors } from 'gitpoort';

const rateLimitPool = new RateLimitPool(redis, {
  keyPrefix: 'ratelimit',
  safetyMargin: 0.1,  // Reserve 10% of limit
  minRemaining: 5,    // Block when < 5 remaining
});

// Check before making request
const check = await rateLimitPool.check('github', 'api');
if (!check.allowed) {
  console.log(`Rate limited, retry after ${check.retryAfter}ms`);
  await sleep(check.retryAfter);
}

// Update after response
await rateLimitPool.update('github', 'api', {
  remaining: parseInt(response.headers['x-ratelimit-remaining']),
  limit: parseInt(response.headers['x-ratelimit-limit']),
  resetAt: parseInt(response.headers['x-ratelimit-reset']) * 1000,
});

// Or use wrapper for automatic tracking
const data = await withRateLimitPool(rateLimitPool, 'github', 'api', async () => {
  const response = await fetch('https://api.github.com/user/repos');
  return {
    data: await response.json(),
    rateLimit: rateLimitExtractors.github(Object.fromEntries(response.headers)),
  };
});
```

## Observability

Hooks for monitoring all operations:

```typescript
import { createObservability } from 'gitpoort';

const obs = createObservability({
  debug: process.env.NODE_ENV === 'development',
  logger: customLogger, // Optional custom logger
});

// Request hooks
obs.onRequestStart(({ provider, method, path }) => {
  metrics.increment('api_requests', { provider, method });
});

obs.onRequestEnd(({ provider, duration, status }) => {
  metrics.histogram('api_request_duration', duration, { provider, status });
});

obs.onRequestError(({ provider, error }) => {
  metrics.increment('api_errors', { provider, error: error.name });
});

// Webhook hooks
obs.onWebhookReceived(({ provider, eventType }) => {
  metrics.increment('webhooks_received', { provider, eventType });
});

obs.onWebhookProcessed(({ provider, eventType, duration }) => {
  metrics.histogram('webhook_processing_time', duration, { provider, eventType });
});

// Git operation hooks
obs.onGitOperationStart(({ operation, repoId }) => {
  metrics.increment('git_operations', { operation });
});

obs.onGitOperationEnd(({ operation, duration }) => {
  metrics.histogram('git_operation_duration', duration, { operation });
});

// Rate limit hooks
obs.onRateLimitHit(({ provider, retryAfter }) => {
  metrics.increment('rate_limits_hit', { provider });
  alerting.warn(`Rate limit hit for ${provider}, retry after ${retryAfter}s`);
});
```

## Redis Queue

Reliable background job processing with Redis:

```typescript
import { RedisQueue, createRedisQueue } from 'gitpoort';

const queue = createRedisQueue(redis, {
  keyPrefix: 'gitpoort:jobs',
  defaultPriority: 5,
  maxRetries: 3,
  retryDelay: 5000,
});

// Enqueue jobs
await queue.enqueue({
  type: 'process_webhook',
  payload: { deliveryId: 'abc123', eventType: 'push' },
  priority: 10, // Higher = processed first
});

// Process jobs
const worker = queue.createWorker(async (job) => {
  switch (job.type) {
    case 'process_webhook':
      await processWebhook(job.payload);
      break;
    case 'sync_repository':
      await syncRepository(job.payload);
      break;
  }
});

// Start processing
await worker.start();

// Graceful shutdown
process.on('SIGTERM', async () => {
  await worker.stop();
  await worker.drain(); // Wait for current jobs
});
```

## Express Integration

Complete Express application with all production features:

```typescript
import express from 'express';
import {
  createGitPoort,
  HealthCheck,
  createRedisHealthCheck,
  createMemoryHealthCheck,
  GracefulShutdown,
  createServerShutdown,
  WebhookReplayManager,
  InMemoryWebhookReplayStore,
  createObservability,
  CircuitBreakerRegistry,
} from 'gitpoort';

const app = express();
app.use(express.json({ verify: (req, res, buf) => { req.rawBody = buf.toString(); } }));

// Initialize GitPoort
const gitpoort = createGitPoort({
  providers: {
    github: {
      oauth: {
        clientId: process.env.GITHUB_CLIENT_ID!,
        clientSecret: process.env.GITHUB_CLIENT_SECRET!,
        redirectUri: `${process.env.APP_URL}/auth/github/callback`,
      },
    },
  },
  webhookSecrets: {
    github: process.env.GITHUB_WEBHOOK_SECRET,
  },
});

// Health checks
const health = new HealthCheck();
health.addCheck(createMemoryHealthCheck(512));
// health.addCheck(createRedisHealthCheck(redis)); // Add when using Redis

app.get('/health/live', async (req, res) => {
  const result = await health.liveness();
  res.status(result.alive ? 200 : 503).json(result);
});

app.get('/health/ready', async (req, res) => {
  const result = await health.readiness();
  res.status(result.status === 'healthy' ? 200 : 503).json(result);
});

// Webhook replay
const replayStore = new InMemoryWebhookReplayStore();
const replayManager = new WebhookReplayManager(replayStore);

// Observability
const obs = createObservability({ debug: true });
obs.onWebhookReceived(({ provider, eventType }) => {
  console.log(`Webhook received: ${provider}/${eventType}`);
});

// Circuit breakers
const circuits = new CircuitBreakerRegistry();

// Webhook endpoint
app.post('/git/webhooks', async (req, res) => {
  const provider = req.headers['x-github-event'] ? 'github' : 'unknown';

  const webhookId = await replayManager.capture(
    provider as any,
    req.headers as Record<string, string>,
    req.body,
    req.rawBody
  );

  try {
    const { delivery, routing } = await gitpoort.webhookRouter.processWebhook(
      req.headers as Record<string, string>,
      req.body,
      req.rawBody
    );

    obs.webhookReceived(provider as any, delivery.eventType, delivery.deliveryId);
    await replayManager.markCompleted(webhookId);

    res.status(202).json({
      received: true,
      deliveryId: delivery.deliveryId,
    });
  } catch (error) {
    await replayManager.markFailed(webhookId, (error as Error).message);
    res.status(500).json({ error: 'Processing failed' });
  }
});

// OAuth routes
app.get('/auth/:provider', (req, res) => {
  const adapter = gitpoort.getProvider(req.params.provider as any);
  if (!adapter) {
    return res.status(404).json({ error: 'Provider not found' });
  }
  const state = Buffer.from(JSON.stringify({ ts: Date.now() })).toString('base64');
  res.redirect(adapter.getAuthorizationUrl(state));
});

app.get('/auth/:provider/callback', async (req, res) => {
  const adapter = gitpoort.getProvider(req.params.provider as any);
  if (!adapter) {
    return res.status(404).json({ error: 'Provider not found' });
  }

  try {
    const tokenInfo = await adapter.exchangeCodeForToken(req.query.code as string);
    // Store token in your database
    res.json({ success: true, expiresAt: tokenInfo.expiresAt });
  } catch (error) {
    res.status(400).json({ error: (error as Error).message });
  }
});

// Start server with graceful shutdown
const server = app.listen(3000, () => {
  console.log('Server running on port 3000');
});

const shutdown = new GracefulShutdown({ timeout: 30000 });
shutdown.register('http-server', createServerShutdown(server));
shutdown.listen();
```

## Snapshot Resolution

Resolve repository state at a specific timestamp:

```typescript
const { gitOperations } = gitpoort;

// Clone/fetch the repository
const repoPath = await gitOperations.ensureClone(
  'https://x-access-token:TOKEN@github.com/owner/repo.git',
  'repo-id'
);

// Find commit at specific date
const commitSha = await gitOperations.resolveCommitAtDate(
  repoPath,
  'main',
  new Date('2024-06-30T23:59:59Z')
);

// Get commit info
const commitInfo = await gitOperations.getCommitInfo(repoPath, commitSha);

// Compute diff between two commits
const diff = await gitOperations.computeDiff(repoPath, 'commit1', 'commit2');
```

## Architecture

GitPoort follows hexagonal architecture:

```
src/
├── core/           # Domain types and port interfaces
│   ├── types.ts    # Core domain types
│   ├── ports.ts    # Port interfaces (contracts)
│   ├── errors.ts   # Custom error classes
│   └── observability.ts # Observability hooks
├── adapters/       # Provider implementations
│   ├── github/     # GitHub OAuth + App adapters
│   ├── gitlab/     # GitLab adapter
│   ├── bitbucket/  # Bitbucket adapter
│   └── azure-devops/ # Azure DevOps adapter
├── webhook/        # Webhook routing and replay
├── git/            # Git operations (clone, diff, etc.)
├── stores/         # Storage implementations
├── queue/          # Redis job queue
├── health/         # Health checks
├── lifecycle/      # Graceful shutdown
├── resilience/     # Circuit breaker, rate limit pool
└── config/         # Configuration validation
```

## Custom Store Implementations

Provide your own store implementations for production:

```typescript
const gitpoort = createGitPoort({
  providers: { /* ... */ },
  stores: {
    connection: new PostgresConnectionStore(db),
    repo: new PostgresRepoStore(db),
    job: new RedisJobQueue(redis),
    snapshot: new S3SnapshotStore(s3),
  },
});
```

## Configuration Validation

Validate configuration at startup:

```typescript
import {
  validateProviderConfig,
  validateRetryOptions,
  validateWebhookSecrets,
} from 'gitpoort';

// Throws ValidationError with details on invalid config
const githubConfig = validateProviderConfig('github', {
  clientId: process.env.GITHUB_CLIENT_ID,
  clientSecret: process.env.GITHUB_CLIENT_SECRET,
  redirectUri: process.env.GITHUB_REDIRECT_URI,
});

const retryConfig = validateRetryOptions({
  maxRetries: 3,
  baseDelay: 1000,
});
```

## API Reference

Generate full API documentation:

```bash
npm run docs
# Open docs/index.html
```

### ProviderPort

All provider adapters implement this interface:

```typescript
interface ProviderPort {
  provider: GitProvider;
  getAuthorizationUrl(state: string): string;
  exchangeCodeForToken(code: string): Promise<TokenInfo>;
  refreshToken?(refreshToken: string): Promise<TokenInfo>;
  listRepositories(token: AccessToken): Promise<Repository[]>;
  getRepository(token: AccessToken, repoId: string): Promise<Repository>;
  getAuthenticatedCloneUrl(token: AccessToken, repo: Repository): string;
  listCommits(token: AccessToken, repoId: string, branch: string, options?: {...}): Promise<CommitInfo[]>;
  getCommit(token: AccessToken, repoId: string, sha: string): Promise<CommitInfo>;
}
```

### GitOperationsPort

```typescript
interface GitOperationsPort {
  ensureClone(cloneUrl: string, repoId: string): Promise<string>;
  resolveCommitAtDate(repoPath: string, branch: string, timestamp: Date): Promise<string | null>;
  getCommitInfo(repoPath: string, sha: string): Promise<CommitInfo>;
  checkoutCommit(repoPath: string, sha: string, targetPath: string): Promise<void>;
  computeDiff(repoPath: string, fromSha: string, toSha: string): Promise<Diff>;
  listFiles(repoPath: string, sha: string): Promise<string[]>;
}
```

## Token Management

Use `TokenManager` for automatic token refresh before API calls:

```typescript
import { TokenManager } from 'gitpoort';

const tokenManager = new TokenManager(connectionStore, providers, {
  refreshBuffer: 60000, // Refresh 1 min before expiry (default)
  onTokenRefresh: (connectionId, newToken) => {
    console.log(`Token refreshed for ${connectionId}`);
  },
});

// Automatically refreshes expired tokens
const token = await tokenManager.getValidToken(tenantId, connectionId, 'github');
await adapter.listRepositories(token);
```

## Security

GitPoort includes several security measures:

- **Tenant isolation**: All store operations require tenantId validation
- **Timing-safe comparison**: Webhook signatures use constant-time comparison
- **Webhook secret warnings**: Configurable alerts when secrets are missing

```typescript
const webhookRouter = new WebhookRouter({
  secrets: { github: process.env.GITHUB_WEBHOOK_SECRET },
  connectionStore,
  requireSecrets: true, // Throw error if no secret configured
  onWarning: (msg) => logger.warn(msg), // Custom warning handler
});
```

## Retry & Rate Limiting

All adapters include built-in retry logic and rate limit handling:

```typescript
// Retry options (configurable per adapter)
const adapter = new GitHubOAuthAdapter(config, 'https://api.github.com', {
  maxRetries: 3,        // Max retry attempts (default: 3)
  baseDelay: 1000,      // Base delay in ms (default: 1000)
  maxDelay: 30000,      // Max delay cap (default: 30000)
  retryOnRateLimit: true, // Auto-retry on 429 (default: true)
});

// Retries automatically on:
// - 5xx server errors
// - Network errors (ECONNRESET, timeout, DNS failures)
// - Rate limits (429) with retry-after header

// Rate limit errors include detailed info
try {
  await adapter.listRepositories(token);
} catch (error) {
  if (error instanceof RateLimitError) {
    console.log(`Rate limited. Retry after ${error.retryAfter}s`);
    console.log(`Resets at: ${error.resetAt}`);
  }
}
```

## License

MIT
