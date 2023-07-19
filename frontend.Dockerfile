FROM node:latest as builder

COPY package-lock.json app/package-lock.json
COPY package.json app/package.json
COPY frontend/tsconfig.json app/frontend/tsconfig.json
COPY frontend/next.config.js app/frontend/next.config.js
COPY frontend/public app/frontend/public
COPY frontend/src app/frontend/src

WORKDIR /app

RUN npm install
RUN npm run build

FROM node:latest

COPY --from=builder /app/frontend/.next /app/frontend/.next
COPY --from=builder /app/package.json /app/package.json
COPY --from=builder /app/node_modules /app/node_modules

WORKDIR /app

CMD npm run start