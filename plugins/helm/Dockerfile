FROM quay.io/ipedrazas/drone-helm as plugin
FROM dronelab/dronelab
COPY --from=plugin / /
ENV DRONELAB_UPSTREAM_IMAGE="quay.io/ipedrazas/drone-helm"
ENV DRONELAB_UPSTREAM_AUTHOR="ipedrazas"
ENV DRONELAB_UPSTREAM_REPO="https://github.com/ipedrazas/drone-helm"
ENV DRONELAB="/bin/drone-helm"
