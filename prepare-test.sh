echo "starting the docker environment"
bash ./tests/resources/wait.sh
docker-compose -f ./tests/resources/docker-compose.yml up -d server
sleep 5
