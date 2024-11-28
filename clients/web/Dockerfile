FROM node:23.3.0-alpine3.20 AS base
WORKDIR /app
COPY package.json .
RUN npm install
ENV PATH=/app/node_modules/.bin:$PATH
COPY . .

FROM base AS dev
EXPOSE 3000
CMD ["npm", "run", "dev", "--", "--host"]

FROM base AS builder
RUN npm run build

FROM nginx:1.27.3-alpine3.20 AS prod
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /var/www/html/
EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]
