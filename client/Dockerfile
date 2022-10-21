################
##### Builder
FROM node:buster as builder

# set working directory
WORKDIR /usr/src/app

# build react app
COPY package.json package-lock.json ./
COPY src ./src
COPY public ./public
RUN npm install --silent
RUN npm run build

# create express app
COPY express/index.js express/package.json /usr/src/app/express/
WORKDIR /usr/src/app/express
RUN npm install --production=true --silent


################
##### Runtime
FROM alpine:3.16.2 as runner

# create node user (copied from node:alpine)
RUN addgroup -g 1000 node && \
    adduser -u 1000 -G node -s /bin/sh -D node

# add nodejs and dumb-init
RUN apk add --no-cache nodejs-current
RUN apk add --no-cache dumb-init

# set to production environment
ENV NODE_ENV production

# get express app from builder
COPY --from=builder /usr/src/app/build /usr/src/app/build
COPY --from=builder /usr/src/app/express /usr/src/app/express

# set the working dir to express app
WORKDIR /usr/src/app/express

# run it
EXPOSE 3000
USER node
CMD ["dumb-init", "node", "index.js"]