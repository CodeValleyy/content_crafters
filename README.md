# Content Crafters
    
[![Build Status](https://travis-ci.com/jabibamman/content_crafters.svg?branch=main)](https://travis-ci.com/jabibamman/content_crafters)
[![codecov](https://codecov.io/gh/jabibamman/content_crafters/branch/main/graph/badge.svg?token=QZQZQZQZQZ)](https://codecov.io/gh/jabibamman/content_crafters)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description

Content Crafters is a microservice that manages metadata for programs and user data. It also handles information about content imports, edits, and shares. The service stores data related to likes, comments, and social interactions related to content. Additionally, it manages versioning information and collaborative changes to programs.

## Installation

### Clone the repository

```bash
git clone https://github.com/jabibamman/content_crafters.git
```

### Change directory

```bash
cd content_crafters
```

### Install dependencies

```bash
cargo build
```

### Run the application

```bash
cargo run
```

The application will start at `http://localhost:8080`.

If you want to run the application with different port, you can set the `APP_PORT` environment variable.

```bash
APP_PORT=3000 cargo run
```

## Kubernetes

The application provides a Kubernetes deployment file in the `k8s` directory. You can deploy the application using the following command:

```bash
./deploy_k8s.sh
```

The script will create a namespace, deployment, and service for the application.

### For the environment variables

We use a secret service to store the environment variables. You can create the secret using the following command:

```bash
kubectl apply -f secret.yaml
```

If you want to add more environment variables, you can add them to the `secret.yaml` file.

**WARNING**: The environment variables are base64 encoded. You can encode your environment variables using the following command:

```bash
echo -n "your-environment-variable" | base64
```

## CLI

The application also provides a CLI to interact with the service. You can run the CLI using the following command:

```bash
cargo run -- --help
```

## Dockerize the application

### Build the image

```bash
docker build -t content_crafters .
```

### Run the container

```bash
docker run -d -p 3000:3000 -e APP_PORT=3000 content_crafters
```

If you want to run the container with logs, you can add the 
`-e VERBOSE=1`
OR
`-e DEBUG=1`
OR
`-e TRACE=1`
flag to the `docker run` command.

## Docker Compose

You can also use Docker Compose to run the application. The `docker-compose.yml` file is already provided in the repository. You can run the application using the following command:

```bash
docker-compose up --build
```