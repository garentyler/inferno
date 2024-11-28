FROM node:23.3.0-alpine3.20 AS builder
WORKDIR /app
COPY package.json .
RUN npm install
ENV PATH=/app/node_modules/.bin:$PATH
COPY . .
RUN npm run build

FROM nginx:1.27.3-alpine3.20
COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist /var/www/html/
EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]