#!/bin/bash
# Container Kill Chaos Test
# Kills the web container - Docker Compose will restart it

echo "🔧 Killing pumba-web container (will auto-restart)..."
echo "💡 Watch: docker-compose logs -f web"

docker run -it --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  gaiaadm/pumba \
  kill --signal SIGKILL pumba-web

echo "✅ Container killed - should be restarting..."
sleep 5
docker ps | grep pumba-web
