#!/bin/bash
# Network Delay Chaos Test
# Adds 1000ms latency to web container for 30 seconds

echo "🔧 Injecting 1000ms network delay into pumba-web for 30 seconds..."
echo "💡 Try: curl http://localhost:3000/data/test (will be slow)"

docker run -it --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  gaiaadm/pumba \
  netem --duration 30s delay --time 1000 pumba-web

echo "✅ Network delay removed"
