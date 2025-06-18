Pods

- A pod is the smallest deployable unit in kubernetes.
- It usually just runs one container.
- They can die and be replaces automatically.
- Each pod has its own api address.
- AKA: A running instances of part of your app.

Service

- An abstraction that provides a stable network endpoint for access a group of pods.
- Services automatically load balance requests across matching pods.

Recap

- Two instances of shop-app running
- Each containing an image for the api
- Kind is running the kubernetes cluser in a docker container
- Proxy service running and exposing port 8080:8080
