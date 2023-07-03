docker build -t lesser-bank --build-arg PORT=8081 .

docker run -p 8081:8081 --rm lesser-bank