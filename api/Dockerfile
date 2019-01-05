FROM node:11-slim

WORKDIR /app

ENV PORT=3000 TIMEOUT=1000

COPY package*json ./
RUN npm install -s --only=production

COPY index.js ./

CMD node index.js
