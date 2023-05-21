UI_IMAGE_NAME="chat_ui" \
IMAGE_NAME="chat_server" \
LOAD_BALANCER_IMAGE_NAME="load_balancer" \

cd ./ui
if [ "$(docker images -q $UI_IMAGE_NAME 2> /dev/null)" == "" ]; then
  echo "Building ui image..."
  docker build -t chat_ui .
fi

cd ../chat\ server
if [ "$(docker images -q $IMAGE_NAME 2> /dev/null)" == "" ]; then
  echo "Building server image..."
  docker build -t chat_server .
fi

cd ../load_balancer
if [ "$(docker images -q $LOAD_BALANCER_IMAGE_NAME 2> /dev/null)" == "" ]; then
  echo "Building load balancer image..."
  docker build -t load_balancer .
fi

echo "Starting server..."
docker compose up -d
echo "Server is running!"
