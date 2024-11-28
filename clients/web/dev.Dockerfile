FROM node:23.3.0-alpine3.20
WORKDIR /app
COPY package.json .
ENV PATH=/app/node_modules/.bin:$PATH
RUN npm install
COPY . .
EXPOSE 3000
CMD ["npm", "run", "dev", "--", "--host"]
