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
	@if ! command -v istioctl &> /dev/null; then \
		$(MAKE) install-istio; \
	fi
	echo "Waiting for Ingress to be ready..."
	kubectl wait --namespace ingress-nginx --for=condition=ready pod --selector=app.kubernetes.io/component=controller --timeout=10m
	kubectl apply -f kyverno/
	kubectl apply -f k8s/
	kubectl label namespace sentence istio-injection=enabled
	kubectl apply -f ./istio

destroy:
	kubectl delete -f kyverno/
	helm uninstall kyverno -n kyverno
	helm uninstall falco -n falco
	kubectl delete -f k8s/

run:
	$(MAKE) create-cluster
	$(MAKE) deploy

install-istio:
	curl -L https://istio.io/downloadIstio | sh -
	istio-*/bin/istioctl install --set profile=demo -y