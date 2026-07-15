# Development

### Serving Site
Site is set to be run from ./site/ using `dx serve`

When running in a container, the image needs to be bundled
```bash
dx bundle --release --web -p site
```

# Deployment
### Local
Build image from root
```bash
docker build -t hbme .
```
Log in to registry
```bash
docker login hfserv.lan:8443
```
Retag image to match registry name
```bash
docker tag hbme:latest hfserv.lan:8443/hbme:latest
```
Push retagged image
```bash
docker push hfserv.lan:8443:/hbme:latest
```

### Remote
Remote in to server
```bash
ssh hfserv.lan
```
Pull image from registry
```bash
docker pull localhost:5000/hbme:latest
```
Update image tag in dockerfile if needed, then navigate to repository.
In repository root, run docker compose
```bash
docker compose down
docker compose up
```
*NOTE* Container image is exposed on port 8080
