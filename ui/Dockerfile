FROM node:alpine as build

ARG API_URL

WORKDIR /app
COPY . .
RUN npm install
RUN echo $API_URL
RUN REACT_APP_API_URL=${API_URL} npm run build

FROM nginx:stable-alpine
COPY --from=build /app/build /usr/share/nginx/html
COPY --from=build /app/nginx/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]