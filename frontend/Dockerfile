FROM node:13.3.0-buster-slim as yarn-build-stage
RUN apt-get -y update && apt-get install -y git
RUN yarn global add @vue/cli
WORKDIR /frontend
COPY package*.json ./
COPY babel.config.js ./
COPY .browserslistrc ./
COPY tslint.json ./
COPY tsconfig.json ./
RUN yarn install
COPY public ./public
COPY src ./src
# COPY .env.production ./

RUN apt-get autoremove -y \
    && apt-get autoclean -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
RUN yarn build
FROM nginx:stable
COPY --from=yarn-build-stage /frontend/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80
ENTRYPOINT ["nginx", "-g", "daemon off;"]

#EXPOSE 8080
#USER node
#CMD ["yarn", "serve"]
