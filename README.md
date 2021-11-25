# A quick look at krustlet

This repo contains fundamental WebAssembly (Wasm) workloads that could be executed on [krustlet](https://krustlet.dev).

## Setup

- obviously, you must have a krustlet running. You can add krustlet to different Kubernetes distributions and managed Kubernetes services.
- an OCI distribution spec compliant Container Registry (e.g. Azure Container Registry) is required. We will use it as distribution channel for our Wasm workloads
- `wasm32-wasi` must be installed as target (`rustup target add wasm32-wasi`)
- Workloads must be compiled against the `wasm32-wasi` using `cargo build --release --target wasm32-wasi`

## Container Registry Authentication

Authentication information for the container registry must be persisted in the desired Kubernetes Namespace. You can use the following snippet to persist such an authentication information:

```bash
regLoginServer=foo.azurecr.io
regUser=bob
regPassword=bob
k8sNamespace=default

kubectl create secret docker-registry acr \
  --docker-username $regUser \
  --docker-server $regLoginServer \
  --docker-password $regPassword
  --namespace $k8sNamespace
```

## Verify krustlet taints

Depending on your environment you may find different `taints` being assigned to the krustlet nodes. Verify if your taints are `wasm32-wasi` or `wasm32-wagi`:

```bash

# get all nodes in your cluster

kubectl get nodes

NAME                 STATUS     ROLES                  AGE    VERSION
kind-control-plane   Ready      control-plane,master   103m   v1.21.1
foobar               Ready      <none>                 39m    1.0.0-alpha.1


# get node taints
kubectl describe node foobar

# omitted
Taints: kubernetes.io/arch=wasm32-wasi:NoExecute
        kubernetes.io/arch=wasm32-wasi:NoSchedule
# omitted
```

Note down the `arch`, you must specify it as part of the Pods `tolerations` (see `pod.yml` in both samples -> `podSpec.tolerations`)

## Publishing Wasm modules to Azure Container Registry (ACR)

Assuming having access to an ACR instance called `foobar`, we must push both Wasm modules (`hello-krustlet` and `hello-wasi`) to the ACR instance. To do so, we use [wasm-to-oci](https://github.com/engineerd/wasm-to-oci)

```bash
# authenticate against ACR (either use Azure CLI or use Docker CLI)
az acr login -n foobar

cd hello-krustlet
cargo build --release --target wasm32-wasi

wasm-to-oci ./target/wasm32-wasi/release/hello-krustlet.wasm foobar.azurecr.io/hello-krustlet:0.0.1
cd ..

cd hello-wasi
cargo build --release --target wasm32-wasi

wasm-to-oci ./target/wasm32-wasi/release/hello-wasi.wasm foobar.azurecr.io/hello-wasi:0.0.1

```

## Running hello-krustlet

You can run `hello-krustlet` by applying the Kubernetes manifest located in the kubernetes subfolder:

```bash
kubectl apply -f ./hello-krustlet/kubernetes/pod.yml
```

## Running hello-krustlet

You can run `hello-krustlet` locally using any (non-browser) WASM runtime. The following sample uses `wasmtime`:

```bash
cd hello-krustlet
cargo build --release --target wasm32-wasi

wasmtime run ./target/wasm32-wasi/release/hello-krustlet.wasm
```

## Running hello-wasi in KIND

Before running `hello-wasi`, you must configure the persistent volume used in the pod spec `podSpec.volumes[0]`.

The WASI sample will issue a system call (create a file). You must allow the module to write to the desired location (`/mnt/data` -> see `podSpec.containers[0].env` and `podSpec.containers[0].volumeMounts[0]`)
You can run `hello-krustlet` by applying the Kubernetes manifest located in the kubernetes subfolder:

```bash
kubectl apply -f ./hello-krustlet/kubernetes/pod.yml
```

## Running hello-wasi locally

You can run `hello-wasi` locally using any (non-browser) WASM runtime. The following sample uses `wasmtime`:

```bash
cd hello-wasi
cargo build --release --target wasm32-wasi

wasmtime run --env TARGET=/Users/bob/Downloads --dir /Users/bob/Downloads ./target/wasm32-wasi/release/hello-wasi.wasm
```
