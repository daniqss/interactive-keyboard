FROM oven/bun:1.2.8-alpine as builder

WORKDIR /app
COPY package.json ./
COPY bun.lock ./
COPY tsconfig.json ./
COPY tsconfig.node.json ./
COPY vite.config.ts ./

COPY index.html ./
COPY public ./public
COPY src ./src

COPY src-tauri/assets/audio ./src-tauri/assets/audio
COPY src-tauri/assets/image ./src-tauri/assets/image

RUN bun install
RUN VITE_COMPLETE="true" bun run build

FROM nginx:stable-alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
