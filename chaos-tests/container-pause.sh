#!/bin/bash
# Container Pause Chaos Test
# Pauses the web container for 15 seconds

echo "🔧 Pausing pumba-web container for 15 seconds..."
echo "💡 Try: curl http://localhost:3000/health (will hang)"

docker run -it --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  gaiaadm/pumba \
  pause --duration 15s pumba-web

echo "✅ Container unpaused"
