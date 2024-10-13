FROM node:lts-alpine3.20 as build

ARG API_URL
ARG MAPBOX_ACCESS_TOKEN

WORKDIR /app
COPY . .
RUN npm install
RUN REACT_APP_API_URL=${API_URL} REACT_APP_MAPBOX_ACCESS_TOKEN=${MAPBOX_ACCESS_TOKEN} npm run build

FROM nginx:stable-alpine
COPY --from=build /app/build /usr/share/nginx/html
COPY --from=build /app/nginx/nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
