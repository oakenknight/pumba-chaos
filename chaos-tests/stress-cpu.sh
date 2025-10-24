#!/bin/bash
# CPU Stress Chaos Test
# Stresses CPU in web container for 30 seconds

echo "🔧 Stressing CPU in pumba-web for 30 seconds..."
echo "💡 Try: curl http://localhost:3000/slow (may timeout)"
echo "💡 Watch: docker stats pumba-web"

docker run -it --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  gaiaadm/pumba \
  stress --duration 30s --stress-image alexeiled/stress-ng:latest-ubuntu \
  pumba-web --cpu 2

echo "✅ CPU stress removed"
