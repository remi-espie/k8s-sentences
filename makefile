create-cluster:
	kind create cluster --name ssi --config ./kind/kind-config.yaml
	kubectl apply -f https://kind.sigs.k8s.io/examples/ingress/deploy-ingress-nginx.yaml

delete-cluster:
	kind delete cluster --name ssi

deploy:
	helm repo add kyverno https://kyverno.github.io/kyverno/
	helm repo add falcosecurity https://falcosecurity.github.io/charts
	helm repo update
	helm install kyverno kyverno/kyverno -n kyverno --create-namespace
	helm install  -n falco --create-namespace --set tty=true --set driver.kind=modern_ebpf --generate-name falcosecurity/falco
	kubectl get crd gateways.gateway.networking.k8s.io &> /dev/null || \
  	{ kubectl kustomize "github.com/kubernetes-sigs/gateway-api/config/crd?ref=v1.2.0" | kubectl apply -f -; }
	@if command -v istioctl &> /dev/null; then \
		istioctl install --set profile=minimal -y; \
	else \
		curl -L https://istio.io/downloadIstio | sh - && istio-*/bin/istioctl install --set profile=minimal -y; \
	fi
	echo "Waiting for Ingress to be ready..."
	kubectl wait --namespace ingress-nginx --for=condition=ready pod --selector=app.kubernetes.io/component=controller --timeout=10m
	kubectl apply -f k8s/00-namespace.yaml
	kubectl label namespace sentence istio-injection=enabled
	kubectl apply -f k8s/
	kubectl apply -f istio/
	kubectl apply -f kyverno/

destroy:
	kubectl delete -f istio/
	kubectl label namespace sentence istio-injection-
	kubectl delete -f kyverno/
	helm uninstall kyverno -n kyverno
	helm uninstall falco -n falco
	kubectl delete -f k8s/

run:
	$(MAKE) create-cluster
	$(MAKE) deploy

test:
	@if command -v istioctl &> /dev/null; then \
		istioctl install --set profile=demo -y; \
	else \
		curl -L https://istio.io/downloadIstio | sh - && istio-*/bin/istioctl install --set profile=demo -y; \
	fi