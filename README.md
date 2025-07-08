# Rust Axum + Kubernetes + Skaffold

Simple Rust web server with Axum, containerized via Nixpacks and deployed to Kubernetes using Skaffold.

## Quick Start

### Check kubernetes connection

```bash
kubectl get pods -A
```

### Kubernetes Deployment
```bash
skaffold dev
```

## Access Application

```bash
# Via ingress (recommended)
curl http://my-app.local/
```