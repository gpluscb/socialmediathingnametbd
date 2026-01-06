# syntax=docker/dockerfile:1

FROM node:25 as node-build
WORKDIR /app

COPY stellwerk-frontend/package.json stellwerk-frontend/package-lock.json ./
RUN npm ci

COPY stellwerk-frontend/ .
RUN npm run build
RUN npm prune --production

FROM node:25 as final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=node-build /app/build/ build/
COPY --from=node-build /app/node_modules/ node_modules/

COPY stellwerk-frontend/package.json .

ENV NODE_ENV=production

CMD ["node", "build"]
