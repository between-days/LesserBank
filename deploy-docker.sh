docker build -t lesser-bank-api . # --build-arg PORT=8080 .

docker run -p 8080:8080 --rm lesser-bank-api