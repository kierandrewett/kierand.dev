FROM node:23-alpine

WORKDIR /app

COPY package*.json ./

RUN yarn install --frozen-lockfile

COPY . .

RUN yarn build

EXPOSE 4173

CMD ["yarn", "preview", "--host"]
