FROM node:23-alpine

WORKDIR /app

COPY package*.json ./

RUN yarn install --frozen-lockfile

COPY . . 

EXPOSE 4173

CMD ["yarn", "start"]
