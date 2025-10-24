#!/bin/bash
# Network Packet Loss Chaos Test
# Adds 30% packet loss to web container for 30 seconds

echo "🔧 Injecting 30% packet loss into pumba-web for 30 seconds..."
echo "💡 Try: curl http://localhost:3000/health (may fail intermittently)"

docker run -it --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  gaiaadm/pumba \
  netem --duration 30s loss --percent 30 pumba-web

echo "✅ Packet loss removed"
