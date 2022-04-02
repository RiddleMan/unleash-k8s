load('ext://helm_remote', 'helm_remote')
helm_remote(
    'unleash',
    repo_name='unleash',
    repo_url='https://docs.getunleash.io/helm-charts'
)

k8s_resource(workload='unleash', port_forwards=['4242:4242'])