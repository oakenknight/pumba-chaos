# Pumba Chaos Testing Demo

A chaos engineering demonstration project using Rust, Axum web framework, Redis, Docker, and Pumba for fault injection testing.

## Technologies

- **Rust 1.89+** - Systems programming language
- **Axum** - Modern async web framework
- **Redis 7** - In-memory data store
- **Docker & Docker Compose** - Containerization
- **Pumba** - Chaos engineering tool for Docker

## Prerequisites

- Docker and Docker Compose installed
- curl for testing (or any HTTP client like Postman)
- Pumba runs via Docker (no separate installation needed)

## Quick Start

```bash
# Build and start services
docker-compose up --build -d

# Check logs
docker-compose logs -f web

# Verify health
curl http://localhost:3000/health
```

## API Endpoints

### Health Check
Check application and Redis connectivity status.

```bash
curl http://localhost:3000/health
```

**Expected Response:**

```json
{
  "status": "ok",
  "redis": "connected",
  "timestamp": "2025-10-23T12:00:00Z"
}
```

### Store Data
Store a key-value pair in Redis with 300 second TTL.

```bash
curl -X POST http://localhost:3000/store \
  -H "Content-Type: application/json" \
  -d '{"key":"test","value":"hello world"}'
```

**Expected Response:**

```json
{
  "success": true,
  "key": "test",
  "message": "Data stored successfully with 300 second TTL"
}
```

### Get Data

Retrieve a value from Redis by key.

```bash
curl http://localhost:3000/data/test
```

**Expected Response:**
```json
{
  "key": "test",
  "value": "hello world",
  "retrieved_at": "2025-10-23T12:00:00Z"
}
```

### Slow Operation
Test endpoint that takes 3 seconds to complete.

```bash
curl http://localhost:3000/slow
```

**Expected Response:**
```json
{
  "message": "Slow operation completed",
  "duration": 3000
}
```

### Statistics
Get application uptime, request count, and Redis status.

```bash
curl http://localhost:3000/stats
```

**Expected Response:**
```json
{
  "uptime": 120,
  "requests": 42,
  "redis_connected": true
}
```

## Chaos Testing

### Prerequisites
- Services must be running: `docker-compose up -d`
- All scripts are executable (already set during setup)

### Available Chaos Tests

#### 1. Network Delay
Adds 1000ms latency to all requests for 30 seconds.

```bash
./chaos-tests/network-delay.sh
```

**What to observe:**
- All API requests become noticeably slower
- Response times increase by ~1 second
- Application remains functional but degraded

**Monitor with:**
```bash
# Terminal 1: Watch logs
docker-compose logs -f web

# Terminal 2: Test during chaos
curl http://localhost:3000/data/test
```

#### 2. Network Packet Loss
Adds 30% packet loss for 30 seconds.

```bash
./chaos-tests/network-loss.sh
```

**What to observe:**
- Some requests may fail intermittently
- Timeouts may occur
- Redis connection issues may appear

**Monitor with:**
```bash
# Test repeatedly during chaos
for i in {1..10}; do curl http://localhost:3000/health; echo; sleep 1; done
```

#### 3. Container Kill
Forcefully kills the web container (Docker will restart it).

```bash
./chaos-tests/container-kill.sh
```

**What to observe:**
- Container dies immediately
- Docker Compose automatically restarts it
- Downtime of ~5-10 seconds
- Request count resets to 0

**Monitor with:**
```bash
# Watch restart in real-time
docker-compose logs -f web
```

#### 4. Container Pause
Pauses the web container for 15 seconds (freezes all processes).

```bash
./chaos-tests/container-pause.sh
```

**What to observe:**
- All requests hang for 15 seconds
- Container appears running but doesn't respond
- Requests resume normally after unpause

**Test during chaos:**
```bash
# This will hang for 15 seconds
time curl http://localhost:3000/health
```

#### 5. CPU Stress
Stresses CPU with 2 workers for 30 seconds.

```bash
./chaos-tests/stress-cpu.sh
```

**What to observe:**
- High CPU usage
- Slower response times
- `/slow` endpoint may timeout
- Application remains responsive but degraded

**Monitor with:**
```bash
# Watch CPU usage in real-time
docker stats pumba-web
```

## Observing Chaos Effects

For the best experience, open multiple terminals:

**Terminal 1: Application Logs**
```bash
docker-compose logs -f web
```

**Terminal 2: Run Chaos Script**
```bash
./chaos-tests/network-delay.sh
```

**Terminal 3: Test API During Chaos**
```bash
# Store data
curl -X POST http://localhost:3000/store \
  -H "Content-Type: application/json" \
  -d '{"key":"chaos","value":"testing"}'

# Retrieve data
curl http://localhost:3000/data/chaos

# Check stats
curl http://localhost:3000/stats
```

**Terminal 4 (Optional): Container Stats**
```bash
docker stats pumba-web pumba-redis
```

## Cleanup

Stop and remove all containers and volumes:

```bash
docker-compose down -v
```

## Development

### Build locally
```bash
cargo build --release
```

### Run tests
```bash
cargo test
```

### Check code quality
```bash
cargo clippy
```

### Build Docker image
```bash
docker-compose build
```

## Troubleshooting

### Container won't start
```bash
# Check logs
docker-compose logs web

# Rebuild from scratch
docker-compose down -v
docker-compose up --build
```

### Redis connection fails
```bash
# Check Redis health
docker-compose ps
docker-compose logs redis

# Restart Redis
docker-compose restart redis
```

### Port already in use
```bash
# Check what's using port 3000
lsof -i :3000

# Kill the process or change port in docker-compose.yml
```

## Learning Resources

- [Pumba Documentation](https://github.com/alexei-led/pumba)
- [Chaos Engineering Principles](https://principlesofchaos.org/)
- [Axum Framework](https://github.com/tokio-rs/axum)
- [Redis Commands](https://redis.io/commands)
